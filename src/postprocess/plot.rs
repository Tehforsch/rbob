use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use super::plot_template::PlotTemplate;
use super::{post_fn_name::PostFnName, snapshot::Snapshot};
use crate::{
    config, config_file::ConfigFile, sim_params::SimParams, util::get_shell_command_output,
};

pub struct PlotInfo {
    pub plot_folder: PathBuf,
    pub data_folder: PathBuf,
    pub plot_name: String,
    pub function_name: String,
}

impl PlotInfo {
    pub fn new(
        sim_set_folder: &Path,
        mb_sim: Option<&SimParams>,
        function: &PostFnName,
        mb_snap: Option<&Snapshot>,
    ) -> PlotInfo {
        let plot_name = match mb_sim {
            Some(sim) => {
                let sim_name = sim.folder.file_name().unwrap().to_str().unwrap();
                match mb_snap {
                    None => format!("{}_{}", function.to_string(), sim_name),
                    Some(snap) => {
                        format!("{}_{}_{}", function.to_string(), sim_name, snap.get_name())
                    }
                }
            }
            None => function.to_string(),
        }
        .to_owned();
        let plot_folder = sim_set_folder.join("pics").join(&plot_name);
        let data_folder = plot_folder.join("data");
        PlotInfo {
            plot_folder,
            data_folder,
            plot_name,
            function_name: function.to_string(),
        }
    }

    pub fn create_folders_if_nonexistent(&self) -> Result<()> {
        create_folder_if_nonexistent(&self.plot_folder)?;
        create_folder_if_nonexistent(&self.data_folder)
    }

    pub fn get_plot_template(&self, config_file: &ConfigFile) -> Result<PlotTemplate> {
        PlotTemplate::new(config_file, &self.function_name)
    }
}

fn create_folder_if_nonexistent(folder: &Path) -> Result<()> {
    if !folder.is_dir() {
        fs::create_dir_all(&folder)?;
    };
    Ok(())
}

pub fn run_plot(
    config_file: &ConfigFile,
    info: &PlotInfo,
    filenames: &Vec<PathBuf>,
    replacements: HashMap<String, String>,
) -> Result<()> {
    let plot_file = copy_plot_template(config_file, info, replacements)?;
    run_gnuplot_command(info, &plot_file)?;
    Ok(())
}

fn run_gnuplot_command(info: &PlotInfo, plot_file: &Path) -> Result<()> {
    let out = get_shell_command_output(
        "gnuplot",
        &[&plot_file.file_name().unwrap().to_str().unwrap()],
        Some(&info.plot_folder),
    );
    match out.success {
        false => Err(anyhow!("Error in gnuplot command: {}", out.stderr)),
        true => Ok(()),
    }
}

fn copy_plot_template(
    config_file: &ConfigFile,
    info: &PlotInfo,
    replacements: HashMap<String, String>,
) -> Result<PathBuf> {
    let plot_template = info.get_plot_template(config_file)?;
    let plot_file = info.plot_folder.join(config::DEFAULT_PLOT_FILE_NAME);
    plot_template.write_to(&plot_file, replacements)?;
    Ok(plot_file.to_owned())
}
