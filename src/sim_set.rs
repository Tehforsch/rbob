use camino::Utf8Path;
use camino::Utf8PathBuf;
use std::{collections::HashMap, fs, iter::FromIterator, slice::Iter};

use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::sim_params::SimParams;
use crate::util::get_folders;
use crate::{param_value::ParamValue, sim_params::SimParamsKind};

#[derive(Serialize, Deserialize)]
enum CartesianType {
    NoCartesian,
    All,
    Grouped(Vec<Vec<String>>),
}

#[derive(Serialize, Deserialize)]
pub struct SimSetConfig {
    cartesian_type: CartesianType,
    substitutions: HashMap<String, Value>,
}

impl SimSetConfig {
    pub fn from_file<U: AsRef<Utf8Path>>(path: U) -> Result<SimSetConfig> {
        let data = fs::read_to_string(path.as_ref()).context(format!(
            "While reading bob config file at {:?}",
            path.as_ref()
        ))?;
        Ok(serde_yaml::from_str(&data).context("Reading bob config file contents")?)
    }
}

pub struct SimSet {
    // config: SimSetConfig,
    simulations: Vec<(usize, SimParams)>,
}

impl FromIterator<(usize, SimParams)> for SimSet {
    fn from_iter<T: IntoIterator<Item = (usize, SimParams)>>(iter: T) -> Self {
        SimSet {
            simulations: iter.into_iter().collect(),
        }
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

    pub fn get_folder(&self) -> Result<Utf8PathBuf> {
        self.simulations
            .get(0)
            .ok_or_else(|| anyhow!("No simulation in sim set, cannot determine folder."))
            .map(|(_, sim)| sim.folder.parent().unwrap().to_owned())
    }

    pub fn len(&self) -> usize {
        self.simulations.len()
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
    substitutions: Vec<HashMap<String, ParamValue>>,
) -> Result<Vec<(usize, SimParams)>> {
    substitutions
        .iter()
        .enumerate()
        .map(|(i, substitution_map)| {
            let mut new_sim = base.clone();
            for (k, v) in substitution_map.iter() {
                match new_sim.insert(k, v) {
                    None => Err(anyhow!("Found parameter in substitutions that does not appear in parameter files: {}", k))?,
                    _ => {}
                }
            }
            Ok((i as usize, new_sim))
        })
        .collect()
}

fn get_substitutions_normal(
    substitutions: &HashMap<String, Value>,
) -> Result<Vec<HashMap<String, ParamValue>>> {
    let length = count_length_or_singular(Box::new(substitutions.iter()))?;
    (0..length)
        .map(|i| {
            let mut subst = HashMap::new();
            for (k, v) in substitutions.iter() {
                let this_v = if let Value::Sequence(s) = v { &s[i] } else { v };
                subst.insert(k.to_string(), ParamValue::new(this_v)?);
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
) -> Result<Vec<HashMap<String, ParamValue>>> {
    let parameter_groups = get_parameter_groups(substitutions, grouped_params);
    let lengths: Vec<usize> = parameter_groups
        .iter()
        .map(|group| {
            count_length_or_singular(Box::new(
                substitutions.iter().filter(|(k, _)| group.contains(&k)),
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
            r.insert(k.to_owned(), ParamValue::new(value)?);
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml::to_value;
    #[test]
    fn normal_sim_set() -> Result<()> {
        let mut substitutions = HashMap::new();
        substitutions.insert("a".to_owned(), to_value([1, 2, 3])?);
        substitutions.insert("b".to_owned(), to_value([4, 5, 6])?);
        let s = get_substitutions_normal(&substitutions)?;
        assert_eq!(s.len(), 3);
        assert_eq!(s[0]["a"], ParamValue::Int(1));
        assert_eq!(s[0]["b"], ParamValue::Int(4));
        assert_eq!(s[1]["a"], ParamValue::Int(2));
        assert_eq!(s[1]["b"], ParamValue::Int(5));
        assert_eq!(s[2]["a"], ParamValue::Int(3));
        assert_eq!(s[2]["b"], ParamValue::Int(6));
        Ok(())
    }

    #[test]
    fn cartesian_sim_set_config() -> Result<()> {
        let mut substitutions = HashMap::new();
        substitutions.insert("a".to_owned(), to_value([1, 2])?);
        substitutions.insert("b".to_owned(), to_value([3, 4])?);
        let s = dbg!(get_substitutions_cartesian(&substitutions, None)?);
        assert_eq!(s.len(), 4);
        assert_eq!(s[0]["a"], ParamValue::Int(1));
        assert_eq!(s[0]["b"], ParamValue::Int(3));

        assert_eq!(s[1]["a"], ParamValue::Int(1));
        assert_eq!(s[1]["b"], ParamValue::Int(4));

        assert_eq!(s[2]["a"], ParamValue::Int(2));
        assert_eq!(s[2]["b"], ParamValue::Int(3));

        assert_eq!(s[3]["a"], ParamValue::Int(2));
        assert_eq!(s[3]["b"], ParamValue::Int(4));
        Ok(())
    }

    #[test]
    fn cartesian_sim_set_config_parameter_groups() -> Result<()> {
        let mut substitutions = HashMap::new();
        substitutions.insert("a".to_owned(), to_value([1, 2])?);
        substitutions.insert("b".to_owned(), to_value([3, 4])?);
        substitutions.insert("c".to_owned(), to_value([4, 5])?);
        let s = dbg!(get_substitutions_cartesian(
            &substitutions,
            Some(vec![
                vec!["a".to_owned(), "c".to_owned()],
                vec!["b".to_owned()]
            ])
        )?);
        assert_eq!(s.len(), 4);
        assert_eq!(s[0]["a"], ParamValue::Int(1));
        assert_eq!(s[0]["b"], ParamValue::Int(3));
        assert_eq!(s[0]["c"], ParamValue::Int(4));

        assert_eq!(s[1]["a"], ParamValue::Int(1));
        assert_eq!(s[1]["b"], ParamValue::Int(4));
        assert_eq!(s[1]["c"], ParamValue::Int(4));

        assert_eq!(s[2]["a"], ParamValue::Int(2));
        assert_eq!(s[2]["b"], ParamValue::Int(3));
        assert_eq!(s[2]["c"], ParamValue::Int(5));

        assert_eq!(s[3]["a"], ParamValue::Int(2));
        assert_eq!(s[3]["b"], ParamValue::Int(4));
        assert_eq!(s[3]["c"], ParamValue::Int(5));
        Ok(())
    }

    #[test]
    fn cartesian_sim_set_config_parameter_groups_wrong_lengths() -> Result<()> {
        let mut substitutions = HashMap::new();
        substitutions.insert("a".to_owned(), to_value([1, 2])?);
        substitutions.insert("b".to_owned(), to_value([3, 4])?);
        substitutions.insert("c".to_owned(), to_value([4, 5, 6])?);
        assert!(get_substitutions_cartesian(
            &substitutions,
            Some(vec![
                vec!["a".to_owned(), "c".to_owned()],
                vec!["b".to_owned()]
            ])
        )
        .is_err());
        Ok(())
    }
}
