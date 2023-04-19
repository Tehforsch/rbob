use std::fs;

use anyhow::Context;
use anyhow::Result;
use camino::Utf8Path;
use camino::Utf8PathBuf;
use serde_yaml::Mapping;
use serde_yaml::Value;

use crate::config;
use crate::job_params::JobParams;
use crate::strfmt_utils::strfmt_anyhow;
use crate::util::copy_file;
use crate::util::write_file;

#[derive(Debug, Clone, PartialEq)]
pub enum SimParamsKind {
    Input,
    Output,
}

#[derive(Debug, Clone)]
pub struct SimParams {
    pub folder: Utf8PathBuf,
    params: Mapping,
    pub kind: SimParamsKind,
}

#[derive(PartialEq, Eq)]
enum ParamType {
    Param,
    Special,
}

impl From<&str> for ParamType {
    fn from(value: &str) -> Self {
        if config::SPECIAL_PARAMS.contains(&value) {
            Self::Special
        } else {
            Self::Param
        }
    }
}

impl SimParams {
    pub fn from_folder<U: AsRef<Utf8Path>>(folder: U, kind: SimParamsKind) -> Result<SimParams> {
        let param_file_path = get_param_file_path(&folder);
        let params = read_param_file(&param_file_path)
            .with_context(|| format!("While reading parameter file at {:?}", param_file_path))?;
        SimParams::new(folder.as_ref(), params.as_mapping().unwrap().clone(), kind)
    }

    fn get_param_mut(&mut self, key: &str) -> Option<&mut Value> {
        let indices: Vec<_> = key.split("/").collect();
        get_param_mut_from_indices(&mut self.params, &indices)
    }

    fn get_param(&self, key: &str) -> Option<&Value> {
        let indices: Vec<_> = key.split("/").collect();
        get_param_from_indices(&self.params, &indices)
    }

    pub fn insert(&mut self, key: &str, value: &Value) {
        *self
            .get_param_mut(key)
            .unwrap_or_else(|| panic!("Failed to find key: {}", key)) = value.clone();
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.get_param(key).map(|v| &*v)
    }

    pub fn get_default_string(&self, key: &str, default: &str) -> String {
        self.get(key)
            .map(|s| s.as_str().unwrap().to_owned())
            .unwrap_or_else(|| default.to_owned())
    }

    pub fn get_default_i64(&self, key: &str, default: &i64) -> i64 {
        self.get(key)
            .map(|s| s.as_i64().unwrap())
            .unwrap_or_else(|| default.to_owned())
    }

    pub fn get_default_bool(&self, key: &str, default: bool) -> bool {
        self.get(key)
            .map(|s| s.as_bool().unwrap())
            .unwrap_or_else(|| default)
    }

    pub fn new(folder: &Utf8Path, params: Mapping, kind: SimParamsKind) -> Result<SimParams> {
        // Super ugly, but I don't want to overengineer this for now
        let job_params = JobParams::default();
        let mut sim_params = SimParams {
            folder: folder.to_owned(),
            params,
            kind,
        };
        sim_params.params.insert(
            Value::String("job".into()),
            serde_yaml::to_value(&job_params).unwrap(),
        );
        Ok(sim_params)
    }

    pub fn get_name(&self) -> String {
        self.folder.file_name().unwrap().to_owned()
    }

    pub fn output_folder(&self) -> Utf8PathBuf {
        get_output_folder_from_sim_folder(self, &self.folder)
    }

    pub fn write_param_file(&self, path: &Utf8Path) -> Result<()> {
        let contents = self.get_param_file_contents();
        write_file(path, &contents)?;
        Ok(())
    }

    fn get_param_file_contents(&self) -> String {
        let mut params = self.params.clone();
        params.remove("job");
        serde_yaml::to_string(&params).unwrap()
    }

    pub fn write_job_file(&self, path: &Utf8Path) -> Result<()> {
        let contents = self.get_job_file_contents()?;
        write_file(path, &contents)?;
        Ok(())
    }

    pub fn get_ics_files(&self) -> Vec<Utf8PathBuf> {
        let ics_files = self.get("input/paths").unwrap();
        ics_files
            .as_sequence()
            .unwrap()
            .into_iter()
            .map(|f| Utf8Path::new(f.as_str().unwrap()).into())
            .collect()
    }

    pub fn copy_ics(&self, target_folder: &Utf8Path, symlink_ics: bool) -> Result<()> {
        let sim_output_folder = get_output_folder_from_sim_folder(self, target_folder);
        for ics_file_name in self.get_ics_files() {
            // Nothing to do if the ICS are given as an absolute path
            if ics_file_name.is_absolute() {
                continue;
            }
            fs::create_dir_all(&sim_output_folder)?;
            let source = self.folder.join(&ics_file_name);
            let target = target_folder.join(&ics_file_name);
            fs::create_dir_all(target.parent().unwrap())
                .expect("Failed to create target directory");
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
        }
        Ok(())
    }

    fn get_job_file_contents(&self) -> Result<String> {
        self.get_job_file_contents_from_job_params()
    }

    pub fn get_job_file_contents_from_job_params(&self) -> Result<String> {
        let to_str = |v: &Value| match v {
            Value::Null => todo!(),
            Value::Bool(b) => b.to_string(),
            Value::Number(x) => x.to_string(),
            Value::String(s) => s.to_owned(),
            Value::Sequence(_) => todo!(),
            Value::Mapping(_) => todo!(),
            Value::Tagged(_) => todo!(),
        };
        let replacements = self.params["job"]
            .as_mapping()
            .unwrap()
            .into_iter()
            .map(|(k, v)| (to_str(k), to_str(v)))
            .collect();
        strfmt_anyhow(&config::JOB_FILE_TEMPLATE, replacements)
    }

    pub fn write_bob_param_file(&self, path: &Utf8Path) -> Result<()> {
        let contents = serde_yaml::to_string(&self.params)?;
        write_file(&path, &contents)
    }

    pub fn get_num_cores(&self) -> Result<i64> {
        match self.kind {
            SimParamsKind::Input => Ok(self.get("numCores").unwrap().as_i64().unwrap()),
            SimParamsKind::Output => todo!(),
        }
    }
}

// I dont know how to make functions generic over mutability ...
fn get_param_mut_from_indices<'a>(
    params: &'a mut Mapping,
    indices: &[&str],
) -> Option<&'a mut Value> {
    if indices.len() == 1 {
        Some(params.get_mut(indices[0])?)
    } else {
        get_param_mut_from_indices(params[indices[0]].as_mapping_mut()?, &indices[1..])
    }
}

fn get_param_from_indices<'a>(params: &'a Mapping, indices: &[&str]) -> Option<&'a Value> {
    if indices.len() == 1 {
        Some(params.get(indices[0])?)
    } else {
        get_param_from_indices(params.get(indices[0])?.as_mapping()?, &indices[1..])
    }
}

pub fn get_output_folder_from_sim_folder(sim: &SimParams, sim_folder: &Utf8Path) -> Utf8PathBuf {
    sim_folder.join(Utf8Path::new(
        &sim.get_default_string("output/folder", "output"),
    ))
}

pub fn get_param_file_path<U: AsRef<Utf8Path>>(folder: U) -> Utf8PathBuf {
    folder.as_ref().join(config::DEFAULT_PARAM_FILE_NAME)
}

fn read_param_file(path: &Utf8Path) -> Result<Value> {
    let data = fs::read_to_string(path)
        .context(format!("While reading raxiom param file at {:?}", path,))?;
    serde_yaml::from_str(&data).context("Reading param file contents")
}
