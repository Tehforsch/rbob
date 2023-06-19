use anyhow::anyhow;
use anyhow::Result;

use crate::config;
use crate::config::RAXIOM_BUILD_PATH;
use crate::config::RAXIOM_PATH;
use crate::sim_params::SimParams;
use crate::sim_set::SimSet;
use crate::systype::Systype;
use crate::util::copy_file;
use crate::util::get_shell_command_output;

pub fn build_sim_set(
    sim_set: &SimSet,
    verbose: bool,
    systype: &Option<Systype>,
    debug_build: bool,
) -> Result<()> {
    for (i, sim) in sim_set.enumerate() {
        println!("Building sim {}", i);
        build_sim(sim, verbose, systype, debug_build)?;
    }
    Ok(())
}

fn build_sim(
    sim: &SimParams,
    verbose: bool,
    _systype: &Option<Systype>,
    debug_build: bool,
) -> Result<()> {
    build_raxiom(verbose, debug_build)?;
    copy_binary(sim, debug_build)?;
    Ok(())
}

fn build_raxiom(verbose: bool, debug_build: bool) -> Result<()> {
    let out = if debug_build {
        get_shell_command_output(
            "cargo",
            &["build", "--color=always"],
            Some(&RAXIOM_PATH),
            verbose,
        )
    } else {
        get_shell_command_output(
            "cargo",
            &["build", "--release", "--color=always"],
            Some(&RAXIOM_PATH),
            verbose,
        )
    };
    if !out.success {
        println!("{}", out.stdout);
        println!("{}", out.stderr);
        return Err(anyhow!("Compilation failed!"));
    }
    Ok(())
}

fn copy_binary(sim: &SimParams, debug_build: bool) -> Result<()> {
    let path = if debug_build {
        RAXIOM_BUILD_PATH.parent().unwrap().join("debug")
    } else {
        RAXIOM_BUILD_PATH.to_owned()
    };
    let source = path.join(config::DEFAULT_RAXIOM_EXECUTABLE_NAME);
    let target = sim.folder.join(config::DEFAULT_RAXIOM_EXECUTABLE_NAME);
    copy_file(source, target)
}
