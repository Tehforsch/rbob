use std::{fs, path::Path};

use anyhow::{Context, Result};

use crate::config;
use crate::sim_params::SimParams;
use crate::sim_set::SimSet;
use crate::util::copy_recursive;

pub fn copy_sim_set<U: AsRef<Path>>(
    sim_set: &SimSet,
    input_folder: U,
    output_folder: U,
) -> Result<SimSet> {
    fs::create_dir(output_folder.as_ref()).with_context(|| "When creating the output folder")?;
    let output_sim_set: Result<SimSet> = sim_set
        .enumerate()
        .map(|(i, sim)| -> Result<(usize, SimParams)> {
            println!("Copying files for sim {}:", i);
            let sim_output_folder = output_folder.as_ref().join(String::from(i.to_string()));
            Ok((
                *i,
                copy_sim(sim, &input_folder.as_ref(), &sim_output_folder)
                    .with_context(|| format!("When copying simulation {}", i))?,
            ))
        })
        .collect();
    output_sim_set
}

fn copy_sim(sim: &SimParams, input_folder: &Path, sim_output_folder: &Path) -> Result<SimParams> {
    copy_recursive(input_folder, sim_output_folder)?;
    sim.write_param_file(&sim_output_folder.join(config::DEFAULT_PARAM_FILE_NAME))?;
    sim.write_config_file(&sim_output_folder.join(config::DEFAULT_CONFIG_FILE_NAME))?;
    // This is not the most efficient thing ever but it should be completely fine since this is not done very often, nor is the hashmap very large.
    let mut new_sim_params = sim.clone();
    new_sim_params.folder = sim_output_folder.to_owned();
    Ok(new_sim_params)
}
