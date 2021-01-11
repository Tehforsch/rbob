use anyhow::Result;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use crate::sim_params::SimParams;

use super::{post_fn_name::PostFnName, snapshot::Snapshot};

pub struct PlotInfo {
    pub plot_folder: PathBuf,
    pub data_folder: PathBuf,
    pub plot_name: String,
}

impl PlotInfo {
    pub fn new(
        sim_set_folder: &Path,
        sim: &SimParams,
        function: &PostFnName,
        snap: Option<&Snapshot>,
    ) -> PlotInfo {
        let sim_name = sim.folder.file_name().unwrap().to_str().unwrap();
        let plot_name = match snap {
            None => format!("{}_{}", function.to_string(), sim_name),
            Some(s) => format!("{}_{}_{}", function.to_string(), sim_name, s.get_name()),
        }
        .to_owned();
        let plot_folder = sim_set_folder.join("pics").join(&plot_name);
        let data_folder = plot_folder.join("data");
        PlotInfo {
            plot_folder,
            data_folder,
            plot_name,
        }
    }

    pub fn create_folders_if_nonexistent(&self) -> Result<()> {
        create_folder_if_nonexistent(&self.plot_folder)?;
        create_folder_if_nonexistent(&self.data_folder)
    }
}

fn create_folder_if_nonexistent(folder: &Path) -> Result<()> {
    if !folder.is_dir() {
        fs::create_dir_all(&folder)?;
    };
    Ok(())
}

fn run_plot(info: &PlotInfo, kwargs: HashMap<String, String>) {
    dbg!("RUNNING SOME SHIT");
}
