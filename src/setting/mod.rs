use crate::target::Targets;
use getset::Getters;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, Getters)]
pub struct Var {
    #[getset(get = "pub")]
    name: String,
    #[getset(get = "pub")]
    key: String,
}

#[derive(Debug, Deserialize, Getters)]
pub struct Config {
    #[getset(get = "pub")]
    target: Targets,
    #[getset(get = "pub")]
    vars: Vec<Var>,
    #[getset(get = "pub")]
    template: String,
}

impl Config {
    pub fn new(path: &str) -> Result<Config, String> {
        let config_raw =
            std::fs::read_to_string(path).map_err(|_| "failed to read config file".to_owned())?;
        let c: Config =
            serde_yaml::from_str(&config_raw).map_err(|_| "failed to parse config file")?;
        return Ok(c);
    }
}
