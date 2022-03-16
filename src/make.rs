use std::fs;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use camino::Utf8Path;

use crate::config;
use crate::config::AREPO_PATH;
use crate::config::DEFAULT_SYSTYPE;
use crate::sim_params::get_config_file_path;
use crate::sim_params::SimParams;
use crate::sim_set::SimSet;
use crate::systype::Systype;
use crate::util::copy_file;
use crate::util::copy_recursive;
use crate::util::get_shell_command_output;
use crate::util::read_file_contents;
use crate::util::write_file;

pub fn build_sim_set(sim_set: &SimSet, verbose: bool, systype: &Option<Systype>) -> Result<()> {
    for (i, sim) in sim_set.enumerate() {
        println!("Building sim {}", i);
        build_sim(sim, verbose, systype)?;
    }
    // copy_source_code_to_output(&config::AREPO_PATH, &sim_set.iter().next().unwrap().folder)?;
    Ok(())
}

fn build_sim(sim: &SimParams, verbose: bool, systype: &Option<Systype>) -> Result<()> {
    write_systype_file(systype)?;
    copy_config_file(sim)?;
    if let Some(commit) = sim.get("arepoCommit") {
        checkout_arepo_commit(commit.unwrap_string())?;
    }
    build_arepo(verbose)?;
    copy_arepo_file(sim)?;
    Ok(())
}

fn checkout_arepo_commit(commit: &str) -> Result<()> {
    let out = get_shell_command_output(
        "git",
        &[&"checkout", &commit],
        Some(&config::AREPO_PATH),
        false,
    );
    match out.success {
        true => Ok(()),
        false => Err(anyhow!("Failed to check out arepo commit {}", commit)),
    }
}

fn write_systype_file(systype: &Option<Systype>) -> Result<()> {
    let systype_file = &AREPO_PATH.join("Makefile.systype");
    let current_contents = read_file_contents(systype_file)?;
    let default_systype = DEFAULT_SYSTYPE.clone();
    let new_contents = match systype {
        None => DEFAULT_SYSTYPE.clone(),
        Some(option) => match option {
            Systype::Asan => format!("{}{}", &default_systype, "Asan"),
            Systype::Gprof => format!("{}{}", &default_systype, "Gprof"),
        },
    };
    let new_contents = format!("SYSTYPE=\"{}\"", new_contents);
    if current_contents != new_contents {
        write_file(systype_file, &new_contents)
    } else {
        Ok(())
    }
}

fn copy_source_code_to_output(arepo_path: &Utf8Path, path: &Utf8Path) -> Result<()> {
    copy_recursive(
        arepo_path.join(config::DEFAULT_AREPO_SOURCE_FOLDER),
        path.join(config::DEFAULT_AREPO_SOURCE_FOLDER),
    )
}

fn build_arepo(verbose: bool) -> Result<()> {
    delete_arepoconfig_header_file_if_present(&AREPO_PATH)?;
    let out = get_shell_command_output(
        "make",
        &[
            &"-j",
            &config::DEFAULT_NUM_CORES_TO_COMPILE.to_string().as_ref(),
        ],
        Some(&AREPO_PATH),
        verbose,
    );
    if !out.success {
        println!("{}", out.stdout);
        println!("{}", out.stderr);
        return Err(anyhow!("Arepo compilation failed!"));
    }
    copy_arepoconfig_header_file(&AREPO_PATH) // For clang to make sense of the situation
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

fn copy_config_file(sim: &SimParams) -> Result<()> {
    let source = get_config_file_path(&sim.folder);
    let target = AREPO_PATH.join(config::DEFAULT_CONFIG_FILE_NAME);
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

fn copy_arepo_file(sim: &SimParams) -> Result<()> {
    let source = AREPO_PATH.join(config::DEFAULT_AREPO_EXECUTABLE_NAME);
    let target = sim.folder.join(config::DEFAULT_AREPO_EXECUTABLE_NAME);
    copy_file(source, target)
}
