use anyhow::Result;

use crate::args::PostFn;
use crate::sim_params::SimParams;
use crate::sim_set::SimSet;
use crate::util::get_files;
use snapshot::Snapshot;
use std::path::PathBuf;

pub mod post_expansion;
pub mod read_hdf5;
pub mod snapshot;

pub fn postprocess_sim_set(sim_set: &SimSet, function: PostFn) -> Result<()> {
    for sim in sim_set.iter() {
        let (post_function, plot_function) = match function {
            PostFn::Expansion => (post_expansion::post, post_expansion::plot),
        };
        post_function(sim)?;
        plot_function(sim)?;
    }
    Ok(())
}

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
