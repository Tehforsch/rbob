use crate::config;
use crate::sim_params::SimParams;
use crate::sim_set::SimSet;
use anyhow::Result;

pub fn run_sim_set(sim_set: SimSet) -> Result<()> {
    for (i, sim) in sim_set.iter().enumerate() {
        println!("Running sim {}", i);
        run_sim(sim)?;
    }
    Ok(())
}

fn run_sim(sim: &SimParams) -> Result<()> {
    let job_file_name = sim.folder.join(config::DEFAULT_JOB_FILE_NAME);
    sim.write_job_file(&job_file_name)?;
    Ok(())
}
