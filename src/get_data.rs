use anyhow::Result;
use camino::{Utf8Path, Utf8PathBuf};

use crate::{
    config::{
        DEFAULT_CONFIG_FILE_NAME, DEFAULT_JOB_FILE_NAME, DEFAULT_LOG_FILE, DEFAULT_PARAM_FILE_NAME,
    },
    util::{copy_file, get_files, get_folders},
};

pub fn get_data(source: &Utf8Path, target: &Utf8Path) -> Result<()> {
    for sim_folder in get_folders(source)? {
        let target_sim_folder = target.join(sim_folder.file_name().unwrap());
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
    for snapshot in get_snapshot_filenames(source_sim_folder)?.iter() {
        copy_file_relative(&snapshot)?;
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

fn get_snapshot_filenames(source_sim_folder: &Utf8Path) -> Result<Vec<String>> {
    let output_folder = "output";
    Ok(get_files(&source_sim_folder.join(output_folder))?
        .iter()
        .filter(|file| {
            file.extension()
                .map(|extension| extension == "hdf5")
                .unwrap_or(false)
        })
        .map(|path| format!("{}/{}", output_folder, path.file_name().unwrap()))
        .collect())
}
