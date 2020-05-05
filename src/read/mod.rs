mod json;
mod yaml;

use crate::target::Targets;
use serde_json::{from_str as from_str_json, Value as JSONValue};
use serde_yaml::{from_str as from_str_yaml, Value as YAMLValue};
use std::io::Read;

#[derive(Debug)]
pub enum ConksatValue {
    JSON(JSONValue),
    YAML(YAMLValue),
}

pub enum ConksatError {
    NotFound,
    IsObject,
    IsArray,
}

pub trait ConvertFrom: std::fmt::Debug {
    fn get_key(&self, key: &str) -> Result<String, ConksatError>;
}

impl ConvertFrom for ConksatValue {
    fn get_key(&self, key: &str) -> Result<String, ConksatError> {
        match self {
            ConksatValue::JSON(v) => v.get_key(key),
            ConksatValue::YAML(v) => v.get_key(key),
        }
    }
}

pub fn read_type<R: Read>(target: Targets, source: &mut R) -> Result<ConksatValue, String> {
    let mut raw_data = String::new();
    source
        .read_to_string(&mut raw_data)
        .map_err(|_| "failed to read source file")?;

    match target {
        Targets::JSON => from_str_json(&raw_data)
            .map(|v| ConksatValue::JSON(v))
            .map_err(|_| "failed to convert to json from source file".to_owned()),
        Targets::YAML => from_str_yaml(&raw_data)
            .map(|v| ConksatValue::YAML(v))
            .map_err(|_| "failed to convert to yaml from source file".to_owned()),
    }
}
