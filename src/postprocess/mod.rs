use anyhow::{Context, Result};
use csv::WriterBuilder;
use ndarray_csv::Array2Writer;

use crate::util::get_files;
use crate::{config_file::ConfigFile, sim_params::SimParams};
use crate::{sim_set::SimSet, util::get_shell_command_output};

use camino::Utf8PathBuf;
use snapshot::Snapshot;

use self::{data_plot_info::DataPlotInfo, postprocess_args::PostprocessArgs};

pub mod axis;
pub mod data_plot_info;
pub mod kd_tree;
pub mod plot;
pub mod plot_info;
pub mod plot_template;
pub mod post_compare;
pub mod post_expansion;
pub mod post_fn;
pub mod post_fn_name;
pub mod post_scaling;
pub mod post_slice;
pub mod postprocess_args;
pub mod read_hdf5;
pub mod snapshot;

pub fn postprocess_sim_set(
    config_file: &ConfigFile,
    sim_set: &SimSet,
    args: &PostprocessArgs,
) -> Result<()> {
    let function = args.function.get_function();
    let data_plot_info_list = function.run_post(sim_set)?;
    for data_plot_info in data_plot_info_list.iter() {
        data_plot_info.info.create_folders_if_nonexistent()?;
        let filenames = write_results(&data_plot_info)?;
        let image_file = plot::run_plot(config_file, &data_plot_info.info, &filenames)?;
        if args.show {
            show_image(&image_file);
        }
    }
    Ok(())
}

pub fn write_results(data_plot_info: &DataPlotInfo) -> Result<Vec<Utf8PathBuf>> {
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

pub fn get_snapshot_files(sim: &SimParams) -> Result<Box<dyn Iterator<Item = Utf8PathBuf>>> {
    let mut files = get_files(&sim.output_folder()).context(format!(
        "No output folder in simulation folder: {:?} (looking in {:?})",
        sim.folder,
        sim.output_folder(),
    ))?;
    files.sort_by_key(|snap_file| snap_file.file_name().unwrap().to_owned());
    Ok(Box::new(files.into_iter().filter(|f| {
        f.extension().map(|ext| ext == "hdf5").unwrap_or(false)
    })))
}

pub fn show_image(path: &str) {
    get_shell_command_output("viewnior", &[path], None, false);
}
