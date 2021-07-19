use super::{plot_template::PlotTemplate, snapshot::Snapshot};
use crate::{config_file::ConfigFile, sim_params::SimParams, util::copy_file};

use anyhow::{anyhow, Result};
use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlotInfo {
    pub pic_folder: Utf8PathBuf,
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
        };
        let pic_folder = sim_set_folder.join("pics");
        PlotInfo {
            pic_folder,
            plot_name,
            name: name.into(),
            qualified_name: qualified_name.into(),
        }
    }

    pub fn get_plot_folder(&self) -> Utf8PathBuf {
        self.pic_folder.join(&self.name)
    }

    pub fn get_data_folder(&self) -> Utf8PathBuf {
        self.get_plot_folder().join("data")
    }

    pub fn create_folders_if_nonexistent(&self) -> Result<()> {
        create_folder_if_nonexistent(&self.get_plot_folder())?;
        create_folder_if_nonexistent(&self.get_data_folder())
    }

    pub fn get_plot_template(
        &self,
        config_file: &ConfigFile,
        plot_template_name: Option<&str>,
    ) -> Result<PlotTemplate> {
        let plot_template_name = match plot_template_name {
            Some(name) => name,
            None => &self.name,
        };
        PlotTemplate::new(config_file, plot_template_name)
    }

    pub fn find_pic_file_and_copy_one_folder_up(&self) -> Result<Utf8PathBuf> {
        let basename = self.name.clone();
        let potential_extensions = ["pdf", "png"];
        let plot_folder = self.get_plot_folder();
        for extension in potential_extensions.iter() {
            let filename = format!("{}.{}", &basename, extension);
            let path = plot_folder.join(&filename);
            if path.is_file() {
                let target = path.parent().unwrap().parent().unwrap().join(&filename);
                copy_file(&path, target)?;
                return Ok(path);
            }
        }
        Err(anyhow!("No image file generated"))
    }
}

fn create_folder_if_nonexistent(folder: &Utf8Path) -> Result<()> {
    if !folder.is_dir() {
        fs::create_dir_all(&folder)?;
    };
    Ok(())
}
