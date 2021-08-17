use anyhow::Result;

use crate::sim_params::SimParams;

use super::{get_snapshots, snapshot::Snapshot};

pub fn simulate_run_time(sim: &SimParams) -> Result<f64> {
    dbg!(sim);
    let snap = get_last_snapshot(sim)?;
    let grid_file = run_conversion_script(&snap);
    todo!()
}

fn get_last_snapshot(sim: &SimParams) -> Result<Snapshot<'_>> {
    get_snapshots(sim)?.last().unwrap()
}

fn run_conversion_script(snap: &Snapshot) {
    
}
