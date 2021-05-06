use anyhow::{anyhow, Result};
use serde_yaml::Value;

#[derive(Debug, PartialEq, Clone)]
pub enum ParamValue {
    Str(String),
    Int(i64),
    Float(f64, String), // Keep the original string representation to make sure we dont change anything
    Bool(bool),
}

impl std::fmt::Display for ParamValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParamValue::Str(x) => write!(f, "{}", x),
            ParamValue::Int(x) => write!(f, "{}", x),
            ParamValue::Float(_, s) => write!(f, "{}", s),
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
                    Ok(ParamValue::Float(
                        x.as_f64().unwrap(),
                        x.as_f64().unwrap().to_string().to_owned(),
                    ))
                } else {
                    Err(anyhow!("Found invalid number type: {}", &x))
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
            .or(s.parse::<f64>().map(|x| ParamValue::Float(x, s.to_owned())))
            .or(s.parse::<bool>().map(|x| ParamValue::Bool(x)))
            .or(Ok(ParamValue::Str(s.to_string())))
    }

    pub fn unwrap_f64(&self) -> f64 {
        match self {
            ParamValue::Float(f, _) => *f,
            ParamValue::Int(i) => *i as f64,
            _ => panic!("Tried to read value {} as float.", self),
        }
    }

    pub fn unwrap_i64(&self) -> i64 {
        match self {
            ParamValue::Int(i) => *i,
            _ => panic!("Tried to read value {} as int.", self),
        }
    }

    pub fn unwrap_string(&self) -> &str {
        match self {
            ParamValue::Str(s) => s,
            _ => panic!("Tried to read value {} as string.", self),
        }
    }
}
