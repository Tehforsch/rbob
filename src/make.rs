use anyhow::anyhow;
use anyhow::Result;

use crate::build_config::BuildConfig;
use crate::config;
use crate::config::SUBSWEEP_BUILD_PATH;
use crate::config::SUBSWEEP_PATH;
use crate::sim_params::SimParams;
use crate::sim_set::SimSet;
use crate::util::copy_file;
use crate::util::get_shell_command_output;

pub fn build_sim_set(sim_set: &SimSet, verbose: bool, build_config: &BuildConfig) -> Result<()> {
    build_subsweep(verbose, build_config)?;
    // We copy to the parent folder of the simulation once
    for sim in sim_set.iter().take(1) {
        copy_binary(sim, build_config)?;
    }
    Ok(())
}

fn build_subsweep(verbose: bool, build_config: &BuildConfig) -> Result<()> {
    let mut args: Vec<String> = vec!["build".into(), "--color=always".into()];
    if !build_config.debug_build {
        args.push("--release".into());
    }
    if let Some(run_example) = &build_config.run_example {
        args.push("--example".into());
        args.push(run_example.into());
    }
    if !build_config.features.is_empty() {
        args.push("--features".into());
        args.push(build_config.features.join(","));
    }
    if verbose {
        let build_command = format!("cargo {}", args.join(" "));
        println!("Building with: '{}'", build_command);
    }
    let out = get_shell_command_output("cargo", &args, Some(&SUBSWEEP_PATH), verbose);
    if !out.success {
        println!("{}", out.stdout);
        println!("{}", out.stderr);
        return Err(anyhow!("Compilation failed!"));
    }
    Ok(())
}

fn copy_binary(sim: &SimParams, build_config: &BuildConfig) -> Result<()> {
    let path = if build_config.debug_build {
        SUBSWEEP_BUILD_PATH.parent().unwrap().join("debug")
    } else {
        SUBSWEEP_BUILD_PATH.to_owned()
    };
    let source = if let Some(run_example) = &build_config.run_example {
        path.join("examples").join(run_example)
    } else {
        path.join(config::DEFAULT_SUBSWEEP_EXECUTABLE_NAME)
    };
    let target = sim
        .folder
        .parent()
        .unwrap()
        .join(config::DEFAULT_SUBSWEEP_EXECUTABLE_NAME);
    copy_file(source, target)
}
