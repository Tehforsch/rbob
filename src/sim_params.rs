use crate::{
    arepo_log_file::ArepoLogFile, config, sim_units::SimUnits, strfmt_utils::strfmt_anyhow,
    util::copy_file,
};
use anyhow::{anyhow, Context, Result};
use camino::{Utf8Path, Utf8PathBuf};
use itertools::Itertools;
use std::{
    collections::hash_map::Iter, collections::hash_map::Keys, collections::HashMap, fs, ops::Index,
    str::FromStr,
};

use crate::job_params::JobParams;
use crate::param_value::ParamValue;
use crate::util::{read_file_contents, write_file};

use regex::Regex;
use uom::si::{
    f64::{Length, Mass, Time, Velocity},
    length::centimeter,
    mass::gram,
    time::second,
    velocity::centimeter_per_second,
};

#[derive(Debug, Clone, PartialEq)]
pub enum SimParamsKind {
    Input,
    Output,
}

#[derive(Debug, Clone)]
pub struct SimParams {
    pub folder: Utf8PathBuf,
    params: HashMap<String, ParamValue>,
    pub time_limit_cpu: Time,
    pub units: SimUnits,
    pub kind: SimParamsKind,
}

impl SimParams {
    pub fn from_folder<U: AsRef<Utf8Path>>(folder: U, kind: SimParamsKind) -> Result<SimParams> {
        let mut params = HashMap::new();
        let param_file_path = get_param_file_path(&folder);
        let config_file_path = get_config_file_path(&folder);
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
        update_from(&mut params, get_job_file_params())?;
        SimParams::new(folder.as_ref(), params, kind)
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

    pub fn get(&self, key: &str) -> Option<&ParamValue> {
        self.params.get(key)
    }

    pub fn get_default_string(&self, key: &str, default: &str) -> String {
        self.get(key)
            .map(|s| s.unwrap_string().to_owned())
            .unwrap_or_else(|| default.to_owned())
    }

    pub fn get_default_i64(&self, key: &str, default: &i64) -> i64 {
        self.get(key)
            .map(|s| s.unwrap_i64())
            .unwrap_or_else(|| default.to_owned())
    }

    pub fn new(
        folder: &Utf8Path,
        params: HashMap<String, ParamValue>,
        kind: SimParamsKind,
    ) -> Result<SimParams> {
        let get_f64 = |k| try_get_f64(&params, k);
        let units = SimUnits::new(
            Length::new::<centimeter>(get_f64("UnitLength_in_cm")?),
            Velocity::new::<centimeter_per_second>(get_f64("UnitVelocity_in_cm_per_s")?),
            Mass::new::<gram>(get_f64("UnitMass_in_g")?),
        );
        Ok(SimParams {
            folder: folder.to_owned(),
            time_limit_cpu: Time::new::<second>(get_f64("TimeLimitCPU")?),
            units,
            params,
            kind,
        })
    }

    pub fn get_name(&self) -> String {
        self.folder.file_name().unwrap().to_owned()
    }

    pub fn output_folder(&self) -> Utf8PathBuf {
        get_output_folder_from_sim_folder(self, &self.folder)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.params.contains_key(key)
    }

    pub fn write_param_file(&self, path: &Utf8Path) -> Result<()> {
        let contents = self.get_param_file_contents();
        write_file(path, &contents)?;
        Ok(())
    }

    fn get_param_file_contents(&self) -> String {
        let mut sorted_keys: Vec<&String> = self.keys().collect();
        sorted_keys.sort();
        sorted_keys
            .iter()
            .filter(|key| config::PARAM_FILE_PARAMS.contains(&key.as_str()))
            .map(|key| format!("{}    {}", key, self[key]))
            .join("\n")
    }

    pub fn write_config_file(&self, path: &Utf8Path) -> Result<()> {
        let contents = self.get_config_file_contents();
        write_file(path, &contents)?;
        Ok(())
    }

    fn get_config_file_contents(&self) -> String {
        let mut sorted_keys: Vec<&String> = self.keys().collect();
        sorted_keys.sort();
        sorted_keys
            .iter()
            .filter(|key| config::CONFIG_FILE_PARAMS.contains(&key.as_str()))
            .map(|key| match &self[key] {
                ParamValue::Bool(value) => match value {
                    true => Some(key.to_string()),
                    false => None,
                },
                ParamValue::Int(value) => Some(format!("{}={}", key, value)),
                ParamValue::Float(_, s) => Some(format!("{}={}", key, s)),
                _ => panic!("Wrong param value: {}", key),
            })
            .flatten()
            .join("\n")
    }

    pub fn write_job_file(&self, path: &Utf8Path) -> Result<()> {
        let contents = self.get_job_file_contents()?;
        write_file(path, &contents)?;
        Ok(())
    }

    pub fn copy_initial_snapshot_if_needed(
        &self,
        source_folder: &Utf8Path,
        target_folder: &Utf8Path,
    ) -> Result<()> {
        if let Some(initial_snap_file_name) = self.get(config::INITIAL_SNAP_IDENTIFIER) {
            let initial_snap_file = source_folder.join(initial_snap_file_name.unwrap_string());
            let snap_file_base = self.get("SnapshotFileBase").unwrap();
            let sim_output_folder = get_output_folder_from_sim_folder(self, target_folder);
            fs::create_dir_all(&sim_output_folder)?;
            let target_file = sim_output_folder.join(format!(
                "{snap_file_base}_000.hdf5",
                snap_file_base = snap_file_base
            ));
            copy_file(initial_snap_file, target_file)?;
        }
        Ok(())
    }

    fn get_job_file_contents(&self) -> Result<String> {
        let job_params = self.get_job_params()?;
        let replacements = job_params.to_hashmap();
        strfmt_anyhow(&config::JOB_FILE_TEMPLATE, replacements)
    }

    fn get_job_params(&self) -> Result<JobParams> {
        JobParams::new(self)
    }

    pub fn get_log_file(&self) -> ArepoLogFile {
        ArepoLogFile::new(&self.folder.join(config::DEFAULT_LOG_FILE))
    }

    pub fn get_num_cores(&self) -> Result<i64> {
        // For input params, the number of cores should be readable directly from the params
        // For output params, we will read the arepo log file and check for the corresponding line
        // because that is the most accurate way to determine the number of cores.
        match self.kind {
            SimParamsKind::Input => Ok(self.params["numCores"].unwrap_i64()),
            SimParamsKind::Output => self.get_log_file().get_num_cores(),
        }
    }

    pub fn get_run_time(&self) -> Result<f64> {
        assert_eq!(self.kind, SimParamsKind::Output);
        self.get_log_file().get_run_time()
    }
}
pub fn get_output_folder_from_sim_folder(sim: &SimParams, sim_folder: &Utf8Path) -> Utf8PathBuf {
    sim_folder.join(Utf8Path::new(sim.params["OutputDir"].unwrap_string()))
}

pub fn get_param_file_path<U: AsRef<Utf8Path>>(folder: U) -> Utf8PathBuf {
    folder.as_ref().join(config::DEFAULT_PARAM_FILE_NAME)
}

pub fn get_config_file_path<U: AsRef<Utf8Path>>(folder: U) -> Utf8PathBuf {
    folder.as_ref().join(config::DEFAULT_CONFIG_FILE_NAME)
}

pub fn get_job_file_path<U: AsRef<Utf8Path>>(folder: U) -> Utf8PathBuf {
    folder.as_ref().join(config::DEFAULT_JOB_FILE_NAME)
}

pub fn try_get_f64(map: &HashMap<String, ParamValue>, key: &str) -> Result<f64> {
    map.get(key)
        .map(|v| v.unwrap_f64())
        .ok_or_else(|| anyhow!("Key not found: {}", key))
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

fn get_job_file_params() -> HashMap<String, ParamValue> {
    HashMap::new()
}

fn read_config_file(path: &Utf8Path) -> Result<HashMap<String, ParamValue>> {
    let contents =
        read_file_contents(path).context(format!("While reading config file {:?}", path))?;
    read_config_lines(&contents, "#")
}

fn read_config_lines(content: &str, comment_string: &str) -> Result<HashMap<String, ParamValue>> {
    let mut params = HashMap::new();
    for param in config::CONFIG_FILE_PARAMS {
        params.insert(param.to_string(), ParamValue::Bool(false));
    }
    for line in get_nonempty_noncomment_lines(content, comment_string) {
        let (mut key, value) = match line.contains(&"=") {
            true => {
                let split: Vec<&str> = line.split('=').collect();
                match split.len() {
                    2 => Ok((split[0].to_string(), ParamValue::from_str(split[1])?)),
                    _ => Err(anyhow!(format!(
                        "Invalid line in config file:\n\"{}\"",
                        line,
                    ))),
                }
            }
            false => Ok((line.to_string(), ParamValue::Bool(true))),
        }?;
        key = key.trim_start().trim_end().to_string();
        if params.insert(key, value) == None {
            return Err(anyhow!("Found invalid config parameter: {}", line));
        }
    }
    Ok(params)
}

fn read_param_file(path: &Utf8Path) -> Result<HashMap<String, ParamValue>> {
    let contents =
        read_file_contents(path).context(format!("While reading parameter file {:?}", path))?;
    let re = Regex::new("^([^ ]*?)\\s+([^ ]*)\\s*[;%]*.*$").unwrap();
    let key_value_strings = read_parameter_lines(&contents, &re, "%")?;
    key_value_strings
        .into_iter()
        .map(|(k, v)| ParamValue::from_str(&v).map(|x| (k, x)))
        .collect()
}

fn get_nonempty_noncomment_lines<'a>(
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
            // let mut captures = pattern.captures_iter(line);
            // for cap in captures {
            //     dbg!("???{:?}???", &cap);
            // }
            // dbg!(&line);
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
