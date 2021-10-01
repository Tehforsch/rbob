use std::fs;

use anyhow::Result;
use camino::Utf8Path;

use crate::{
    config::{
        DEFAULT_CONFIG_FILE_NAME, DEFAULT_JOB_FILE_NAME, DEFAULT_LOG_FILE, DEFAULT_PARAM_FILE_NAME,
    },
    util::{copy_file, get_files, get_folders},
};

pub fn get_data(source: &Utf8Path, target: &Utf8Path) -> Result<()> {
    for sim_folder in get_folders(source)? {
        let target_sim_folder = target.join(sim_folder.file_name().unwrap());
        fs::create_dir_all(&target_sim_folder)?;
        get_files_for_sim(&sim_folder, &target_sim_folder)?;
    }
    Ok(())
}

fn get_files_for_sim(source_sim_folder: &Utf8Path, target_sim_folder: &Utf8Path) -> Result<()> {
    let copy_file_relative =
        |filename| copy_file_by_name(source_sim_folder, target_sim_folder, filename);
    copy_file_relative(DEFAULT_PARAM_FILE_NAME)?;
    copy_file_relative(DEFAULT_CONFIG_FILE_NAME)?;
    copy_file_relative(DEFAULT_JOB_FILE_NAME)?;
    copy_file_relative(DEFAULT_LOG_FILE)?;
    fs::create_dir_all(&target_sim_folder)?;
    let source_output_folder = source_sim_folder.join("output");
    let target_output_folder = target_sim_folder.join("output");
    fs::create_dir_all(&target_output_folder)?;
    for snapshot in get_snapshot_filenames(&source_output_folder)?.iter() {
        copy_file_relative(snapshot)?;
    }
    Ok(())
}

fn copy_file_by_name(
    source_sim_folder: &Utf8Path,
    target_sim_folder: &Utf8Path,
    filename: &str,
) -> Result<()> {
    let source = source_sim_folder.join(filename);
    let target = target_sim_folder.join(filename);
    println!("Copying {} -> {}", source, target);
    copy_file(source, target)
}

fn get_snapshot_filenames(output_folder: &Utf8Path) -> Result<Vec<String>> {
    Ok(get_files(output_folder)?
        .iter()
        .filter(|file| {
            file.extension()
                .map(|extension| extension == "hdf5")
                .unwrap_or(false)
        })
        .map(|path| {
            format!(
                "{}/{}",
                output_folder.file_name().unwrap(),
                path.file_name().unwrap()
            )
        })
        .collect())
}
