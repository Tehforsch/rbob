use std::{collections::HashMap, fs, path::Path, slice::Iter};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::simulation::Simulation;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum SubstitutionType {
    Normal,
    Cartesian(CartesianType),
}

#[derive(Serialize, Deserialize)]
enum CartesianType {
    All,
    GroupedParameters(Vec<Vec<String>>),
}

#[derive(Serialize, Deserialize)]
pub struct SimSetConfig {
    substitutionType: SubstitutionType,
    substitutions: HashMap<String, Value>,
}

impl SimSetConfig {
    pub fn from_file<U: AsRef<Path>>(path: U) -> Result<SimSetConfig> {
        let data = fs::read_to_string(path).context("While reading bob config file")?;
        Ok(serde_yaml::from_str(&data).context("Reading bob config file contents")?)
    }
}

pub struct SimSet {
    config: SimSetConfig,
    simulations: Vec<Simulation>,
}

impl SimSet {
    pub fn from_file<U: AsRef<Path>>(path: U) -> Result<SimSet> {
        let config = SimSetConfig::from_file(path)?;
        Ok(SimSet {
            config,
            simulations: vec![],
        })
    }

    pub fn iter(&self) -> Iter<Simulation> {
        self.simulations.iter()
    }
}
