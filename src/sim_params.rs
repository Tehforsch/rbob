use anyhow::{anyhow, Context, Result};
use std::{
    collections::hash_map::Iter, collections::hash_map::Keys, collections::HashMap, ops::Index,
};
use std::{fs, path::Path};

use crate::config::{
    AVAILABLE_CONFIG_PARAMS, DEFAULT_CONFIG_FILE_NAME, DEFAULT_JOB_FILE_NAME,
    DEFAULT_PARAM_FILE_NAME,
};
use crate::param_value::ParamValue;

use uom::si::f64::*;
// use uom::si::length::kilometer;
use regex::Regex;
use uom::si::time::second;

#[derive(Debug, Clone)]
pub struct SimParams {
    params: HashMap<String, ParamValue>,
    pub time_limit_cpu: Time,
}

impl SimParams {
    pub fn from_folder<U: AsRef<Path>>(folder: U) -> Result<SimParams> {
        let mut params = HashMap::new();
        let param_file_path = folder.as_ref().join(DEFAULT_PARAM_FILE_NAME);
        let config_file_path = folder.as_ref().join(DEFAULT_CONFIG_FILE_NAME);
        let job_file_path = folder.as_ref().join(DEFAULT_JOB_FILE_NAME);
        update_from(
            &mut params,
            read_param_file(&param_file_path).with_context(|| {
                format!("While reading parameter file at {:?}", param_file_path)
            })?,
        )?;
        update_from(
            &mut params,
            read_config_file(&config_file_path)
                .with_context(|| format!("While reading config file at {:?}", config_file_path))?,
        )?;
        // update_from(&mut params, read_job_file(&job_file_path)?)?;
        Ok(SimParams::new(params)?)
    }

    pub fn insert(&mut self, key: &str, value: &ParamValue) -> Option<ParamValue> {
        self.params.insert(key.to_string(), value.clone())
    }

    pub fn iter(&self) -> Iter<String, ParamValue> {
        self.params.iter()
    }

    pub fn keys(&self) -> Keys<String, ParamValue> {
        self.params.keys()
    }

    pub fn new(params: HashMap<String, ParamValue>) -> Result<SimParams> {
        Ok(SimParams {
            time_limit_cpu: Time::new::<second>(try_get(&params, "TimeLimitCPU")?.unwrap_f64()),
            params,
        })
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.params.contains_key(key)
    }
}
pub fn try_get<'a>(map: &'a HashMap<String, ParamValue>, key: &str) -> Result<&'a ParamValue> {
    map.get(key)
        .ok_or_else(|| anyhow!("Key not found: {}", key))
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

fn read_config_file(path: &Path) -> Result<HashMap<String, ParamValue>> {
    let contents = read_contents(path).context(format!("While reading config file {:?}", path))?;
    read_config_lines(&contents, "#")
}

fn read_config_lines(content: &str, comment_string: &str) -> Result<HashMap<String, ParamValue>> {
    let mut params = HashMap::new();
    for param in AVAILABLE_CONFIG_PARAMS {
        params.insert(param.to_string(), ParamValue::Bool(false));
    }
    for line in get_nonempty_noncomment_lines(content, comment_string) {
        let (key, value) = match line.contains(&"=") {
            true => {
                let split: Vec<&str> = line.split("=").collect();
                match split.len() {
                    2 => Ok((split[0].to_string(), ParamValue::from_str(split[1])?)),
                    _ => Err(anyhow!(format!(
                        "Invalid line in parameter file:\n{}",
                        line,
                    ))),
                }
            }
            false => Ok((line.to_string(), ParamValue::Bool(true))),
        }?;
        match params.insert(key, value) {
            None => return Err(anyhow!("Found invalid config parameter: {}", line)),
            _ => {}
        }
    }
    Ok(params)
}

fn read_param_file(path: &Path) -> Result<HashMap<String, ParamValue>> {
    let contents =
        read_contents(path).context(format!("While reading parameter file {:?}", path))?;
    let re = Regex::new("^([^ ]*?) +([^ ]*)[ %]*$").unwrap();
    let key_value_strings = read_parameter_lines(&contents, &re, "%")?;
    key_value_strings
        .into_iter()
        .map(|(k, v)| ParamValue::from_str(&v).map(|x| (k, x)))
        .collect()
}

pub fn read_contents(path: &Path) -> Result<String> {
    fs::read_to_string(path).context("While reading file")
}

fn get_nonempty_noncomment_lines<'a, 'b>(
    contents: &'a str,
    comment_string: &'a str,
) -> Box<dyn Iterator<Item = &'a str> + 'a> {
    Box::new(
        contents
            .lines()
            .map(|line| line.trim())
            .filter(|line| line != &"")
            .filter(move |line| !line.starts_with(comment_string))
            .map(move |line| match line.find(comment_string) {
                None => line,
                Some(index) => &line[..index],
            }),
    )
}

fn read_parameter_lines(
    contents: &str,
    pattern: &Regex,
    comment_string: &str,
) -> Result<HashMap<String, String>> {
    get_nonempty_noncomment_lines(contents, comment_string)
        .map(|line| {
            let mut captures = pattern.captures_iter(line);
            dbg!(&line);
            for cap in captures {
                dbg!(&cap);
            }
            let mut captures = pattern.captures_iter(line);
            captures.next().filter(|cap| cap.len() == 3).map_or_else(
                || {
                    Err(anyhow!(format!(
                        "Invalid line in parameter file:\n{}",
                        line,
                    )))
                },
                |cap| Ok((cap[1].to_string(), cap[2].to_string())),
            )
        })
        .collect()
}

impl Index<&str> for SimParams {
    type Output = ParamValue;

    fn index(&self, key: &str) -> &ParamValue {
        &self.params[key]
    }
}
