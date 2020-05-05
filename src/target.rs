use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub enum Targets {
    JSON,
    YAML,
}
