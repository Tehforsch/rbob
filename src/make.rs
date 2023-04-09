use anyhow::anyhow;
use anyhow::Result;

use crate::config;
use crate::config::RAXIOM_PATH;
use crate::sim_params::SimParams;
use crate::sim_set::SimSet;
use crate::systype::Systype;
use crate::util::copy_file;
use crate::util::get_shell_command_output;

pub fn build_sim_set(sim_set: &SimSet, verbose: bool, systype: &Option<Systype>) -> Result<()> {
    for (i, sim) in sim_set.enumerate() {
        println!("Building sim {}", i);
        build_sim(sim, verbose, systype)?;
    }
    Ok(())
}

fn build_sim(sim: &SimParams, verbose: bool, _systype: &Option<Systype>) -> Result<()> {
    build_raxiom(verbose)?;
    copy_binary(sim)?;
    Ok(())
}

fn build_raxiom(verbose: bool) -> Result<()> {
    let out = get_shell_command_output(
        "cargo",
        &["build", "--release"],
        Some(&RAXIOM_PATH),
        verbose,
    );
    if !out.success {
        println!("{}", out.stdout);
        println!("{}", out.stderr);
        return Err(anyhow!("Compilation failed!"));
    }
    Ok(())
}

fn copy_binary(sim: &SimParams) -> Result<()> {
    let source = RAXIOM_PATH.join(config::DEFAULT_RAXIOM_EXECUTABLE_NAME);
    let target = sim.folder.join(config::DEFAULT_RAXIOM_EXECUTABLE_NAME);
    copy_file(source, target)
}
