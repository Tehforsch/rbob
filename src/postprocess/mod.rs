use anyhow::{Context, Result};
use csv::WriterBuilder;



use crate::sim_set::SimSet;
use crate::util::get_files;
use crate::{config_file::ConfigFile, sim_params::SimParams};
use post_fn_name::PostFnName;
use snapshot::Snapshot;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader},
    path::{Path, PathBuf},
};

use serde::{de::DeserializeOwned, Serialize};

use self::{plot::PlotInfo};

pub mod axis;
pub mod plot;
pub mod plot_template;
pub mod post_expansion;
pub mod post_fn_name;
pub mod post_scaling;
pub mod post_slice;
pub mod read_hdf5;
pub mod snapshot;

pub trait SnapPostFn {
    type Output: DeserializeOwned + Serialize + core::fmt::Debug;
    fn post(&self, sim: &SimParams, snap: &Snapshot) -> Result<Vec<Vec<Self::Output>>>;
    // fn plot(&self, result: &Vec<Self::Output>, plot_info: &PlotInfo) -> Result<()>;

    fn run_on_sim_snap(
        &self,
        config_file: &ConfigFile,
        sim: &SimParams,
        snap: &Snapshot,
        plot_info: &PlotInfo,
    ) -> Result<()> {
        let res = self.post(sim, snap)?;
        let filenames = write_results(&plot_info.data_folder, &res)?;
        plot::run_plot(config_file, &plot_info, &filenames)
    }
}

pub trait SetPostFn {
    type Output: DeserializeOwned + Serialize + core::fmt::Debug;
    fn post(&self, sim: &SimSet) -> Result<Vec<Vec<Self::Output>>>;
    // fn plot(&self, result: &Vec<Self::Output>, plot_info: &PlotInfo) -> Result<()>;

    fn run_on_sim(
        &self,
        config_file: &ConfigFile,
        sim_set: &SimSet,
        plot_info: &PlotInfo,
    ) -> Result<()> {
        let res = self.post(sim_set)?;
        let filenames = write_results(&plot_info.data_folder, &res)?;
        plot::run_plot(config_file, &plot_info, &filenames)
    }
}

pub fn write_results(
    data_folder: &Path,
    results: &Vec<Vec<impl Serialize>>,
) -> Result<Vec<PathBuf>> {
    results
        .iter()
        .enumerate()
        .map(|(i, _res)| {
            let file = data_folder.join(i.to_string());
            let mut wtr = WriterBuilder::new()
                .has_headers(false)
                .delimiter(b' ')
                .terminator(csv::Terminator::CRLF)
                .from_path(&file)?;
            for line in results.iter() {
                for record in line.iter() {
                    wtr.serialize(record)?;
                }
            }
            wtr.flush()?;
            Ok(file.to_owned())
        })
        .collect()
}

pub fn postprocess_sim_set(
    config_file: &ConfigFile,
    sim_set: &SimSet,
    function: PostFnName,
) -> Result<()> {
    let sim_set_folder = sim_set.get_folder()?;
    for sim in sim_set.iter() {
        for mb_snap in get_snapshots(sim)? {
            let snap = mb_snap?;
            let plot_info = PlotInfo::new(&sim_set_folder, Some(sim), &function, Some(&snap));
            plot_info.create_folders_if_nonexistent()?;
            match function {
                PostFnName::Slice(ref l) => {
                    l.run_on_sim_snap(config_file, sim, &snap, &plot_info)?
                }
                _ => {}
            }
        }
    }
    let plot_info = PlotInfo::new(&sim_set_folder, None, &function, None);
    plot_info.create_folders_if_nonexistent()?;
    match function {
        PostFnName::Scaling(ref l) => l.run_on_sim(config_file, sim_set, &plot_info)?,
        _ => {}
    }
    Ok(())
}

pub fn get_snapshots<'a>(
    sim: &'a SimParams,
) -> Result<Box<dyn Iterator<Item = Result<Snapshot<'a>>> + 'a>> {
    Ok(Box::new(get_snapshot_files(sim)?.map(move |snap_file| {
        Snapshot::from_file(sim, &snap_file)
    })))
}

pub fn get_snapshot_files(sim: &SimParams) -> Result<Box<dyn Iterator<Item = PathBuf>>> {
    Ok(Box::new(
        get_files(&sim.output_folder())
            .context(format!(
                "No output folder in simulation folder: {:?} (looking in {:?})",
                sim.folder,
                sim.output_folder(),
            ))?
            .into_iter()
            .filter(|f| {
                f.extension()
                    .map(|ext| ext.to_str().unwrap() == "hdf5")
                    .unwrap_or(false)
            }),
    ))
}
