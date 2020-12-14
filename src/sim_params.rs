use anyhow::{anyhow, Context, Result};
use std::{collections::HashMap, ops::Index};
use std::{fs, path::Path};

use crate::config::DEFAULT_PARAM_FILE_NAME;
use crate::param_value::ParamValue;

#[derive(Debug, Clone)]
pub struct SimParams {
    params: HashMap<String, ParamValue>,
    // time_limit_cpu: Time,
}

impl SimParams {
    pub fn from_folder<U: AsRef<Path>>(folder: U) -> Result<SimParams> {
        let mut params = HashMap::new();
        let param_file_path = folder.as_ref().join(DEFAULT_PARAM_FILE_NAME);
        let config_file_path = folder.as_ref().join(DEFAULT_PARAM_FILE_NAME);
        let job_file_path = folder.as_ref().join(DEFAULT_PARAM_FILE_NAME);
        update_from(&mut params, read_param_file(&param_file_path)?)?;
        Ok(SimParams { params })
    }

    pub fn insert(&mut self, key: &str, value: &ParamValue) -> Option<ParamValue> {
        self.params.insert(key.to_string(), value.clone())
    }
}

fn update_from(
    params: &mut HashMap<String, ParamValue>,
    new_params: HashMap<String, ParamValue>,
) -> Result<()> {
    for (key, value) in new_params.into_iter() {
        if params.contains_key(&key) {
            return Err(anyhow!(format!(
                "Key {} is present in multiple files.",
                key
            )));
        }
        params.insert(key, value);
    }
    Ok(())
}

fn read_param_file(path: &Path) -> Result<HashMap<String, ParamValue>> {
    let contents =
        read_contents(path).context(format!("While reading parameter file {:?}", path))?;
    let key_value_strings = read_parameter_lines(contents, "=", "%")?;
    key_value_strings
        .into_iter()
        .map(|(k, v)| ParamValue::from_str(&v).map(|x| (k, x)))
        .collect()
}

pub fn read_contents(path: &Path) -> Result<String> {
    fs::read_to_string(path).context("While reading file")
}

fn read_parameter_lines(
    contents: String,
    separator_string: &str,
    comment_string: &str,
) -> Result<HashMap<String, String>> {
    contents
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.starts_with(comment_string))
        .map(|line| {
            let split: Vec<&str> = line.split(separator_string).collect();
            if split.len() == 2 {
                Ok((split[0].to_string(), split[1].to_string()))
            } else {
                Err(anyhow!("Invalid line in parameter file"))
            }
        })
        .collect()
}

impl Index<&str> for SimParams {
    type Output = ParamValue;

    fn index(&self, key: &str) -> &ParamValue {
        &self.params[key]
    }
}
