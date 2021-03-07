use anyhow::{anyhow, Context, Result};
use camino::{Utf8Path, Utf8PathBuf};
use std::fs;

use crate::config;
use crate::sim_params::get_config_file_path;
use crate::sim_params::SimParams;
use crate::sim_set::SimSet;
use crate::util::{copy_file, copy_recursive, get_shell_command_output};

pub fn build_sim_set(sim_set: &SimSet) -> Result<()> {
    for (i, sim) in sim_set.enumerate() {
        println!("Building sim {}", i);
        build_sim(sim)?;
    }
    Ok(())
}

fn build_sim(sim: &SimParams) -> Result<()> {
    copy_config_file(sim)?;
    build_arepo()?;
    copy_arepo_file(sim)?;
    copy_source_code_to_output(sim)?;
    Ok(())
}

fn copy_source_code_to_output(sim: &SimParams) -> Result<()> {
    copy_recursive(
        get_arepo_path().join(config::DEFAULT_AREPO_SOURCE_FOLDER),
        sim.folder.join(config::DEFAULT_AREPO_SOURCE_FOLDER),
    )
}

fn build_arepo() -> Result<()> {
    let arepo_path = Utf8Path::new(config::DEFAULT_AREPO_FOLDER);
    delete_arepoconfig_header_file_if_present()?;
    let out = get_shell_command_output("make", &[&"build"], Some(arepo_path));
    if !out.success {
        println!("{}", out.stdout);
        println!("{}", out.stderr);
        return Err(anyhow!("Arepo compilation failed!"));
    }
    copy_arepoconfig_header_file() // For clang to make sense of the situation
}

fn delete_arepoconfig_header_file_if_present() -> Result<()> {
    let arepo_path = get_arepo_path();
    let file = arepo_path.join(config::DEFAULT_AREPO_CONFIG_SOURCE_FILE);
    match file.is_file() {
        true => fs::remove_file(&file).with_context(|| {
            format!(
                "While deleting the arepo config file in the src folder at {:?}",
                &file
            )
        }),
        false => Ok(()),
    }
}

fn copy_arepoconfig_header_file() -> Result<()> {
    let arepo_path = get_arepo_path();
    let source = arepo_path.join(config::DEFAULT_AREPO_CONFIG_BUILD_FILE);
    let target = arepo_path.join(config::DEFAULT_AREPO_CONFIG_SOURCE_FILE);
    copy_file(source, target)
}

fn copy_config_file(sim: &SimParams) -> Result<()> {
    let source = get_config_file_path(&sim.folder);
    let arepo_path = Utf8Path::new(config::DEFAULT_AREPO_FOLDER);
    let target = arepo_path.join(config::DEFAULT_CONFIG_FILE_NAME);
    copy_file(source, target)
}

fn copy_arepo_file(sim: &SimParams) -> Result<()> {
    let arepo_path = get_arepo_path();
    let source = arepo_path.join(config::DEFAULT_AREPO_EXECUTABLE_NAME);
    let target = sim.folder.join(config::DEFAULT_AREPO_EXECUTABLE_NAME);
    copy_file(source, target)
}

fn get_arepo_path() -> Utf8PathBuf {
    Utf8Path::new(config::DEFAULT_AREPO_FOLDER).to_path_buf()
}
