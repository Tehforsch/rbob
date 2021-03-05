use self::args::Opts;
use bob::{config::DEFAULT_BOB_CONFIG_NAME, config_file::ConfigFile};
use bob::copy::copy_sim_set;
use bob::diff;
use bob::make::build_sim_set;
use bob::postprocess::postprocess_sim_set;
use bob::run::run_sim_set;

use anyhow::{anyhow, Result};
use args::SubCommand;
use bob::sim_params::SimParams;
use bob::sim_set::SimSet;
use clap::Clap;
use std::{error::Error, path::Path};

pub mod args;

fn main() -> Result<(), Box<dyn Error>> {
    let a = Opts::parse();
    let config_file = ConfigFile::read()?;
    match a.subcmd {
        SubCommand::Show(l) => {
            let sim_set = get_sim_set_from_input(&l.folder)?;
            show_sim_set(sim_set, &l.param_names)?;
        }
        SubCommand::Diff(l) => {
            diff::show_sim_diff(&l.folder1, &l.folder2)?;
        }
        SubCommand::ShowOutput(l) => {
            let sim_set = get_sim_set_from_output(&l.output_folder)?;
            show_sim_set(sim_set, &l.param_names)?;
        }
        SubCommand::Copy(l) => {
            let sim_set = get_sim_set_from_input(&l.input_folder)?;
            copy_sim_set(&sim_set, l.input_folder, l.output_folder)?;
        }
        SubCommand::Build(l) => {
            let sim_set = get_sim_set_from_output(&l.output_folder)?;
            build_sim_set(&sim_set)?;
        }
        SubCommand::Run(l) => {
            let sim_set = get_sim_set_from_output(&l.output_folder)?;
            run_sim_set(&sim_set)?;
        }
        SubCommand::Start(l) => {
            let sim_set = get_sim_set_from_input(&l.input_folder)?;
            start_sim_set(sim_set, &l.input_folder, &l.output_folder)?;
        }
        SubCommand::Post(l) => {
            let sim_set = get_sim_set_from_output(&l.output_folder)?;
            postprocess_sim_set(&config_file, &sim_set, l.function)?;
        }
    }
    Ok(())
}

fn start_sim_set(sim_set: SimSet, input_folder: &Path, output_folder: &Path) -> Result<()> {
    let output_sim_set = copy_sim_set(&sim_set, input_folder, output_folder)?;
    build_sim_set(&output_sim_set)?;
    run_sim_set(&output_sim_set)
}

fn show_sim_set(sim_set: SimSet, param_names: &Vec<String>) -> Result<()> {
    let print_param = |sim: &SimParams, param: &str| println!("\t{}: {:?}", param, sim[param]);
    for (i, sim) in sim_set.enumerate() {
        println!("{}:", i);
        if param_names.is_empty() {
            for param in sim.keys() {
                print_param(sim, param)
            }
        } else {
            for param in param_names.iter() {
                if !sim.contains_key(param) {
                    return Err(anyhow!("Parameter {} not present in parameter files!"));
                }
                print_param(sim, param)
            }
        }
    }
    Ok(())
}

fn get_sim_set_from_input(folder: &Path) -> Result<SimSet> {
    let config_file_path = folder.join(DEFAULT_BOB_CONFIG_NAME);
    SimSet::from_bob_file_and_input_folder(&config_file_path, folder)
}

fn get_sim_set_from_output(folder: &Path) -> Result<SimSet> {
    SimSet::from_output_folder(folder)
}
