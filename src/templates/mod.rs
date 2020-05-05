use crate::read::ConvertFrom;
use crate::setting::{Config, Var};
use regex::{Captures, Regex};

pub struct ConksatReplacer {
    vars: Vec<Var>,
    template: String,
}

impl ConksatReplacer {
    pub fn replace<V: ConvertFrom>(self, data: &V) -> Result<String, String> {
        let re =
            Regex::new(r"\{\{\s*([\w_\-]+)\s*}}").map_err(|_| "regex compile failed".to_owned())?;
        Ok(re
            .replace_all(&self.template, |caps: &Captures| {
                caps.get(1)
                    .and_then(|name| self.vars.iter().find(|v| v.name() == name.as_str()))
                    .and_then(|v| data.get_key(v.key()).ok())
                    .unwrap_or("".to_owned())
            })
            .to_string())
    }
}

impl From<Config> for ConksatReplacer {
    fn from(src: Config) -> Self {
        Self {
            vars: src.vars().clone(),
            template: src.template().clone(),
        }
    }
}
