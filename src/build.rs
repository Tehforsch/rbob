use anyhow::Result;

use crate::sim_params::SimParams;
use crate::sim_set::SimSet;

pub fn build_sim_set(sim_set: SimSet) -> Result<()> {
    for (i, sim) in sim_set.enumerate() {
        println!("Building sim {}", i);
        build_sim(sim)?;
    }
    Ok(())
}

fn build_sim(sim: &SimParams) -> Result<()> {
    todo!()
}
