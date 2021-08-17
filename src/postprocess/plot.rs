use anyhow::{anyhow, Context, Result};
use camino::{Utf8Path, Utf8PathBuf};
use itertools::Itertools;
use std::collections::HashMap;

use super::{
    plot_info::PlotInfo, plot_info_file_contents::PlotInfoFileContents, replot_args::ReplotArgs,
};
use crate::{
    config::{DEFAULT_PLOT_CONFIG_FOLDER_NAME, PLOT_TEMPLATE_FOLDER, DEFAULT_PLOT_EXTENSION, DEFAULT_PLOT_FILE_NAME, DEFAULT_PLOT_INFO_FILE_NAME},
    util::{
        copy_recursive, get_files, get_folders, get_relative_path, get_shell_command_output,
        read_file_contents, write_file,
    },
};

pub fn run_plot(
    create_plot: bool,
    info: &PlotInfo,
    filenames: &[Utf8PathBuf],
    special_replacements: &HashMap<String, String>,
) -> Result<Utf8PathBuf> {
    let mut replacements = get_default_replacements(info, filenames)?;
    for (k, v) in special_replacements {
        replacements.insert(k.to_string(), v.to_string());
    }
    let plot_param_file = write_plot_param_file(info, filenames, &replacements)?;
    let plot_template = copy_plot_template(info)?;
    let main_plot_file = write_main_plot_file(info, vec![&plot_param_file, &plot_template])?;
    copy_plot_config_folder(info)?;
    write_plot_info_file(info, &replacements)?;
    if create_plot {
        run_gnuplot_command(info, &main_plot_file)?;
        maybe_run_pdflatex(info)?;
        info.find_pic_file_and_copy_one_folder_up()
    } else {
        Err(anyhow!(
            "Plot was not actually created - no output file exists. Run with plot instead of post to change this"
        ))
    }
}

pub fn replot(args: &ReplotArgs) -> Result<()> {
    let pic_folder = args.folder.join("pics");
    for folder in get_folders(&pic_folder)? {
        let plot_info_file = folder.join(DEFAULT_PLOT_INFO_FILE_NAME);
        let plot_info = read_plot_info_file(&plot_info_file)?;
        run_plot(
            true,
            &plot_info.info,
            &[],
            &plot_info.replacements,
        )?;
    }
    Ok(())
}

fn write_plot_info_file(info: &PlotInfo, replacements: &HashMap<String, String>) -> Result<()> {
    let contents = serde_yaml::to_string(&PlotInfoFileContents {
        info: info.clone(),
        replacements: replacements.clone(),
    })?;
    let info_file_name = info
        .get_plot_folder()
        .join(DEFAULT_PLOT_INFO_FILE_NAME);
    write_file(&info_file_name, &contents)?;
    Ok(())
}

fn read_plot_info_file(path: &Utf8Path) -> Result<PlotInfoFileContents> {
    let contents = read_file_contents(path)?;
    serde_yaml::from_str(&contents).context("While reading plot info file")
}

fn copy_plot_config_folder(info: &PlotInfo) -> Result<()> {
    let source = PLOT_TEMPLATE_FOLDER
        .join(DEFAULT_PLOT_CONFIG_FOLDER_NAME);
    let target = info
        .get_plot_folder()
        .join(DEFAULT_PLOT_CONFIG_FOLDER_NAME);
    copy_recursive(source, target)
}

fn get_default_replacements(
    info: &PlotInfo,
    filenames: &[Utf8PathBuf],
) -> Result<HashMap<String, String>> {
    let mut result = HashMap::new();
    result.insert("numFiles".into(), filenames.len().to_string());
    result.insert("picFile".into(), in_quotes(&info.name));
    result.insert(
        "files".into(),
        in_quotes(&get_joined_filenames(info, filenames)?),
    );
    Ok(result)
}

fn in_quotes(s: &str) -> String {
    format!("\"{}\"", s)
}

fn copy_plot_template(
    info: &PlotInfo,
) -> Result<Utf8PathBuf> {
    let plot_template = info.get_plot_template()?;
    let plot_file =
        info.get_plot_folder()
            .join(format!("{}.{}", &info.name, DEFAULT_PLOT_EXTENSION));
    plot_template.write_to(&plot_file)?;
    Ok(plot_file)
}

fn write_main_plot_file(info: &PlotInfo, files_to_load: Vec<&Utf8Path>) -> Result<Utf8PathBuf> {
    let path = info.get_plot_folder().join(DEFAULT_PLOT_FILE_NAME);
    let contents = files_to_load
        .iter()
        .map(|file| format!("load \"{}\"", file.file_name().unwrap()))
        .join("\n");
    write_file(&path, &contents)?;
    Ok(path)
}

fn write_plot_param_file(
    info: &PlotInfo,
    filenames: &[Utf8PathBuf],
    replacements: &HashMap<String, String>,
) -> Result<Utf8PathBuf> {
    let path = info.get_plot_folder().join("params.gp");
    let contents = get_plot_param_file_contents(info, filenames, replacements)?;
    write_file(&path, &contents)?;
    Ok(path)
}

fn get_plot_param_file_contents(
    _info: &PlotInfo,
    _filenames: &[Utf8PathBuf],
    replacements: &HashMap<String, String>,
) -> Result<String> {
    let contents = replacements
        .iter()
        .map(|(key, value)| format!("{} = {}", key, value))
        .join("\n");
    Ok(contents)
}

fn get_joined_filenames(info: &PlotInfo, filenames: &[Utf8PathBuf]) -> Result<String> {
    Ok(filenames
        .iter()
        .map(|filename| {
            get_relative_path(filename, &info.get_plot_folder())
                .map(|rel_path| rel_path.as_str().to_owned())
        })
        .collect::<Result<Vec<String>>>()?
        .join(" "))
}

fn run_gnuplot_command(info: &PlotInfo, plot_file: &Utf8Path) -> Result<()> {
    let out = get_shell_command_output(
        "gnuplot",
        &[&plot_file.file_name().unwrap()],
        Some(&info.get_plot_folder()),
        false,
    );
    match out.success {
        false => Err(anyhow!("Error in gnuplot command:\n{}", out.stderr)),
        true => Ok(()),
    }
}

fn maybe_run_pdflatex(info: &PlotInfo) -> Result<()> {
    let plot_folder = info.get_plot_folder();
    let files = get_files(&plot_folder)?;
    let latex_file = files.iter().find(|file| {
        file.extension()
            .map(|extension| extension == "tex")
            .unwrap_or(false)
    });
    if let Some(latex_file) = latex_file {
        let out = get_shell_command_output(
            "pdflatex",
            &[&latex_file.file_name().unwrap()],
            Some(&info.get_plot_folder()),
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
