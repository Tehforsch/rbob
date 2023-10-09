use std::error::Error;

use anyhow::Result;
use args::StartSimulation;
use args::SubCommand;
use camino::Utf8Path;
use clap::Parser;
use rbob::build_config::BuildConfig;
use rbob::config::DEFAULT_BOB_CONFIG_NAME;
use rbob::config::DEFAULT_FEATURES;
use rbob::config::DEFAULT_PROFILE;
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
            let build_config = BuildConfig {
                run_example: l.run_example,
                features: DEFAULT_FEATURES.clone(),
                profile: get_profile(l.debug_build),
            };
            build_sim_set(&sim_set, a.verbose, &build_config)?;
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

fn get_profile(debug_build: bool) -> String {
    if debug_build {
        "dev".into()
    } else {
        DEFAULT_PROFILE.clone().unwrap_or("release".into())
    }
}

fn start_sim_set(sim_set: SimSet, args: &StartSimulation, verbose: bool) -> Result<()> {
    let output_sim_set = copy_sim_set(
        &sim_set,
        &args.input_folder,
        &args.output_folder,
        args.delete,
        !args.do_not_symlink_ics,
    )?;
    let build_config = BuildConfig {
        profile: get_profile(args.debug_build),
        run_example: args.run_example.clone(),
        features: DEFAULT_FEATURES.clone(),
    };
    build_sim_set(&output_sim_set, verbose, &build_config)?;
    run_sim_set(&output_sim_set, verbose)
}

fn get_sim_set_from_input(folder: &Utf8Path) -> Result<SimSet> {
    let config_file_path = folder.join(DEFAULT_BOB_CONFIG_NAME);
    SimSet::from_bob_file_and_input_folder(&config_file_path, folder)
}

fn get_sim_set_from_output(folder: &Utf8Path) -> Result<SimSet> {
    SimSet::from_output_folder(folder)
}
