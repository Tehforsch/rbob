use std::{fs, path::Path};

use anyhow::{Context, Result};

use crate::config;
use crate::sim_params::SimParams;
use crate::simulation_set::SimSet;
use crate::util::copy_recursive;

pub fn copy_sim_set<U: AsRef<Path>>(sim_set: SimSet, input_folder: U, run_folder: U) -> Result<()> {
    fs::create_dir(run_folder.as_ref()).with_context(|| "When creating the run folder")?;
    for (i, sim) in sim_set.iter().enumerate() {
        println!("Copying files for sim {}:", i);
        let sim_run_folder = run_folder.as_ref().join(String::from(i.to_string()));
        copy_sim(sim, &input_folder.as_ref(), &sim_run_folder)
            .with_context(|| format!("When copying simulation {}", i))?;
    }
    Ok(())
}

fn copy_sim(sim: &SimParams, input_folder: &Path, sim_run_folder: &Path) -> Result<()> {
    copy_recursive(input_folder, sim_run_folder)?;
    sim.write_param_file(&sim_run_folder.join(config::DEFAULT_PARAM_FILE_NAME))?;
    sim.write_config_file(&sim_run_folder.join(config::DEFAULT_CONFIG_FILE_NAME))?;
    Ok(())
}
