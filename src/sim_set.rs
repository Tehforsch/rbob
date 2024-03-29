use std::collections::HashMap;
use std::collections::HashSet;
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
use crate::job_cascade::get_substitutions_cascade;
use crate::job_cascade::CascadeArgs;
use crate::param_value::ParamValue;
use crate::sim_params::SimParams;
use crate::sim_params::SimParamsKind;
use crate::util::get_common_path;
use crate::util::get_folders;

#[derive(Serialize, Deserialize)]
enum CartesianType {
    NoCartesian,
    All,
    Grouped(Vec<Vec<String>>),
    Cascade(CascadeArgs),
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
            folder.as_ref(),
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

    pub fn join(sets: impl Iterator<Item = SimSet>) -> SimSet {
        sets.flat_map(|set| set.into_iter().map(|(_, s)| s))
            .enumerate()
            .collect()
    }

    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a SimParams> + 'a> {
        Box::new(self.simulations.iter().map(|(_, s)| s))
    }

    pub fn enumerate(&self) -> Iter<(usize, SimParams)> {
        self.simulations.iter()
    }

    pub fn sort_by_key<K>(&mut self, mut f: impl FnMut(&SimParams) -> K)
    where
        K: Ord,
    {
        self.simulations.sort_by_key(|(_, sim)| f(sim))
    }

    pub fn varies(&self, param: &str) -> bool {
        let first_sim = self.simulations.first().map(|(_, s)| s);
        match first_sim {
            None => false,
            Some(sim) => self.iter().any(|s| s.get(param) != sim.get(param)),
        }
    }

    pub fn get_folder(&self) -> Result<Utf8PathBuf> {
        let parent_folders = get_common_path(self.iter().map(|sim| sim.folder.parent().unwrap()));
        parent_folders.ok_or_else(|| anyhow!("No simulation in sim set, cannot determine folder."))
    }

    pub fn len(&self) -> usize {
        self.simulations.len()
    }

    pub fn is_empty(&self) -> bool {
        self.simulations.is_empty()
    }

    pub fn quotient(&self, param: &str) -> Vec<SimSet> {
        let mut possible_values = HashSet::new();
        for sim in self.iter() {
            possible_values.insert(sim[param].clone());
        }
        let mut sub_sim_sets = vec![];
        for possible_value in possible_values.iter() {
            sub_sim_sets.push(SimSet {
                simulations: self
                    .iter()
                    .filter(|sim| sim[param] == *possible_value)
                    .cloned()
                    .enumerate()
                    .collect(),
            });
        }
        sub_sim_sets.sort_by_key(|sim_set| sim_set.iter().next().unwrap()[param].clone());
        sub_sim_sets
    }

    pub fn quotients<'a>(&'a self, params: &[&str]) -> Vec<SimSet> {
        let next_param = params.first();
        match next_param {
            Some(param) => self
                .quotient(param)
                .into_iter()
                .flat_map(|sim_set| sim_set.quotients(&params[1..]))
                .collect(),
            None => vec![self.clone()],
        }
    }
}

fn get_sim_params(
    folder: &Utf8Path,
    config: &SimSetConfig,
    base_sim_params: SimParams,
) -> Result<Vec<(usize, SimParams)>> {
    let substitutions = match &config.cartesian_type {
        CartesianType::NoCartesian => get_substitutions_normal(&config.substitutions),
        CartesianType::All => get_substitutions_cartesian(&config.substitutions, None),
        CartesianType::Grouped(l) => {
            get_substitutions_cartesian(&config.substitutions, Some(l.to_vec()))
        }
        CartesianType::Cascade(l) => {
            get_substitutions_cascade(&base_sim_params, folder, &config.substitutions, l)
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

pub fn get_substitutions_cartesian(
    substitutions: &HashMap<String, Value>,
    grouped_params: Option<Vec<Vec<String>>>,
) -> Result<Vec<HashMap<String, ParamValue>>> {
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
    use serde_yaml::to_value;

    use super::*;
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
