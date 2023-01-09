use anyhow::Context;
use anyhow::Result;
use camino::Utf8Path;
use camino::Utf8PathBuf;
use snapshot::Snapshot;

use self::data_plot_info::DataPlotInfo;
use self::plotter::Plotter;
use self::postprocess_args::PostprocessArgs;
use crate::config;
use crate::config::DEFAULT_PIC_FOLDER;
use crate::sim_params::SimParams;
use crate::sim_set::SimSet;
use crate::source_file::SourceFile;
use crate::thread_pool::ThreadPool;
use crate::util::get_files;
use crate::util::get_shell_command_output;
use crate::util::write_file;

pub mod axis;
pub mod calculations;
pub mod data_plot_info;
pub mod field_identifier;
mod named;
pub mod plot;
pub mod plot_info;
pub mod plot_info_file_contents;
pub mod plot_params;
pub mod plot_template;
mod plotter;
pub mod post_compare;
pub mod post_convergence;
pub mod post_expansion;
pub mod post_fn;
pub mod post_fn_name;
pub mod post_ionization;
pub mod post_scaling;
pub mod post_shadowing;
pub mod post_slice;
pub mod postprocess_args;
pub mod read_hdf5;
pub mod replot_args;
pub mod snapshot;
pub mod voronoi_swim;

pub fn postprocess_sim_set(
    create_plot: bool,
    sim_set: SimSet,
    args: &PostprocessArgs,
) -> Result<()> {
    let sim_set = filter_sim_set(sim_set, args.select.as_ref());
    let data_plot_info_iter = args.function.run(&sim_set, args.plot_template.as_deref());
    let mut pool: ThreadPool<anyhow::Result<()>, _> = ThreadPool::new(config::MAX_NUM_POST_THREADS);
    for data_plot_info in data_plot_info_iter {
        let folder = sim_set.get_folder()?;
        pool.add_job(move || {
            let data_plot_info = data_plot_info?;
            let plotter = Plotter::from_sim_set_folder(&folder, data_plot_info.info.clone());
            plotter.create_folders_if_nonexistent()?;
            let filenames = write_results(&plotter.get_data_folder(), &data_plot_info)?;
            plot::run_plot(
                &plotter,
                create_plot,
                &filenames,
                &data_plot_info.replacements,
            )
            .unwrap();
            Ok(())
        });
    }
    let results: Vec<_> = pool.collect();
    for result in results.into_iter() {
        result?;
    }
    if args.showall {
        let pic_folder = sim_set.get_folder()?.join(DEFAULT_PIC_FOLDER);
        show_image_folder(&pic_folder);
    }
    Ok(())
}

fn filter_sim_set(sim_set: SimSet, select: Option<&Vec<usize>>) -> SimSet {
    if let Some(selected_sims) = select {
        sim_set
            .into_iter()
            .filter(|(num, _)| selected_sims.contains(num))
            .collect()
    } else {
        sim_set
    }
}

pub fn write_results(
    data_folder: &Utf8Path,
    data_plot_info: &DataPlotInfo,
) -> Result<Vec<Utf8PathBuf>> {
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

fn filter_first_snapshot_for_postprocessing_runs(
    sim: &SimParams,
    files: Vec<Utf8PathBuf>,
) -> Vec<Utf8PathBuf> {
    if sim
        .get("runParams")
        .map(|run_params| run_params.unwrap_string().contains("21"))
        .unwrap_or(false)
    {
        files
            .iter()
            .filter(move |file| file.file_name().unwrap() != "snap_000.hdf5")
            .map(|pb| pb.to_owned())
            .collect()
    } else {
        files
    }
}

pub fn get_snapshot_files(sim: &SimParams) -> Result<Box<dyn Iterator<Item = Utf8PathBuf>>> {
    let mut files = get_files(&sim.output_folder()).context(format!(
        "No output folder in simulation folder: {:?} (looking in {:?})",
        sim.folder,
        sim.output_folder(),
    ))?;
    files = filter_first_snapshot_for_postprocessing_runs(sim, files);
    files.sort_by_key(|snap_file| snap_file.file_name().unwrap().to_owned());
    Ok(Box::new(files.into_iter().filter(|f| {
        f.extension().map(|ext| ext == "hdf5").unwrap_or(false)
    })))
}

pub fn get_source_file(sim: &SimParams) -> Result<SourceFile> {
    match sim.get("SWEEP_SOURCES").unwrap().unwrap_i64() {
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
