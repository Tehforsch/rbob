use super::snapshot::Snapshot;
use crate::sim_params::SimParams;
use crate::util::get_files;
use anyhow::Result;
use std::path::PathBuf;
use uom::si::f64::*;

struct ExpansionData {
    time: Vec<Time>,
    radius: Vec<Length>,
}

pub fn post(sim: &SimParams) -> Result<()> {
    for snap in get_snapshots(sim)? {
        println!("{:?}", snap?);
    }
    Ok(())
}

pub fn plot(sim: &SimParams) {}

pub fn get_snapshots(sim: &SimParams) -> Result<Box<dyn Iterator<Item = Result<Snapshot>>>> {
    Ok(Box::new(
        get_snapshot_files(sim)?.map(|snap_file| Snapshot::from_file(&snap_file)),
    ))
}

pub fn get_snapshot_files(sim: &SimParams) -> Result<Box<dyn Iterator<Item = PathBuf>>> {
    Ok(Box::new(
        get_files(&sim.output_folder())?.into_iter().filter(|f| {
            f.extension()
                .map(|ext| ext.to_str().unwrap() == "hdf5")
                .unwrap_or(false)
        }),
    ))
}
