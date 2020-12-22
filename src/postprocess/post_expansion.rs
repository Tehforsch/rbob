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
    for mb_snap in get_snapshots(sim)? {
        let snap = mb_snap?;
        let coords = snap.coordinates()?;
        let dens = snap.density()?;
        let abundance = snap.chemical_abundances()?;
        println!("{}\n{}\n{}", coords, dens, abundance);
    }
    Ok(())
}

pub fn plot(sim: &SimParams) -> Result<()> {
    Ok(())
}
