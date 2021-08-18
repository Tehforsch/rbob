use anyhow::{Context, Result};

use crate::sim_params::SimParams;
use crate::util::get_files;
use crate::{sim_set::SimSet, util::get_shell_command_output};
use crate::{source_file::SourceFile, util::write_file};

use camino::{Utf8Path, Utf8PathBuf};
use snapshot::Snapshot;

use self::{data_plot_info::DataPlotInfo, postprocess_args::PostprocessArgs};

pub mod axis;
pub mod calculations;
pub mod data_plot_info;
pub mod field_identifier;
pub mod plot;
pub mod plot_info;
pub mod plot_info_file_contents;
pub mod plot_params;
pub mod plot_template;
pub mod post_compare;
pub mod post_convergence;
pub mod post_expansion;
pub mod post_fn;
pub mod post_fn_name;
pub mod post_scaling;
pub mod post_shadowing;
pub mod post_slice;
pub mod postprocess_args;
pub mod read_hdf5;
pub mod replot_args;
pub mod snapshot;
pub mod swedule;

pub fn postprocess_sim_set(
    create_plot: bool,
    sim_set: &SimSet,
    args: &PostprocessArgs,
) -> Result<()> {
    let function = args.function.get_function();
    let data_plot_info_list = function.run_post(sim_set, args.plot_template.as_deref())?;
    for data_plot_info in data_plot_info_list.iter() {
        data_plot_info.info.create_folders_if_nonexistent()?;
        let filenames = write_results(&data_plot_info)?;
        let image_file = plot::run_plot(
            create_plot,
            &data_plot_info.info,
            &filenames,
            &data_plot_info.replacements,
        );
        if args.show {
            show_image(&image_file?);
        }
    }
    if args.showall && !data_plot_info_list.is_empty() {
        show_image_folder(&data_plot_info_list[0].info.pic_folder);
    }
    Ok(())
}

pub fn write_results(data_plot_info: &DataPlotInfo) -> Result<Vec<Utf8PathBuf>> {
    let data_folder = &data_plot_info.info.get_data_folder();
    data_plot_info
        .data
        .iter()
        .enumerate()
        .map(|(i, res)| {
            let file = data_folder.join(i.to_string());
            // I initially had ndarray_csv here but that actually produced faulty csv files so now
            // I use the worlds most primitive and inefficient way to write a csv file:
            let contents = res
                .rows()
                .into_iter()
                .map(|row| {
                    row.iter()
                        .map(|number| number.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                })
                .collect::<Vec<String>>()
                .join("\n");
            write_file(&file, &contents)?;
            Ok(file)
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
    // files = filter_first_snapshot_for_postprocessing_runs(files);
    files.sort_by_key(|snap_file| snap_file.file_name().unwrap().to_owned());
    Ok(Box::new(files.into_iter().filter(|f| {
        f.extension().map(|ext| ext == "hdf5").unwrap_or(false)
    })))
}

pub fn get_source_file(sim: &SimParams) -> Result<SourceFile> {
    match sim.get("SX_SOURCES").unwrap().unwrap_i64() {
        10 => {
            let path = sim
                .folder
                .join(sim.get("TestSrcFile").unwrap().unwrap_string());
            SourceFile::read(&path)
        }
        9 => Ok(SourceFile::from_params(sim)),
        _ => {
            panic!("Reading sources file for wrong SX_SOURCES value")
        }
    }
}

pub fn show_image(path: &Utf8Path) {
    let string = path.to_string();
    println!("Showing image {}", string);
    get_shell_command_output("nomacs", &[string], None, false);
}

pub fn show_image_folder(path: &Utf8Path) {
    let string = path.to_string();
    println!("Showing all images at {}", string);
    get_shell_command_output("nomacs", &[string], None, false);
}
