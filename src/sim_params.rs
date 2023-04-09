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

    pub fn insert(&mut self, key: &str, value: &Value) -> Option<Value> {
        self.params.insert(key.into(), value.clone())
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.params.get(&Value::String(key.into()))
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
        Ok(SimParams {
            folder: folder.to_owned(),
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

    pub fn write_param_file(&self, path: &Utf8Path) -> Result<()> {
        let contents = self.get_param_file_contents();
        write_file(path, &contents)?;
        Ok(())
    }

    fn get_param_file_contents(&self) -> String {
        serde_yaml::to_string(&self.params).unwrap()
    }

    pub fn write_job_file(&self, path: &Utf8Path) -> Result<()> {
        let contents = self.get_job_file_contents()?;
        write_file(path, &contents)?;
        Ok(())
    }

    pub fn get_ics_filename(&self) -> Utf8PathBuf {
        todo!()
        // let ics_file_base = self.get("input").unwrap().as_str().unwrap();
        // let ics_format = self.get("ICFormat").unwrap().as_i64().unwrap();
        // let ics_extension = match ics_format {
        //     3 => "hdf5",
        //     1 => "",
        //     _ => unimplemented!(),
        // };
        // let path = Utf8Path::new(ics_file_base);
        // let filename_with_extension = path.with_extension(ics_extension);
        // if self.folder.join(&filename_with_extension).is_file() {
        //     filename_with_extension.into()
        // } else {
        //     // Simply return the path to the parent folder of the initial conditions
        //     let f = path.parent().unwrap().into();
        //     println!(
        //         "Did not find ICS file at {:?}, assuming ICS are a folder at {:?}",
        //         filename_with_extension, f
        //     );
        //     f
        // }
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

    pub fn get_num_cores(&self) -> Result<i64> {
        match self.kind {
            SimParamsKind::Input => Ok(self.get("numCores").unwrap().as_i64().unwrap()),
            SimParamsKind::Output => todo!(),
        }
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
