use std::collections::HashMap;

use anyhow::anyhow;
use anyhow::Result;
use camino::Utf8Path;
use serde::Deserialize;
use serde::Serialize;
use serde_yaml::Value;

use crate::config::CASCADE_IDENTIFIER;
use crate::config::CONFIG_FILE;
use crate::param_value::ParamValue;
use crate::postprocess::read_hdf5::read_attr_f64;
use crate::sim_params::SimParams;
use crate::sim_set::get_substitutions_cartesian;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ArepoTime {
    Time(f64),
    ScaleFactor(f64),
}

impl ArepoTime {
    /// Returns the time in code units between `self` and `other`.
    /// Assumes that all times are given in the same code units.
    fn time_until(&self, other: &ArepoTime, sim: &SimParams) -> f64 {
        match self {
            Self::Time(t1) => match other {
                ArepoTime::Time(t2) => t2 - t1,
                ArepoTime::ScaleFactor(_) => unreachable!(),
            },
            Self::ScaleFactor(a1) => match other {
                ArepoTime::ScaleFactor(a2) => get_time_between_scale_factors(*a1, *a2, sim),
                ArepoTime::Time(_) => unreachable!(),
            },
        }
    }
}

fn get_time_between_scale_factors(a1: f64, a2: f64, sim: &SimParams) -> f64 {
    let omega_lambda = sim["OmegaLambda"].unwrap_f64();
    let omega_0 = sim["Omega0"].unwrap_f64();
    let hubble_param = sim["HubbleParam"].unwrap_f64();
    let hubble = 3.2407789e-18;
    let a_to_t = |val: f64| {
        let factor1 = 2.0 / (3.0 * omega_lambda.sqrt());
        let term1 = (omega_lambda / omega_0).sqrt() * val.powf(1.5);
        let term2 = (1.0 + omega_lambda / omega_0 * val.powi(3)).sqrt();
        let factor2 = (term1 + term2).ln();
        factor1 * factor2
    };
    let t1 = a_to_t(a1);
    let t2 = a_to_t(a2);
    let diff_h = t2 - t1;
    let diff_secs = diff_h / (hubble_param * hubble);
    let secs_in_code_units =
        sim["UnitLength_in_cm"].unwrap_f64() / sim["UnitVelocity_in_cm_per_s"].unwrap_f64();
    diff_secs / secs_in_code_units
}

#[derive(Serialize, Deserialize)]
pub struct CascadeArgs {
    files: Vec<String>,
    final_time: ArepoTime,
    original_simulation_comoving: bool,
}

impl CascadeArgs {
    pub fn get_times(&self, folder: &Utf8Path) -> Vec<ArepoTime> {
        let mut times = vec![];
        for file in self.files.iter() {
            times.push(self.get_time_for_snapshot(&folder.join(file)).unwrap());
        }
        times.push(self.final_time);
        times
    }

    fn get_time_for_snapshot(&self, path: &Utf8Path) -> Result<ArepoTime> {
        let h5file = hdf5::File::open_rw(path)?;
        let val = read_attr_f64(&h5file, "Time")?;
        if self.original_simulation_comoving {
            Ok(ArepoTime::ScaleFactor(val))
        } else {
            Ok(ArepoTime::Time(val))
        }
    }
}

fn strip_ending(s: &str) -> String {
    Utf8Path::new(s).with_extension("").into()
}

pub fn get_substitutions_cascade(
    base_sim_params: &SimParams,
    folder: &Utf8Path,
    substitutions: &HashMap<String, Value>,
    cascade: &CascadeArgs,
) -> Result<Vec<HashMap<String, ParamValue>>> {
    let times = cascade.get_times(folder);
    let mut other_substitutions =
        get_non_cascade_substitutions(substitutions, cascade.files.len())?;
    let mut insert_substitution = |i, name: &str, value| {
        let result: &mut HashMap<_, _> = &mut other_substitutions[i];
        match result.insert(name.into(), value) {
            Some(_) => panic!(
                "Parameter {} would be overwritten by cascade settings",
                name
            ),
            None => {}
        }
    };
    assert_eq!(times.len(), cascade.files.len() + 1);
    for (i, (time_begin, time_end)) in times.iter().zip(times[1..].iter()).enumerate() {
        let file = &cascade.files[i];
        let time_diff = time_begin.time_until(time_end, base_sim_params);
        println!(
            "sim {}: {:?} to {:?} ({:.5})",
            i, time_begin, time_end, time_diff
        );
        insert_substitution(i, "InitCondFile", ParamValue::Str(strip_ending(file)));
        insert_substitution(i, "TimeBegin", ParamValue::new_float(0.0));
        insert_substitution(i, "TimeMax", ParamValue::new_float(time_diff));
        insert_substitution(
            i,
            "MaxSizeTimestep",
            ParamValue::new_float(time_diff * 1e-8),
        );
        insert_substitution(
            i,
            "MinSizeTimestep",
            ParamValue::new_float(time_diff * 1e-9),
        );
        insert_substitution(i, CASCADE_IDENTIFIER, ParamValue::Bool(true));
        let rewrite_snapshot_command = get_command_to_rewrite_snapshot(i, file);
        insert_substitution(
            i,
            "additionalCommands",
            ParamValue::Str(rewrite_snapshot_command),
        );
    }
    Ok(other_substitutions)
}

fn get_command_to_rewrite_snapshot(num: usize, snap_name: &str) -> String {
    match num {
        0 => "".into(),
        num => {
            format!(
                "{bob_path} copy-abundances ../{num}/ . {rewritten_name}; rm {normal_name}; mv {rewritten_name} {normal_name}; ",
                bob_path = CONFIG_FILE.bob_path,
                num = num - 1,
                rewritten_name = get_rewritten_snapshot_name(snap_name),
                normal_name = snap_name,
            )
        }
    }
}

fn get_rewritten_snapshot_name(original_snap_name: &str) -> String {
    format!("{}_rewritten.hdf5", strip_ending(original_snap_name))
}

fn get_non_cascade_substitutions(
    substitutions: &HashMap<String, Value>,
    num_cascade_files: usize,
) -> Result<Vec<HashMap<String, ParamValue>>> {
    let mut sims = get_substitutions_cartesian(substitutions, None)?;
    if sims.len() == 1 {
        sims = repeat(&sims[0], num_cascade_files);
        Ok(sims)
    } else if sims.len() != num_cascade_files {
        Err(anyhow!(
            "Number of substitution sims and number of cascade files do not match: {} vs {}",
            sims.len(),
            num_cascade_files,
        ))
    } else {
        Ok(sims)
    }
}

fn repeat<T: Clone>(x: &T, num: usize) -> Vec<T> {
    (0..num).map(|_| x.clone()).collect()
}
