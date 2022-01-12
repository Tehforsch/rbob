use std::collections::HashMap;

use anyhow::anyhow;
use anyhow::Result;
use camino::Utf8Path;
use camino::Utf8PathBuf;
use itertools::Itertools;

use super::plot_info::PlotInfo;
use super::plot_info_file_contents::PlotInfoFileContents;
use super::plot_template::PlotTemplate;
use crate::config;
use crate::config::DEFAULT_PLOT_CONFIG_FOLDER_NAME;
use crate::config::DEFAULT_PLOT_EXTENSION;
use crate::config::DEFAULT_PLOT_FILE_NAME;
use crate::config::DEFAULT_PLOT_INFO_FILE_NAME;
use crate::config::PLOT_TEMPLATE_FOLDER;
use crate::util::copy_file;
use crate::util::copy_recursive;
use crate::util::create_folder_if_nonexistent;
use crate::util::get_files;
use crate::util::get_relative_path;
use crate::util::get_shell_command_output;
use crate::util::write_file;

pub struct Plotter {
    plot_folder: Utf8PathBuf,
    info: PlotInfo,
}

impl Plotter {
    pub fn from_sim_set_folder(folder: &Utf8Path, info: PlotInfo) -> Self {
        let pic_folder = folder.join(config::DEFAULT_PIC_FOLDER).to_owned();
        let plot_folder = pic_folder.join(&info.plot_name);
        Self { info, plot_folder }
    }

    pub fn from_plot_folder(plot_folder: &Utf8Path, info: PlotInfo) -> Self {
        Self {
            info,
            plot_folder: plot_folder.to_owned(),
        }
    }

    pub fn get_data_folder(&self) -> Utf8PathBuf {
        self.plot_folder.join("data")
    }

    pub fn create_folders_if_nonexistent(&self) -> Result<()> {
        create_folder_if_nonexistent(&self.plot_folder)?;
        create_folder_if_nonexistent(&self.get_data_folder())
    }

    pub fn get_plot_template(&self) -> Result<PlotTemplate> {
        PlotTemplate::new(&self.info.plot_template_name)
    }

    pub fn find_pic_file_and_copy_one_folder_up(&self) -> Result<()> {
        let basename = &self.info.plot_name;
        let potential_extensions = ["pdf", "png"];
        for extension in potential_extensions.iter() {
            let filename = format!("{}.{}", basename, extension);
            let path = self.plot_folder.join(&filename);
            if path.is_file() {
                let target = path.parent().unwrap().parent().unwrap().join(&filename);
                copy_file(&path, target)?;
                return Ok(());
            }
        }
        Err(anyhow!("No image file generated"))
    }

    pub fn write_plot_info_file(&self, replacements: &HashMap<String, String>) -> Result<()> {
        let contents = serde_yaml::to_string(&PlotInfoFileContents {
            info: self.info.clone(),
            replacements: replacements.clone(),
        })?;
        let info_file_name = self.plot_folder.join(DEFAULT_PLOT_INFO_FILE_NAME);
        write_file(&info_file_name, &contents)?;
        Ok(())
    }

    pub fn copy_plot_config_folder(&self) -> Result<()> {
        let source = PLOT_TEMPLATE_FOLDER.join(DEFAULT_PLOT_CONFIG_FOLDER_NAME);
        let target = self.plot_folder.join(DEFAULT_PLOT_CONFIG_FOLDER_NAME);
        copy_recursive(source, target)
    }

    pub fn get_default_replacements(
        &self,
        filenames: &[Utf8PathBuf],
    ) -> Result<HashMap<String, String>> {
        let mut result = HashMap::new();
        result.insert("numFiles".into(), filenames.len().to_string());
        result.insert("picFile".into(), in_quotes(&self.info.plot_name));
        result.insert(
            "files".into(),
            in_quotes(&self.get_joined_filenames(filenames)?),
        );
        Ok(result)
    }

    pub fn copy_plot_template(&self) -> Result<Utf8PathBuf> {
        let plot_template = self.get_plot_template()?;
        let plot_file = self
            .plot_folder
            .join(format!("{}.{}", &self.info.name, DEFAULT_PLOT_EXTENSION));
        plot_template.write_to(&plot_file)?;
        Ok(plot_file)
    }

    pub fn write_main_plot_file(&self, files_to_load: Vec<&Utf8Path>) -> Result<Utf8PathBuf> {
        let path = self.plot_folder.join(DEFAULT_PLOT_FILE_NAME);
        let contents = files_to_load
            .iter()
            .map(|file| format!("load \"{}\"", file.file_name().unwrap()))
            .join("\n");
        write_file(&path, &contents)?;
        Ok(path)
    }

    pub fn write_plot_param_file(
        &self,
        replacements: &HashMap<String, String>,
    ) -> Result<Utf8PathBuf> {
        let path = self.plot_folder.join("params.gp");
        let contents = self.get_plot_param_file_contents(replacements)?;
        write_file(&path, &contents)?;
        Ok(path)
    }

    fn get_plot_param_file_contents(
        &self,
        replacements: &HashMap<String, String>,
    ) -> Result<String> {
        let contents = replacements
            .iter()
            .map(|(key, value)| format!("{} = {}", key, value))
            .join("\n");
        Ok(contents)
    }

    fn get_joined_filenames(&self, filenames: &[Utf8PathBuf]) -> Result<String> {
        Ok(filenames
            .iter()
            .map(|filename| {
                get_relative_path(filename, &self.plot_folder)
                    .map(|rel_path| rel_path.as_str().to_owned())
            })
            .collect::<Result<Vec<String>>>()?
            .join(" "))
    }

    pub fn run_gnuplot_command(&self, plot_file: &Utf8Path) -> Result<()> {
        let out = get_shell_command_output(
            "gnuplot",
            &[&plot_file.file_name().unwrap()],
            Some(&self.plot_folder),
            false,
        );
        match out.success {
            false => Err(anyhow!("Error in gnuplot command:\n{}", out.stderr)),
            true => Ok(()),
        }
    }

    pub fn maybe_run_pdflatex(&self) -> Result<()> {
        let files = get_files(&self.plot_folder)?;
        let latex_file = files.iter().find(|file| {
            file.extension()
                .map(|extension| extension == "tex")
                .unwrap_or(false)
        });
        if let Some(latex_file) = latex_file {
            let out = get_shell_command_output(
                "pdflatex",
                &[&latex_file.file_name().unwrap()],
                Some(&self.plot_folder),
                false,
            );
            match out.success {
                false => Err(anyhow!("Error in pdflatex command:\n{}", out.stderr)),
                true => Ok(()),
            }
        } else {
            Ok(())
        }
    }
}

fn in_quotes(s: &str) -> String {
    format!("\"{}\"", s)
}
