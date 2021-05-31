use anyhow::{anyhow, Context, Result};
use camino::Utf8Path;
use std::fs;

use crate::sim_params::get_config_file_path;
use crate::sim_params::SimParams;
use crate::sim_set::SimSet;
use crate::util::{copy_file, copy_recursive, get_shell_command_output};
use crate::{config, util::read_file_contents};

pub fn build_sim_set(arepo_path: &Utf8Path, sim_set: &SimSet, verbose: bool) -> Result<()> {
    for (i, sim) in sim_set.enumerate() {
        println!("Building sim {}", i);
        build_sim(arepo_path, sim, verbose)?;
    }
    Ok(())
}

fn build_sim(arepo_path: &Utf8Path, sim: &SimParams, verbose: bool) -> Result<()> {
    copy_config_file(arepo_path, sim)?;
    build_arepo(arepo_path, verbose)?;
    copy_arepo_file(arepo_path, sim)?;
    copy_source_code_to_output(arepo_path, sim)?;
    Ok(())
}

fn copy_source_code_to_output(arepo_path: &Utf8Path, sim: &SimParams) -> Result<()> {
    copy_recursive(
        arepo_path.join(config::DEFAULT_AREPO_SOURCE_FOLDER),
        sim.folder.join(config::DEFAULT_AREPO_SOURCE_FOLDER),
    )
}

fn build_arepo(arepo_path: &Utf8Path, verbose: bool) -> Result<()> {
    delete_arepoconfig_header_file_if_present(arepo_path)?;
    let out = get_shell_command_output(
        "make",
        &[
            &"build",
            &"-j",
            &config::DEFAULT_NUM_CORES_TO_COMPILE.to_string().as_ref(),
        ],
        Some(arepo_path),
        verbose,
    );
    if !out.success {
        println!("{}", out.stdout);
        println!("{}", out.stderr);
        return Err(anyhow!("Arepo compilation failed!"));
    }
    copy_arepoconfig_header_file(arepo_path) // For clang to make sense of the situation
}

fn delete_arepoconfig_header_file_if_present(arepo_path: &Utf8Path) -> Result<()> {
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

fn copy_arepoconfig_header_file(arepo_path: &Utf8Path) -> Result<()> {
    let source = arepo_path.join(config::DEFAULT_AREPO_CONFIG_BUILD_FILE);
    let target = arepo_path.join(config::DEFAULT_AREPO_CONFIG_SOURCE_FILE);
    copy_file(source, target)
}

fn copy_config_file(arepo_path: &Utf8Path, sim: &SimParams) -> Result<()> {
    let source = get_config_file_path(&sim.folder);
    let target = arepo_path.join(config::DEFAULT_CONFIG_FILE_NAME);
    if config_files_differ(&source, &target)? {
        copy_file(source, target)
    } else {
        println!("Config file is the same as in arepo file - not copying it");
        Ok(())
    }
}

fn config_files_differ(source: &Utf8Path, target: &Utf8Path) -> Result<bool> {
    if source.is_file() && target.is_file() {
        let contents_a = read_file_contents(source)?;
        let contents_b = read_file_contents(target)?;
        if contents_a == contents_b {
            return Ok(false);
        }
    }
    Ok(true)
}

fn copy_arepo_file(arepo_path: &Utf8Path, sim: &SimParams) -> Result<()> {
    let source = arepo_path.join(config::DEFAULT_AREPO_EXECUTABLE_NAME);
    let target = sim.folder.join(config::DEFAULT_AREPO_EXECUTABLE_NAME);
    copy_file(source, target)
}
