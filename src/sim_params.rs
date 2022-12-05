use std::collections::hash_map::Iter;
use std::collections::hash_map::Keys;
use std::collections::HashMap;
use std::fs;
use std::ops::Index;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use camino::Utf8Path;
use camino::Utf8PathBuf;
use itertools::Itertools;
use regex::Regex;
use uom::si::f64::Length;
use uom::si::f64::Mass;
use uom::si::f64::Time;
use uom::si::f64::Velocity;
use uom::si::length::centimeter;
use uom::si::mass::gram;
use uom::si::time::second;
use uom::si::velocity::centimeter_per_second;

use crate::arepo_log_file::ArepoLogFile;
use crate::config;
use crate::job_params::JobParams;
use crate::param_value::ParamValue;
use crate::sim_units::SimUnits;
use crate::simplex_log_file::SimplexLogFile;
use crate::strfmt_utils::strfmt_anyhow;
use crate::util::copy_file;
use crate::util::read_file_contents;
use crate::util::write_file;

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
        let bob_param_file_path = get_bob_param_file_path(&folder);
        let param_file_path = get_param_file_path(&folder);
        let config_file_path = get_config_file_path(&folder);
        if bob_param_file_path.is_file() {
            update_from(
                &mut params,
                read_bob_param_file(&bob_param_file_path).with_context(|| {
                    format!(
                        "While reading bob parameter file at {:?}",
                        bob_param_file_path
                    )
                })?,
            )?;
        }

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

    pub fn get_default_bool(&self, key: &str, default: bool) -> bool {
        self.get(key)
            .map(|s| s.unwrap_bool())
            .unwrap_or_else(|| default)
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
            .filter_map(|key| {
                self[key]
                    .as_option()
                    .map(|value| format!("{}    {}", key, value))
            })
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
                ParamValue::Str(s) => Some(format!("{}={}", key, s)),
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

    pub fn get_ics_filename(&self) -> Utf8PathBuf {
        let ics_file_base = self.get("InitCondFile").unwrap().unwrap_string();
        let ics_format = self.get("ICFormat").unwrap().unwrap_i64();
        let ics_extension = match ics_format {
            3 => "hdf5",
            1 => "",
            _ => unimplemented!(),
        };
        let path = Utf8Path::new(ics_file_base);
        let filename_with_extension = path.with_extension(ics_extension);
        if filename_with_extension.is_file() {
            filename_with_extension.into()
        } else {
            // Simply return the path to the parent folder of the initial conditions
            let f =  path.parent().unwrap().into();
            println!("Did not find ICS file at {:?}, assuming ICS are a folder at {:?}", filename_with_extension, f);
            f
        }
    }

    pub fn copy_ics(&self, target_folder: &Utf8Path, symlink_ics: bool) -> Result<()> {
        let sim_output_folder = get_output_folder_from_sim_folder(self, target_folder);
        let ics_file_name = self.get_ics_filename();
        // Nothing to do if the ICS are given as an absolute path
        if ics_file_name.is_absolute() {
            return Ok(());
        }
        fs::create_dir_all(&sim_output_folder)?;
        let source = self.folder.join(&ics_file_name);
        let target = target_folder.join(&ics_file_name);
        if symlink_ics {
            std::os::unix::fs::symlink(
                &source.canonicalize().context(format!(
                    "While trying to obtain absolute path to initial conditions at {:?}",
                    source
                ))?,
                &target,
            )
            .context(format!(
                "While symlinking ics file from {:?} to {:?}",
                source, target
            ))?;
        } else {
            copy_file(&source, &target)?;
        }
        Ok(())
    }

    pub fn copy_test_sources_file_if_exists(
        &self,
        source_folder: &Utf8Path,
        target_folder: &Utf8Path,
    ) -> Result<()> {
        if let Some(sources_file_identifier) = self.get("TestSrcFile") {
            let sources_file_name = sources_file_identifier.unwrap_string();
            let sources_file = source_folder.join(&sources_file_name);
            let output_sources_file = target_folder.join(&sources_file_name);
            copy_file(sources_file, output_sources_file)?;
        }
        Ok(())
    }

    pub fn copy_treecool_file_if_exists(
        &self,
        source_folder: &Utf8Path,
        target_folder: &Utf8Path,
    ) -> Result<()> {
        if let Some(treecool_file_identifier) = self.get("TreecoolFile") {
            let treecool_file_name = treecool_file_identifier.unwrap_string();
            let treecool_file = source_folder.join(&treecool_file_name);
            let output_treecool_file = target_folder.join(&treecool_file_name);
            copy_file(treecool_file, output_treecool_file)?;
        }
        Ok(())
    }

    fn get_job_file_contents(&self) -> Result<String> {
        let job_params = self.get_job_params()?;
        self.get_job_file_contents_from_job_params(&job_params)
    }

    pub fn get_job_file_contents_from_job_params(&self, job_params: &JobParams) -> Result<String> {
        let replacements = job_params.to_hashmap();
        strfmt_anyhow(&config::JOB_FILE_TEMPLATE, replacements)
    }

    pub fn get_job_params(&self) -> Result<JobParams> {
        JobParams::new(self)
    }

    pub fn write_bob_param_file(&self, path: &Utf8Path) -> Result<()> {
        let contents = serde_yaml::to_string(&self.params)?;
        write_file(&path, &contents)
    }

    pub fn get_log_file(&self) -> ArepoLogFile {
        ArepoLogFile::new(&self.folder.join(config::DEFAULT_LOG_FILE))
    }

    pub fn get_simplex_file(&self) -> SimplexLogFile {
        SimplexLogFile::new(&self.output_folder().join(config::DEFAULT_SIMPLEX_LOG_FILE))
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

    pub fn get_rt_run_time(&self) -> Result<f64> {
        assert_eq!(self.kind, SimParamsKind::Output);
        self.get_log_file().get_run_time(
            self.get("SWEEP")
                .unwrap_or(&ParamValue::Bool(false))
                .unwrap_bool(),
        )
    }

    pub fn get_rt_run_time_per_iteration(&self) -> Result<f64> {
        assert_eq!(self.kind, SimParamsKind::Output);
        let num_pbc_iterations = self
            .get("SweepMaxNumIterations")
            .map(|num| num.unwrap_i64())
            .unwrap_or(1);
        let num_rotations = self.get("SX_NUM_ROT").unwrap().unwrap_i64();
        let run_params = self.get("runParams").unwrap().unwrap_string();
        let re = Regex::new("21 ([0-9+]+).*").unwrap();
        let cap = re.captures_iter(&run_params).next().ok_or_else(|| {
            anyhow!("Not a postprocessing run, failed to get number of iterations")
        })?;
        let num_iterations: i32 = cap
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .context("Wrong format of run params")?;
        self.get_rt_run_time().map(|run_time| {
            run_time / num_pbc_iterations as f64 / num_iterations as f64 / num_rotations as f64
        })
    }

    pub fn get_num_sweep_runs(&self) -> Result<usize> {
        self.get_log_file().get_num_sweep_runs()
    }
}
pub fn get_output_folder_from_sim_folder(sim: &SimParams, sim_folder: &Utf8Path) -> Utf8PathBuf {
    sim_folder.join(Utf8Path::new(sim.params["OutputDir"].unwrap_string()))
}

pub fn get_bob_param_file_path<U: AsRef<Utf8Path>>(folder: U) -> Utf8PathBuf {
    folder.as_ref().join(config::DEFAULT_BOB_PARAM_FILE_NAME)
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
        if let Some(previous_value) = params.insert(key.clone(), value.clone()) {
            if previous_value != value {
                eprintln!(
                    "Differing values of parameter: {}: {} {}",
                    key, value, previous_value
                );
            }
        }
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
    let mut invalid_keys = vec![];
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
        if params.insert(key.clone(), value) == None {
            invalid_keys.push(key);
        }
    }
    if !invalid_keys.is_empty() {
        return Err(anyhow!(
            "Found invalid config parameters:\n{}",
            invalid_keys
                .iter()
                .map(|x| format!("\"{}\",", x))
                .join("\n")
        ));
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
        .filter_map(|(k, v)| {
            ParamValue::from_str(&v)
                .map(|x| x.into_option().map(|x| (k, x)))
                .transpose()
        })
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

fn read_bob_param_file(path: &Utf8Path) -> Result<HashMap<String, ParamValue>> {
    let contents = read_file_contents(path)?;
    serde_yaml::from_str(&contents).context("While reading plot info file")
}

impl Index<&str> for SimParams {
    type Output = ParamValue;

    fn index(&self, key: &str) -> &ParamValue {
        self.params
            .get(key)
            .expect(&format!("Key not found in sim: {}", key))
    }
}
