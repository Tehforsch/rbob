use anyhow::{Context, Result};
use csv::WriterBuilder;
use ndarray_csv::{Array2Reader, Array2Writer};

use crate::sim_set::SimSet;
use crate::util::get_files;
use crate::{config_file::ConfigFile, sim_params::SimParams};
use post_fn_name::PostFnName;
use snapshot::Snapshot;
use std::path::{Path, PathBuf};

use serde::{de::DeserializeOwned, Serialize};

use self::{data_plot_info::DataPlotInfo, plot::PlotInfo, post_fn::PostFn};

pub mod axis;
pub mod data_plot_info;
pub mod plot;
pub mod plot_template;
pub mod post_expansion;
pub mod post_fn;
pub mod post_fn_name;
pub mod post_scaling;
pub mod post_slice;
pub mod read_hdf5;
pub mod snapshot;

pub fn postprocess_sim_set(
    config_file: &ConfigFile,
    sim_set: &SimSet,
    function_name: PostFnName,
) -> Result<()> {
    let data_plot_info_list = function_name
        .get_function()
        .run_post(config_file, sim_set)?;
    for data_plot_info in data_plot_info_list.iter() {
        let filenames = write_results(&data_plot_info)?;
        plot::run_plot(config_file, &data_plot_info.info, &filenames)?;
    }
    Ok(())
}

pub fn write_results(data_plot_info: &DataPlotInfo) -> Result<Vec<PathBuf>> {
    let data_folder = &data_plot_info.info.data_folder;
    data_plot_info
        .data
        .iter()
        .enumerate()
        .map(|(i, res)| {
            let file = data_folder.join(i.to_string());
            let mut wtr = WriterBuilder::new()
                .has_headers(false)
                .delimiter(b' ')
                .from_path(&file)?;
            wtr.serialize_array2(res)?;
            wtr.flush()?;
            Ok(file.to_owned())
        })
        .collect()
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
