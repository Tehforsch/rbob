use std::fs;

use anyhow::anyhow;
use anyhow::Result;
use camino::Utf8Path;
use camino::Utf8PathBuf;
use serde::Deserialize;
use serde::Serialize;

use super::plot_template::PlotTemplate;
use super::snapshot::Snapshot;
use crate::config::DEFAULT_PIC_FOLDER;
use crate::sim_params::SimParams;
use crate::util::copy_file;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlotInfo {
    pub pic_folder: Utf8PathBuf,
    pub plot_name: String,
    pub name: String,
    pub qualified_name: String,
    pub plot_template_name: String,
}

impl PlotInfo {
    pub fn new(
        sim_set_folder: &Utf8Path,
        name: &str,
        qualified_name: &str,
        plot_template_name: Option<&str>,
        mb_sim: Option<&SimParams>,
        mb_snap: Option<&Snapshot>,
    ) -> PlotInfo {
        let plot_template_name = plot_template_name.unwrap_or(name).into();
        let name = format!("{}_{}", name, &plot_template_name);
        let qualified_name = format!("{}_{}", &qualified_name, &plot_template_name);
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
            None => qualified_name.clone(),
        };
        let pic_folder = sim_set_folder.join(DEFAULT_PIC_FOLDER);
        PlotInfo {
            pic_folder,
            plot_name,
            plot_template_name,
            name,
            qualified_name,
        }
    }

    pub fn get_plot_folder(&self) -> Utf8PathBuf {
        self.pic_folder.join(&self.plot_name)
    }

    pub fn get_data_folder(&self) -> Utf8PathBuf {
        self.get_plot_folder().join("data")
    }

    pub fn create_folders_if_nonexistent(&self) -> Result<()> {
        create_folder_if_nonexistent(&self.get_plot_folder())?;
        create_folder_if_nonexistent(&self.get_data_folder())
    }

    pub fn get_plot_template(&self) -> Result<PlotTemplate> {
        PlotTemplate::new(&self.plot_template_name)
    }

    pub fn find_pic_file_and_copy_one_folder_up(&self) -> Result<()> {
        let basename = &self.plot_name;
        let potential_extensions = ["pdf", "png"];
        let plot_folder = self.get_plot_folder();
        for extension in potential_extensions.iter() {
            let filename = format!("{}.{}", basename, extension);
            let path = plot_folder.join(&filename);
            if path.is_file() {
                let target = path.parent().unwrap().parent().unwrap().join(&filename);
                copy_file(&path, target)?;
                return Ok(());
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
