// This is the the main application module. In module namespacing it is the
// `crate` module. It is generally responsible for housing the main() entry
// point. In our case we have the main entry point responsible for the
// following:
//
// - declaring the CLI options interface & help messaging
// - parsing the CLI options into a data structure (ApplicationArguments)
// - map CLI options data structure to subcommand calls & arguments
//
// So any code that fits the above responsibilities should live within this
// module.

use std::{
    fs,
    path::{Path, PathBuf},
};
use structopt::StructOpt;
use tera::{Context, Tera};
use toml::Value;

type Config = std::collections::HashMap<String, Group>;
type Group = std::collections::HashMap<String, toml::Value>;

#[derive(Debug)]
enum TmpTomlErr {
    File(ReadFileErr),
    GroupNotFound(String),
    Config(toml::de::Error),
    Render(TeraRenderErr),
}
impl From<toml::de::Error> for TmpTomlErr {
    fn from(err: toml::de::Error) -> Self {
        TmpTomlErr::Config(err)
    }
}

impl From<ReadFileErr> for TmpTomlErr {
    fn from(err: ReadFileErr) -> Self {
        TmpTomlErr::File(err)
    }
}

impl From<TeraRenderErr> for TmpTomlErr {
    fn from(render_err: TeraRenderErr) -> Self {
        TmpTomlErr::Render(render_err)
    }
}

#[derive(Debug)]
enum ReadFileErr {
    FileNotFound(String),
}

#[derive(Debug)]
enum TeraRenderErr {
    TemplateNotFound(String),
    InvalidTemplate(String),
    RenderError(tera::Error),
}

impl From<ReadFileErr> for TeraRenderErr {
    fn from(err: ReadFileErr) -> Self {
        match err {
            ReadFileErr::FileNotFound(path) => TeraRenderErr::TemplateNotFound(path),
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "tmptoml", about = "Parse toml files for templated solutions")]
pub struct ApplicationArguments {
    #[structopt(name = "config", about = "Path to the config file")]
    pub config: String,
    #[structopt(
        name = "template",
        about = "Path to the template file",
        parse(from_os_str)
    )]
    pub template: PathBuf,
    #[structopt(name = "group_id", about = "ID of the toml group to use")]
    pub group_id: String,
    #[structopt(
        name = "secondary_group_id",
        about = "ID of the toml secondary group to use"
    )]
    pub secondary_group_id: String,
}

fn read_file(path: &str) -> Result<String, ReadFileErr> {
    return fs::read_to_string(path).map_err(|_| ReadFileErr::FileNotFound(path.to_string()));
}

fn build_tera_context(template_values: std::collections::HashMap<String, String>) -> Context {
    let mut context = Context::new();
    for (key, value) in template_values {
        context.insert(key, &value);
    }
    return context;
}

fn render_tera_template(
    template_file_path: &Path,
    context: Context,
) -> Result<String, TeraRenderErr> {
    let mut tera = Tera::default();
    let template_name = "template";

    tera.add_template_file(template_file_path, Some(template_name))
        .map_err(|err| {
            TeraRenderErr::InvalidTemplate(format!(
                "Failed to parse template file with error: {}",
                &err
            ))
        })?;
    return tera
        .render(template_name, &context)
        .map_err(|err| TeraRenderErr::RenderError(err));
}

fn flatten_sections(
    group_section: &std::collections::HashMap<String, Value>,
) -> std::collections::HashMap<String, String> {
    let mut flattened: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    group_section.iter().for_each(|(key, value)| {
        if let toml::Value::Table(table) = value {
            table.iter().for_each(|(key, value)| {
                if !flattened.contains_key(key) {
                    flattened.insert(key.to_string(), value.to_string());
                }
            });
        } else {
            if !flattened.contains_key(key) {
                flattened.insert(key.to_string(), value.to_string());
            }
        }
    });

    return flattened;
}

fn run() -> Result<String, TmpTomlErr> {
    let opt: ApplicationArguments = ApplicationArguments::from_args();
    let config_file_path: String = opt.config.clone();
    let template_file_path: PathBuf = opt.template.clone();
    let group_id: String = opt.group_id.clone();
    let sec_group_id: String = opt.secondary_group_id.clone();
    let debug_print = false;

    let config_file: String = read_file(&config_file_path)?;
    let config: Config =
        toml::from_str(config_file.as_str()).map_err(|toml_err| TmpTomlErr::Config(toml_err))?;

    if debug_print {
        println!("Config File:\n{:?}\n", config);
    }

    if !config.contains_key(&group_id) {
        return Err(TmpTomlErr::GroupNotFound(group_id));
    }

    if !config[&group_id].contains_key(&sec_group_id) {
        return Err(TmpTomlErr::GroupNotFound(sec_group_id));
    }

    let group_section = &config[&group_id];
    let sec_group_section = &config[&group_id][&sec_group_id];

    if debug_print {
        println!("Config File:\n{:?}\n", config);
        println!("Group\n{:?}\n", &group_id);
        println!("Group Section\n{:?}\n", &group_section);
        println!("Secondary Group Group\n{:?}\n", &sec_group_id);
        println!("{:?} Section\n{:?}\n", sec_group_id, sec_group_section);
        let sub_group_table = sec_group_section.as_table();
        match sub_group_table {
            Some(table) => {
                table.iter().for_each(|(key, value)| {
                    println!("{:?} {:?}", key, value);
                });
            }
            None => println!("{:?}", sec_group_section),
        }
    }

    let template_values = flatten_sections(group_section);
    let tera_context = build_tera_context(template_values);
    let rendered_template = render_tera_template(template_file_path.as_path(), tera_context)?;
    return Ok(rendered_template);
}
fn main() {
    match run() {
        Ok(output) => println!("{}", output),
        Err(err) => match err {
            TmpTomlErr::File(file_error) => println!(
                "ERROR: There was an issue reading the config or template file. Reason: {:?}",
                file_error
            ),
            TmpTomlErr::GroupNotFound(key_id) => println!(
                "ERROR: Specified group_id or secondary_group_id ({:?}) could not be found in the config file.",
                key_id
            ),
            TmpTomlErr::Config(config_error) => println!(
                "ERROR: The specified config file could not be parsed. Reason: {:?}",
                config_error
            ),
            TmpTomlErr::Render(render_error) => println!(
                "ERROR: Unable to render the specified template. Reason: {:?}",
                render_error
            ),
        },
    };
}
