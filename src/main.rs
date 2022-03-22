// // This is the the main application module. In module namespacing it is the
// // `crate` module. It is generally responsible for housing the main() entry
// // point. In our case we have the main entry point responsible for the
// // following:
// //
// // - declaring the CLI options interface & help messaging
// // - parsing the CLI options into a data structure (ApplicationArguments)
// // - map CLI options data structure to subcommand calls & arguments
// //
// // So any code that fits the above responsibilities should live within this
// // module.

use std::path::PathBuf;
use structopt::StructOpt;
use tmptoml;

#[derive(StructOpt, Debug)]
#[structopt(name = "tmptoml", about = "Parse toml files for templated solutions")]
pub struct ApplicationArguments {
    #[structopt(name = "config", about = "Path to the config file", parse(from_os_str))]
    pub config: PathBuf,
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

fn run() -> Result<String, tmptoml::TmpTomlErr> {
    let opt: ApplicationArguments = ApplicationArguments::from_args();
    let config_file_path: PathBuf = opt.config.clone();
    let template_file_path: PathBuf = opt.template.clone();
    let group_id: String = opt.group_id.clone();
    let sec_group_id: String = opt.secondary_group_id.clone();

    return tmptoml::render_template(
        &config_file_path,
        &template_file_path,
        group_id,
        sec_group_id,
    );
}
fn main() {
    match run() {
        Ok(output) => println!("{}", output),
        Err(err) => match err {
            tmptoml::TmpTomlErr::File(file_error) => println!(
                "ERROR: There was an issue reading the config or template file. Reason: {:?}",
                file_error
            ),
            tmptoml::TmpTomlErr::GroupNotFound(key_id) => println!(
                "ERROR: Specified group_id or secondary_group_id ({:?}) could not be found in the config file.",
                key_id
            ),
            tmptoml::TmpTomlErr::Config(config_error) => println!(
                "ERROR: The specified config file could not be parsed. Reason: {:?}",
                config_error
            ),
            tmptoml::TmpTomlErr::Render(render_error) => println!(
                "ERROR: Unable to render the specified template. Reason: {:?}",
                render_error
            ),
        },
    };
}
