
use std::{
    fs,
    path::{Path, PathBuf},
};
use tera::{Context, Tera};
use toml::Value;

type Config = std::collections::HashMap<String, Group>;
type Group = std::collections::HashMap<String, toml::Value>;

#[derive(Debug)]
pub enum TmpTomlErr {
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
pub enum ReadFileErr {
    FileNotFound(String),
}

#[derive(Debug)]
pub enum TeraRenderErr {
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

fn read_file(path: Option<&str>) -> Result<String, ReadFileErr> {
    match path {
        Some(path) => {
            return fs::read_to_string(path)
                .map_err(|_| ReadFileErr::FileNotFound(path.to_string()));
        }
        None => return Err(ReadFileErr::FileNotFound("".to_string())),
    }
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

pub fn render_template(
    config_file_path: &PathBuf,
    template_file_path: &PathBuf,
    group_id: String,
    sec_group_id: String,
) -> Result<String, TmpTomlErr> {
    let debug_print = false;

    let config_file: String = read_file(config_file_path.to_str())?;
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
