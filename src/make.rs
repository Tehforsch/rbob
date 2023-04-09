use anyhow::anyhow;
use anyhow::Result;

use crate::config;
use crate::config::DEFAULT_SYSTYPE;
use crate::config::RAXIOM_PATH;
use crate::sim_params::SimParams;
use crate::sim_set::SimSet;
use crate::systype::Systype;
use crate::util::copy_file;
use crate::util::get_shell_command_output;
use crate::util::read_file_contents;
use crate::util::write_file;

pub fn build_sim_set(sim_set: &SimSet, verbose: bool, systype: &Option<Systype>) -> Result<()> {
    for (i, sim) in sim_set.enumerate() {
        println!("Building sim {}", i);
        build_sim(sim, verbose, systype)?;
    }
    Ok(())
}

fn build_sim(sim: &SimParams, verbose: bool, systype: &Option<Systype>) -> Result<()> {
    write_systype_file(systype)?;
    if let Some(commit) = sim.get("arepoCommit") {
        checkout_arepo_commit(commit.unwrap_string())?;
    }
    build_arepo(verbose)?;
    copy_binary(sim)?;
    Ok(())
}

fn checkout_arepo_commit(commit: &str) -> Result<()> {
    let out = get_shell_command_output(
        "git",
        &[&"checkout", &commit],
        Some(&config::RAXIOM_PATH),
        false,
    );
    match out.success {
        true => Ok(()),
        false => Err(anyhow!("Failed to check out arepo commit {}", commit)),
    }
}

fn write_systype_file(systype: &Option<Systype>) -> Result<()> {
    let systype_file = &RAXIOM_PATH.join("Makefile.systype");
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

fn build_arepo(verbose: bool) -> Result<()> {
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
