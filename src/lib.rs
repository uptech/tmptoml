mod tmptoml;
pub use tmptoml::{
    parse_toml_to_config, render_template, Config, Group, ReadFileErr, TeraRenderErr, TmpTomlErr,
};
