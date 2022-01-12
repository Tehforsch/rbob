use std::collections::HashMap;

use anyhow::Context;
use anyhow::Result;
use camino::Utf8Path;
use camino::Utf8PathBuf;

use super::plot_info_file_contents::PlotInfoFileContents;
use super::plotter::Plotter;
use super::replot_args::ReplotArgs;
use crate::config;
use crate::config::DEFAULT_PIC_FOLDER;
use crate::config::DEFAULT_PLOT_INFO_FILE_NAME;
use crate::thread_pool::ThreadPool;
use crate::util::get_folders;
use crate::util::read_file_contents;

pub fn run_plot(
    plotter: &Plotter,
    create_plot: bool,
    filenames: &[Utf8PathBuf],
    special_replacements: &HashMap<String, String>,
) -> Result<()> {
    let mut replacements = plotter.get_default_replacements(filenames)?;
    for (k, v) in special_replacements {
        replacements.insert(k.to_string(), v.to_string());
    }
    let plot_param_file = plotter.write_plot_param_file(&replacements)?;
    let plot_template = plotter.copy_plot_template()?;
    let main_plot_file = plotter.write_main_plot_file(vec![&plot_param_file, &plot_template])?;
    plotter.copy_plot_config_folder()?;
    plotter.write_plot_info_file(&replacements)?;
    if create_plot {
        plotter.run_gnuplot_command(&main_plot_file)?;
        plotter.maybe_run_pdflatex()?;
        plotter.find_pic_file_and_copy_one_folder_up()
    } else {
        Ok(())
    }
}

pub fn read_plot_info_file(path: &Utf8Path) -> Result<PlotInfoFileContents> {
    let contents = read_file_contents(path)?;
    serde_yaml::from_str(&contents).context("While reading plot info file")
}

pub fn replot(args: &ReplotArgs) -> Result<()> {
    let mut pool: ThreadPool<Result<()>, _> = ThreadPool::new(config::MAX_NUM_POST_THREADS);
    let pic_folder = args.folder.join(DEFAULT_PIC_FOLDER);
    for folder in get_folders(&pic_folder)? {
        println!("Replotting {}", &folder);
        pool.add_job(move || {
            let plot_info_file = folder.join(DEFAULT_PLOT_INFO_FILE_NAME);
            let plot_info = read_plot_info_file(&plot_info_file)?;
            let plotter = Plotter::from_plot_folder(&folder, plot_info.info);
            run_plot(&plotter, true, &[], &plot_info.replacements)?;
            Ok(())
        });
    }
    for result in pool.collect::<Vec<_>>().into_iter() {
        result?
    }
    Ok(())
}
