use std::collections::HashMap;
use std::fs;

use anyhow::anyhow;
use anyhow::Result;
use camino::Utf8Path;
use camino::Utf8PathBuf;
use pathdiff::diff_utf8_paths;
use serde::Deserialize;
use serde::Serialize;
use serde_yaml::Value;

use crate::sim_params::SimParams;
use crate::sim_set::get_substitutions_cartesian;

struct Cosmology {
    omega_lambda: f64,
    omega_0: f64,
    hubble_param: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Time {
    Time(f64),
    Redshift(f64),
    ScaleFactor(f64),
}

impl Time {
    fn time_until_seconds(&self, other: &Self, cosmology: &Option<Cosmology>) -> f64 {
        match self {
            Time::Time(time1) => {
                if let Self::Time(time2) = other {
                    time2 - time1
                } else {
                    unimplemented!()
                }
            }
            Time::Redshift(redshift) => Time::ScaleFactor(redshift_to_scalefactor(*redshift))
                .time_until_seconds(other, cosmology),
            Time::ScaleFactor(a1) => {
                let a2 = match other {
                    Time::Time(_) => unimplemented!(),
                    Time::Redshift(redshift) => redshift_to_scalefactor(*redshift),
                    Time::ScaleFactor(a2) => *a2,
                };
                get_time_between_scale_factors_in_s(*a1, a2, cosmology.as_ref().unwrap())
            }
        }
    }
}

fn redshift_to_scalefactor(redshift: f64) -> f64 {
    1.0 / (1.0 + redshift)
}

fn get_time_between_scale_factors_in_s(a1: f64, a2: f64, cosmology: &Cosmology) -> f64 {
    let hubble = 3.2407789e-18;
    let a_to_t = |val: f64| {
        let factor1 = 2.0 / (3.0 * cosmology.omega_lambda.sqrt());
        let term1 = (cosmology.omega_lambda / cosmology.omega_0).sqrt() * val.powf(1.5);
        let term2 = (1.0 + cosmology.omega_lambda / cosmology.omega_0 * val.powi(3)).sqrt();
        let factor2 = (term1 + term2).ln();
        factor1 * factor2
    };
    let t1 = a_to_t(a1);
    let t2 = a_to_t(a2);
    let diff_h = t2 - t1;
    let diff_secs = diff_h / (cosmology.hubble_param * hubble);
    diff_secs
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CascadeArgs {
    snapshots: Vec<Utf8PathBuf>,
    final_time: Time,
    original_simulation_comoving: bool,
}

fn get_files(path: &Utf8Path) -> Vec<Utf8PathBuf> {
    if path.is_file() {
        vec![path.to_owned()]
    } else {
        fs::read_dir(path)
            .unwrap_or_else(|e| {
                panic!("Error: {e} while trying to read path {path:?} as directory")
            })
            .flat_map(|entry| {
                let entry = entry.unwrap();
                let path = entry.path();
                let ext = path.extension()?.to_str()?;
                if path.is_file() && ext == "hdf5" {
                    Some(Utf8PathBuf::from_path_buf(entry.path()).unwrap())
                } else {
                    None
                }
            })
            .collect()
    }
}

impl CascadeArgs {
    pub fn get_times(&self, base_folder: &Utf8Path) -> Vec<Time> {
        let mut times = vec![];
        for (i, _) in self.snapshots.iter().enumerate() {
            let files = self.get_files_for_snapshot(base_folder, i);
            let file = &files[0]; // Any representative will do, the parameters have to all be the same
            times.push(self.get_time_for_snapshot(file).unwrap());
        }
        times.push(self.final_time);
        times
    }

    fn get_time_for_snapshot(&self, path: &Utf8Path) -> Result<Time> {
        let value = read_header_attr(path, "Time")?;
        if self.original_simulation_comoving {
            Ok(Time::ScaleFactor(value))
        } else {
            Ok(Time::Time(value))
        }
    }

    fn get_files_for_snapshot(&self, base_folder: &Utf8Path, i: usize) -> Vec<Utf8PathBuf> {
        get_files(&base_folder.join(&self.snapshots[i]))
    }
}

fn read_header_attr(file: &Utf8Path, attr_name: &str) -> Result<f64> {
    let h5file = hdf5::File::open(file)?;
    Ok(h5file.group("Header")?.attr(attr_name)?.read_scalar()?)
}

fn get_cosmology(file: &Utf8Path) -> Option<Cosmology> {
    let omega_lambda = read_header_attr(file, "OmegaLambda").ok()?;
    let omega_0 = read_header_attr(file, "Omega0").ok()?;
    let hubble_param = read_header_attr(file, "HubbleParam").ok()?;
    Some(Cosmology {
        omega_lambda,
        omega_0,
        hubble_param,
    })
}

pub fn get_substitutions_cascade(
    base_sim_params: &SimParams,
    substitutions: &HashMap<String, Value>,
    cascade: &CascadeArgs,
) -> Result<Vec<HashMap<String, Value>>> {
    let base_folder = &base_sim_params.folder;
    let mut other_substitutions =
        get_non_cascade_substitutions(substitutions, cascade.snapshots.len())?;
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
    let times = cascade.get_times(&base_folder);
    let cosmology = get_cosmology(&cascade.get_files_for_snapshot(&base_folder, 0)[0]);
    assert_eq!(times.len(), cascade.snapshots.len() + 1);
    for (i, (time_begin, time_end)) in times.iter().zip(times[1..].iter()).enumerate() {
        let files = cascade.get_files_for_snapshot(&base_folder, i);
        let time_diff_seconds = time_begin.time_until_seconds(time_end, &cosmology);
        let time_diff_kiloyear = time_diff_seconds / (31560000.0 * 1000.0);
        println!(
            "sim {}: {:?} to {:?} ({:.5} kyr)",
            i, time_begin, time_end, time_diff_kiloyear
        );
        insert_substitution(
            i,
            "input/paths",
            Value::Sequence(
                files
                    .iter()
                    .map(|f| {
                        let f = diff_utf8_paths(f, &base_folder).unwrap();
                        f.as_str().into()
                    })
                    .collect(),
            ),
        );
        insert_substitution(
            i,
            "simulation/final_time",
            format!("{} kyr", time_diff_kiloyear).into(),
        );
        if cascade.original_simulation_comoving {
            let a = read_header_attr(&files[0], "Time")?;
            let h = read_header_attr(&files[0], "HubbleParam")?;
            insert_substitution(i, "cosmology/a", a.into());
            insert_substitution(i, "cosmology/h", h.into());
        }
    }
    Ok(other_substitutions)
}

fn get_non_cascade_substitutions(
    substitutions: &HashMap<String, Value>,
    num_cascade_files: usize,
) -> Result<Vec<HashMap<String, Value>>> {
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
