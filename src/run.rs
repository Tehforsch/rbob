use crate::sim_params::SimParams;
use crate::sim_set::SimSet;
use crate::{config, util::get_shell_command_output};
use anyhow::{anyhow, Result};

pub fn run_sim_set(sim_set: &SimSet, verbose: bool) -> Result<()> {
    for (i, sim) in sim_set.iter().enumerate() {
        println!("Running sim {}", i);
        run_sim(sim, verbose)?;
    }
    Ok(())
}

fn run_sim(sim: &SimParams, verbose: bool) -> Result<()> {
    run_job_file(
        sim,
        &sim.folder.join(config::DEFAULT_JOB_FILE_NAME),
        verbose,
    )?;
    Ok(())
}

pub fn run_job_file(sim: &SimParams, job_file_name: &camino::Utf8PathBuf, verbose: bool) -> Result<()> {
    let args: &[&str] = &[job_file_name.file_name().unwrap()];
    let out = get_shell_command_output(config::RUN_COMMAND, args, Some(&sim.folder), verbose);
    match out.success {
        false => {
            if !verbose {
                println!("{}", out.stdout);
                println!("{}", out.stderr);
            }
            Err(anyhow!("Running the job file failed."))
        }
        true => Ok(()),
    }
}
