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
use crate::sim_set::get_substitutions_cartesian;

#[derive(Serialize, Deserialize)]
pub struct CascadeArgs {
    files: Vec<String>,
    final_time: f64,
}

impl CascadeArgs {
    pub fn get_times(&self, folder: &Utf8Path) -> Vec<f64> {
        let mut times = vec![];
        for file in self.files.iter() {
            times.push(get_time_for_snapshot(&folder.join(file)).unwrap());
        }
        times.push(self.final_time);
        times
    }
}

fn get_time_for_snapshot(path: &Utf8Path) -> Result<f64> {
    let h5file = hdf5::File::open_rw(path)?;
    read_attr_f64(&h5file, "Time")
}

fn strip_ending(s: &str) -> String {
    Utf8Path::new(s).with_extension("").into()
}

pub fn get_substitutions_cascade(
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
        println!("sim {}: {} to {}", i, time_begin, time_end);
        let file = &cascade.files[i];
        insert_substitution(i, "InitCondFile", ParamValue::Str(strip_ending(file)));
        insert_substitution(i, "TimeBegin", ParamValue::new_float(*time_begin));
        insert_substitution(i, "TimeMax", ParamValue::new_float(*time_end));
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
                "{bob_path} copy-abundances ../{num}/ . {rewritten_name}; rm {normal_name}; mv {rewritten_name} {normal_name}",
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
