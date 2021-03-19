use super::{plot_template::PlotTemplate, snapshot::Snapshot};
use crate::{config, config_file::ConfigFile, sim_params::SimParams};
use anyhow::Result;
use camino::{Utf8Path, Utf8PathBuf};
use std::fs;

pub struct PlotInfo {
    pub pic_folder: Utf8PathBuf,
    pub plot_folder: Utf8PathBuf,
    pub data_folder: Utf8PathBuf,
    pub plot_name: String,
    pub name: String,
    pub qualified_name: String,
}

impl PlotInfo {
    pub fn new(
        sim_set_folder: &Utf8Path,
        name: &str,
        qualified_name: &str,
        mb_sim: Option<&SimParams>,
        mb_snap: Option<&Snapshot>,
    ) -> PlotInfo {
        let plot_name = match mb_sim {
            Some(sim) => {
                let sim_name = sim.folder.file_name().unwrap();
                match mb_snap {
                    None => format!("{}_{}", qualified_name, sim_name),
                    Some(snap) => {
                        format!("{}_{}_{}", qualified_name, sim_name, snap.get_name())
                    }
                }
            }
            None => qualified_name.into(),
        }
        .to_owned();
        let pic_folder = sim_set_folder.join("pics");
        let plot_folder = sim_set_folder.join("pics").join(&plot_name);
        let data_folder = plot_folder.join("data");
        PlotInfo {
            pic_folder,
            plot_folder,
            data_folder,
            plot_name,
            name: name.into(),
            qualified_name: qualified_name.into(),
        }
    }

    pub fn create_folders_if_nonexistent(&self) -> Result<()> {
        create_folder_if_nonexistent(&self.plot_folder)?;
        create_folder_if_nonexistent(&self.data_folder)
    }

    pub fn get_plot_template(&self, config_file: &ConfigFile) -> Result<PlotTemplate> {
        PlotTemplate::new(config_file, &self.name)
    }

    pub fn get_pic_file(&self) -> Utf8PathBuf {
        let filename = format!("{}.{}", self.plot_name, config::PIC_FILE_ENDING);
        self.pic_folder.join(filename).to_path_buf()
    }
}

fn create_folder_if_nonexistent(folder: &Utf8Path) -> Result<()> {
    if !folder.is_dir() {
        fs::create_dir_all(&folder)?;
    };
    Ok(())
}
