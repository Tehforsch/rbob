use super::get_snapshots;
// use super::snapshot::Snapshot;
use crate::sim_params::SimParams;
use anyhow::Result;
use uom::si::{f64::Length, length::meter};

// struct ExpansionData {
//     time: Array<Time>,
//     radius: Vec<Length>,
// }

pub fn post(sim: &SimParams) -> Result<()> {
    for snap in get_snapshots(sim)? {
        let coords = snap?.coordinates();
        let unit: Length = Length::new::<meter>(109000.0);
        println!("Some multiple: {}", coords? * unit);
    }
    Ok(())
}

pub fn plot(sim: &SimParams) -> Result<()> {
    Ok(())
}
