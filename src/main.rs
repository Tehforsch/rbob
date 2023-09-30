use std::error::Error;

use anyhow::Result;
use args::StartSimulation;
use args::SubCommand;
use camino::Utf8Path;
use clap::Parser;
use rbob::config::DEFAULT_BOB_CONFIG_NAME;
use rbob::copy::copy_sim_set;
use rbob::make::build_sim_set;
use rbob::run::run_sim_set;
use rbob::sim_set::SimSet;

use self::args::Opts;

pub mod args;

fn main() -> Result<(), Box<dyn Error>> {
    let a = Opts::parse();
    match a.subcmd {
        SubCommand::Copy(l) => {
            let sim_set = get_sim_set_from_input(&l.input_folder)?;
            copy_sim_set(
                &sim_set,
                l.input_folder,
                l.output_folder,
                l.delete,
                !l.do_not_symlink_ics,
            )?;
        }
        SubCommand::Build(l) => {
            let sim_set = get_sim_set_from_output(&l.output_folder)?;
            build_sim_set(&sim_set, a.verbose, &l.systype, l.debug_build, &None)?;
        }
        SubCommand::Run(l) => {
            let sim_set = get_sim_set_from_output(&l.output_folder)?;
            run_sim_set(&sim_set, a.verbose)?;
        }
        SubCommand::Start(l) => {
            let sim_set = get_sim_set_from_input(&l.input_folder)?;
            start_sim_set(sim_set, &l, a.verbose)?;
        }
    }
    Ok(())
}

fn start_sim_set(sim_set: SimSet, args: &StartSimulation, verbose: bool) -> Result<()> {
    let output_sim_set = copy_sim_set(
        &sim_set,
        &args.input_folder,
        &args.output_folder,
        args.delete,
        !args.do_not_symlink_ics,
    )?;
    build_sim_set(
        &output_sim_set,
        verbose,
        &args.systype,
        args.debug_build,
        &args.run_example,
    )?;
    run_sim_set(&output_sim_set, verbose)
}

fn get_sim_set_from_input(folder: &Utf8Path) -> Result<SimSet> {
    let config_file_path = folder.join(DEFAULT_BOB_CONFIG_NAME);
    SimSet::from_bob_file_and_input_folder(&config_file_path, folder)
}

fn get_sim_set_from_output(folder: &Utf8Path) -> Result<SimSet> {
    SimSet::from_output_folder(folder)
}
