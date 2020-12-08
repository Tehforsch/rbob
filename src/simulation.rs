use std::collections::HashMap;

#[derive(Debug)]
pub struct Simulation {
    pub params: SimParams,
}

impl Simulation {
    pub fn from_file<U: AsRef<Path>>(path: U) -> Result<SimSetConfig> {
        let data = fs::read_to_string(path).context("While reading bob config file")?;
        Ok(serde_yaml::from_str(&data).context("Reading bob config file contents")?)
    }
}

#[derive(Debug)]
pub enum ParamValue {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

type SimParams = HashMap<String, ParamValue>;
