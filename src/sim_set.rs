use std::collections::HashMap;
use std::fs;
use std::iter::FromIterator;
use std::slice::Iter;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use camino::Utf8Path;
use camino::Utf8PathBuf;
use itertools::Itertools;
use serde::Deserialize;
use serde::Serialize;
use serde_yaml::Value;

use crate::config;
use crate::sim_params::SimParams;
use crate::sim_params::SimParamsKind;
use crate::util::get_folders;

#[derive(Serialize, Deserialize)]
enum CartesianType {
    NoCartesian,
    All,
    Grouped(Vec<Vec<String>>),
}

#[derive(Serialize, Deserialize)]
struct SimSetConfig {
    cartesian_type: CartesianType,
    substitutions: HashMap<String, Value>,
}

impl SimSetConfig {
    fn from_file<U: AsRef<Utf8Path>>(path: U) -> Result<SimSetConfig> {
        let data = fs::read_to_string(path.as_ref()).context(format!(
            "While reading bob config file at {:?}",
            path.as_ref()
        ))?;
        serde_yaml::from_str(&data).context("Reading bob config file contents")
    }
}

#[derive(Clone)]
pub struct SimSet {
    simulations: Vec<(usize, SimParams)>,
}

impl FromIterator<(usize, SimParams)> for SimSet {
    fn from_iter<T: IntoIterator<Item = (usize, SimParams)>>(iter: T) -> Self {
        SimSet {
            simulations: iter.into_iter().collect(),
        }
    }
}

impl IntoIterator for SimSet {
    type Item = (usize, SimParams);
    type IntoIter = std::vec::IntoIter<(usize, SimParams)>;
    fn into_iter(self) -> Self::IntoIter {
        self.simulations.into_iter()
    }
}

impl SimSet {
    pub fn from_bob_file_and_input_folder<U: AsRef<Utf8Path>, V: AsRef<Utf8Path>>(
        config_file_path: U,
        folder: V,
    ) -> Result<SimSet> {
        let config = SimSetConfig::from_file(config_file_path)?;
        let simulations = get_sim_params(
            &config,
            SimParams::from_folder(folder.as_ref(), SimParamsKind::Input)?,
        )?;
        Ok(SimSet { simulations })
    }

    pub fn from_output_folder<U: AsRef<Utf8Path>>(folder: U) -> Result<SimSet> {
        let all_folders = get_folders(folder.as_ref())?;
        let mut sim_folders: Vec<(usize, &Utf8PathBuf)> = all_folders
            .iter()
            .map(|f| (f.file_name().unwrap().parse::<usize>(), f))
            .filter(|(maybe_num, _)| maybe_num.is_ok())
            .map(|(num, f)| (num.unwrap(), f))
            .collect();
        sim_folders.sort();
        sim_folders
            .into_iter()
            .map(|(num, f)| -> Result<(usize, SimParams)> {
                Ok((num, SimParams::from_folder(f, SimParamsKind::Output)?))
            })
            .collect()
    }

    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a SimParams> + 'a> {
        Box::new(self.simulations.iter().map(|(_, s)| s))
    }

    pub fn enumerate(&self) -> Iter<(usize, SimParams)> {
        self.simulations.iter()
    }
}

fn get_sim_params(
    config: &SimSetConfig,
    base_sim_params: SimParams,
) -> Result<Vec<(usize, SimParams)>> {
    let substitutions = match &config.cartesian_type {
        CartesianType::NoCartesian => get_substitutions_normal(&config.substitutions),
        CartesianType::All => get_substitutions_cartesian(&config.substitutions, None),
        CartesianType::Grouped(l) => {
            get_substitutions_cartesian(&config.substitutions, Some(l.to_vec()))
        }
    }?;
    get_sim_params_from_substitutions(base_sim_params, substitutions)
}

fn get_sim_params_from_substitutions(
    base: SimParams,
    substitutions: Vec<HashMap<String, Value>>,
) -> Result<Vec<(usize, SimParams)>> {
    substitutions
        .iter()
        .enumerate()
        .map(|(i, substitution_map)| {
            let mut new_sim = base.clone();
            for (k, v) in substitution_map.iter() {
                if new_sim.insert(k, v) == None && !is_special_param(k) {
                    return Err(anyhow!("Found (non-special) parameter in substitutions that does not appear in parameter files: {}", k));
                }
            }
            Ok((i as usize, new_sim))
        })
        .collect()
}

fn is_special_param(k: &str) -> bool {
    config::SPECIAL_PARAMS.contains(&k)
}

fn get_substitutions_normal(
    substitutions: &HashMap<String, Value>,
) -> Result<Vec<HashMap<String, Value>>> {
    let length = count_length_or_singular(Box::new(substitutions.iter()))?;
    (0..length)
        .map(|i| {
            let mut subst = HashMap::new();
            for (k, v) in substitutions.iter() {
                let this_v = if let Value::Sequence(s) = v { &s[i] } else { v };
                subst.insert(k.into(), this_v.clone());
            }
            Ok(subst)
        })
        .collect()
}

fn get_parameter_groups(
    substitutions: &HashMap<String, Value>,
    grouped_params: Option<Vec<Vec<String>>>,
) -> Vec<Vec<String>> {
    let mut param_groups = grouped_params
        .unwrap_or_else(|| substitutions.keys().map(|k| vec![k.to_owned()]).collect());
    param_groups.sort();
    param_groups
}

fn get_substitutions_cartesian(
    substitutions: &HashMap<String, Value>,
    grouped_params: Option<Vec<Vec<String>>>,
) -> Result<Vec<HashMap<String, Value>>> {
    let parameter_groups = get_parameter_groups(substitutions, grouped_params);
    let lengths: Vec<usize> = parameter_groups
        .iter()
        .map(|group| {
            count_length_or_singular(Box::new(
                substitutions.iter().filter(|(k, _)| group.contains(k)),
            ))
        })
        .collect::<Result<Vec<usize>>>()?;
    let indices: Vec<Vec<usize>> = lengths
        .iter()
        .map(|l| (0..*l))
        .multi_cartesian_product()
        .collect();
    let mut result = vec![];
    let find_k_index = {
        |k| {
            parameter_groups
                .iter()
                .enumerate()
                .find(|(_, group)| group.contains(k))
                .ok_or_else(|| {
                    anyhow!(format!(
                        "Key has list of values but is not listed in the parameter groups: {}",
                        k
                    ))
                })
                .map(|v| v.0)
        }
    };
    for multi_index in indices {
        let mut r = HashMap::new();
        for (k, param) in substitutions.iter() {
            let k_index = find_k_index(k)?;
            let value = if let Value::Sequence(s) = param {
                &s[multi_index[k_index]]
            } else {
                param
            };
            r.insert(k.to_owned(), value.clone());
        }
        result.push(r);
    }
    Ok(result)
}

fn count_length_or_singular<'a>(
    items: Box<dyn 'a + Iterator<Item = (&'a String, &'a Value)>>,
) -> Result<usize> {
    let mut length: Option<usize> = None;
    for (_, value) in items {
        if let Value::Sequence(s) = value {
            match length {
                Some(l) => {
                    if l != s.len() {
                        return Err(anyhow!("Found different lengths of parameter lists!"));
                    }
                }
                None => {
                    length = Some(s.len());
                }
            }
        }
    }
    Ok(length.unwrap_or(1))
}
