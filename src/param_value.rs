use anyhow::{anyhow, Result};
use serde_yaml::Value;

#[derive(Debug, PartialEq, Clone)]
pub enum ParamValue {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

impl std::fmt::Display for ParamValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParamValue::Str(x) => write!(f, "{}", x),
            ParamValue::Int(x) => write!(f, "{}", x),
            ParamValue::Float(x) => write!(f, "{}", x),
            ParamValue::Bool(x) => write!(f, "{}", x),
        }
    }
}

impl ParamValue {
    pub fn new(v: &Value) -> Result<ParamValue> {
        match v {
            Value::Null => {
                panic!("Null value in serde value");
            }
            Value::Bool(x) => Ok(ParamValue::Bool(x.clone())),
            Value::Number(x) => {
                if x.is_i64() {
                    Ok(ParamValue::Int(x.as_i64().unwrap()))
                } else if x.is_f64() {
                    Ok(ParamValue::Float(x.as_f64().unwrap()))
                } else {
                    Err(anyhow!(format!("Found invalid number type: {}", &x)))
                }
            }
            Value::String(x) => Ok(ParamValue::Str(x.as_str().to_owned())),
            Value::Sequence(_) => Err(anyhow!("List in serde value - invalid bob file structure?")),
            Value::Mapping(_) => panic!("Mapping in serde value!"),
        }
    }

    pub fn from_str(s: &str) -> Result<ParamValue> {
        s.parse::<i64>()
            .map(|x| ParamValue::Int(x))
            .or(s.parse::<f64>().map(|x| ParamValue::Float(x)))
            .or(Ok(ParamValue::Str(s.to_string())))
    }
}
