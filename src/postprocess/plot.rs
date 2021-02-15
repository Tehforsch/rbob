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
