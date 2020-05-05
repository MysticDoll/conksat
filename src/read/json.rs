use crate::read::{ConksatError, ConvertFrom};
use serde_json::{to_string, Value};
impl ConvertFrom for Value {
    fn get_key(&self, key: &str) -> Result<String, ConksatError> {
        match key
            .split('.')
            .skip(1)
            .fold(Some(self), |v, k| v.and_then(|v| v.get(k)))
        {
            None => Err(ConksatError::NotFound),
            Some(Value::Object(_)) => Err(ConksatError::IsObject),
            Some(Value::Array(_)) => Err(ConksatError::IsArray),
            Some(Value::String(v)) => Ok(v.to_owned()),
            Some(v) => to_string(v).map_err(|_| ConksatError::NotFound),
        }
    }
}
