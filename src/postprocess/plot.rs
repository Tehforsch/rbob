use anyhow::{anyhow, Result};
use camino::{Utf8Path, Utf8PathBuf};
use itertools::Itertools;
use std::collections::HashMap;

use super::plot_info::PlotInfo;
use crate::{
    config,
    config_file::ConfigFile,
    util::{copy_recursive, get_relative_path, get_shell_command_output, write_file},
};

pub fn run_plot(
    config_file: &ConfigFile,
    info: &PlotInfo,
    filenames: &Vec<Utf8PathBuf>,
) -> Result<String> {
    let replacements = get_default_replacements(info, filenames)?;
    let plot_param_file = write_plot_param_file(info, filenames, &replacements)?;
    let plot_template = copy_plot_template(config_file, info)?;
    let main_plot_file = write_main_plot_file(info, vec![&plot_param_file, &plot_template])?;
    copy_plot_config_folder(config_file, info)?;
    run_gnuplot_command(info, &main_plot_file)?;
    Ok(info.get_pic_file().as_str().into())
}

fn copy_plot_config_folder(config_file: &ConfigFile, info: &PlotInfo) -> Result<()> {
    let source = config_file
        .plot_template_folder
        .join(config::DEFAULT_PLOT_CONFIG_FOLDER_NAME);
    let target = info
        .plot_folder
        .join(config::DEFAULT_PLOT_CONFIG_FOLDER_NAME);
    copy_recursive(source, target)
}

fn get_default_replacements(
    info: &PlotInfo,
    filenames: &Vec<Utf8PathBuf>,
) -> Result<HashMap<String, String>> {
    let mut result = HashMap::new();
    result.insert("numFiles".into(), filenames.len().to_string());
    result.insert(
        "picFile".into(),
        in_quotes(
            &get_relative_path(&info.get_pic_file(), &info.plot_folder)?
                .as_str()
                .to_string(),
        ),
    );
    result.insert(
        "files".into(),
        in_quotes(&get_joined_filenames(info, filenames)?),
    );
    Ok(result)
}

fn in_quotes(s: &str) -> String {
    format!("\"{}\"", s)
}

fn copy_plot_template(config_file: &ConfigFile, info: &PlotInfo) -> Result<Utf8PathBuf> {
    let plot_template = info.get_plot_template(config_file)?;
    let plot_file =
        info.plot_folder
            .join(format!("{}.{}", &info.name, config::DEFAULT_PLOT_EXTENSION));
    plot_template.write_to(&plot_file)?;
    Ok(plot_file.to_owned())
}

fn write_main_plot_file(info: &PlotInfo, files_to_load: Vec<&Utf8Path>) -> Result<Utf8PathBuf> {
    let path = info.plot_folder.join(config::DEFAULT_PLOT_FILE_NAME);
    let contents = files_to_load
        .iter()
        .map(|file| format!("load \"{}\"", file.file_name().unwrap()))
        .join("\n");
    write_file(&path, &contents)?;
    Ok(path.to_owned())
}

fn write_plot_param_file(
    info: &PlotInfo,
    filenames: &Vec<Utf8PathBuf>,
    replacements: &HashMap<String, String>,
) -> Result<Utf8PathBuf> {
    let path = info.plot_folder.join("params.gp");
    let contents = get_plot_param_file_contents(info, filenames, replacements)?;
    write_file(&path, &contents)?;
    Ok(path.to_owned())
}

fn get_plot_param_file_contents(
    _info: &PlotInfo,
    _filenames: &Vec<Utf8PathBuf>,
    replacements: &HashMap<String, String>,
) -> Result<String> {
    let contents = replacements
        .iter()
        .map(|(key, value)| format!("{} = {}", key, value))
        .join("\n");
    Ok(contents)
}

fn get_joined_filenames(info: &PlotInfo, filenames: &Vec<Utf8PathBuf>) -> Result<String> {
    Ok(filenames
        .iter()
        .map(|filename| {
            get_relative_path(filename, &info.plot_folder)
                .map(|rel_path| rel_path.as_str().to_owned())
        })
        .collect::<Result<Vec<String>>>()?
        .join(" "))
}

fn run_gnuplot_command(info: &PlotInfo, plot_file: &Utf8Path) -> Result<()> {
    let out = get_shell_command_output(
        "gnuplot",
        &[&plot_file.file_name().unwrap()],
        Some(&info.plot_folder),
    );
    match out.success {
        false => Err(anyhow!("Error in gnuplot command:\n{}", out.stderr)),
        true => Ok(()),
    }
}
