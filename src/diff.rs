use anyhow::Result;
use camino::Utf8Path;

use crate::param_value::ParamValue;
use crate::sim_params::SimParams;
use crate::sim_params::SimParamsKind;

enum ParamDiff {
    Same,
    Diff(String, String, String),
}

impl ParamDiff {
    pub fn show(&self) {
        match self {
            Self::Same => {}
            Self::Diff(n, s1, s2) => println!("< {}: {}\n> {}: {}", &n, &s1, &n, &s2),
        }
    }
}

pub fn show_sim_diff(folder1: &Utf8Path, folder2: &Utf8Path) -> Result<()> {
    let sim1 = SimParams::from_folder(folder1, SimParamsKind::Input)?;
    let sim2 = SimParams::from_folder(folder2, SimParamsKind::Input)?;
    let mut diffs: Vec<ParamDiff> = get_param_diffs(&sim1, &sim2).collect();
    diffs.sort_by_key(|diff| match diff {
        ParamDiff::Diff(name, _, _) => name.clone(),
        ParamDiff::Same => "".to_string(),
    });
    for diff in diffs.iter() {
        diff.show();
    }
    Ok(())
}

fn get_param_diffs<'a>(
    sim1: &'a SimParams,
    sim2: &'a SimParams,
) -> Box<dyn Iterator<Item = ParamDiff> + 'a> {
    let names: Vec<&String> = sim1.keys().collect();
    Box::new(
        sim1.iter()
            .map(move |(name, v1)| {
                let v2 = sim2.get(name);
                get_param_diff(name, Some(v1), v2)
            })
            .chain(
                sim2.iter()
                    .filter(move |(name, _)| !names.contains(name))
                    .map(move |(name, v2)| {
                        let v1 = sim1.get(name);
                        get_param_diff(name, v1, Some(v2))
                    }),
            ),
    )
}

fn get_param_diff(name: &str, v1: Option<&ParamValue>, v2: Option<&ParamValue>) -> ParamDiff {
    let s1 = diff_value_to_string(v1);
    let s2 = diff_value_to_string(v2);
    match v1.zip(v2) {
        None => ParamDiff::Diff(name.to_string(), s1, s2),
        Some((val1, val2)) => {
            if val1 == val2 {
                ParamDiff::Same
            } else {
                ParamDiff::Diff(name.to_string(), s1, s2)
            }
        }
    }
}

fn diff_value_to_string(v: Option<&ParamValue>) -> String {
    match v {
        None => "Not present".to_string(),
        Some(val) => val.to_string(),
    }
}
