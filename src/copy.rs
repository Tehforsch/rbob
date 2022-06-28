use std::fs;

use anyhow::Context;
use anyhow::Result;
use camino::Utf8Path;

use crate::config;
use crate::sim_params::SimParams;
use crate::sim_set::SimSet;

pub fn copy_sim_set<U: AsRef<Utf8Path>>(
    sim_set: &SimSet,
    input_folder: U,
    output_folder: U,
    delete: bool,
) -> Result<SimSet> {
    let output_folder = output_folder.as_ref();
    if delete && output_folder.is_dir() {
        fs::remove_dir_all(output_folder)
            .with_context(|| "When deleting the previous output folder")?;
    }
    fs::create_dir(output_folder).with_context(|| "When creating the output folder")?;
    let output_sim_set: Result<SimSet> = sim_set
        .enumerate()
        .map(|(i, sim)| -> Result<(usize, SimParams)> {
            println!("Copying files for sim {}:", i);
            let sim_output_folder = output_folder.join(i.to_string());
            Ok((
                *i,
                copy_sim(sim, input_folder.as_ref(), &sim_output_folder)
                    .with_context(|| format!("When copying simulation {}", i))?,
            ))
        })
        .collect();
    output_sim_set
}

fn copy_sim(
    sim: &SimParams,
    input_folder: &Utf8Path,
    sim_output_folder: &Utf8Path,
) -> Result<SimParams> {
    assert_eq!(input_folder, sim.folder);
    fs::create_dir_all(sim_output_folder)?;
    sim.write_param_file(&sim_output_folder.join(config::DEFAULT_PARAM_FILE_NAME))?;
    sim.write_config_file(&sim_output_folder.join(config::DEFAULT_CONFIG_FILE_NAME))?;
    sim.write_job_file(&sim_output_folder.join(config::DEFAULT_JOB_FILE_NAME))?;
    sim.copy_ics(sim_output_folder)?;
    sim.copy_test_sources_file_if_exists(input_folder, sim_output_folder)?;
    sim.copy_treecool_file_if_exists(input_folder, sim_output_folder)?;
    sim.write_bob_param_file(&sim_output_folder.join(config::DEFAULT_BOB_PARAM_FILE_NAME))?;
    // This is not the most efficient thing ever but it should be completely fine since this is not done very often, nor is the hashmap very large.
    let mut new_sim_params = sim.clone();
    new_sim_params.folder = sim_output_folder.to_owned();
    Ok(new_sim_params)
}
