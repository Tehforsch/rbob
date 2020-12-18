use crate::sim_set::SimSet;
use anyhow::Result;
use std::path::Path;

pub fn run_sim_set(sim_set: SimSet) -> Result<()> {
    for (i, sim) in sim_set.iter().enumerate() {
        println!("Running sim {}", i);
        run_sim(sim)?;
    }
    Ok(())
}

fn run_sim(sim: &crate::sim_params::SimParams) -> Result<()> {
    println!("heyo");
    Ok(())
}
