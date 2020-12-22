pub mod args;
pub mod build;
pub mod config;
pub mod copy;
pub mod job_params;
pub mod param_value;
pub mod postprocess;
pub mod run;
pub mod sim_params;
pub mod sim_set;
pub mod sim_units;
pub mod unit_array;
pub mod util;

use crate::args::Opts;
use crate::build::build_sim_set;
use crate::config::DEFAULT_BOB_CONFIG_NAME;
use crate::copy::copy_sim_set;
use crate::postprocess::postprocess_sim_set;
use crate::run::run_sim_set;

use anyhow::{anyhow, Result};
use args::SubCommand;
use clap::Clap;
use sim_params::SimParams;
use sim_set::SimSet;
use std::{error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let a = Opts::parse();
    match a.subcmd {
        SubCommand::Show(l) => {
            let sim_set = get_sim_set_from_input(&l.folder)?;
            show_sim_set(sim_set, &l.param_names).expect("When showing parameters")
        }
        SubCommand::ShowOutput(l) => {
            let sim_set = get_sim_set_from_output(&l.output_folder)?;
            show_sim_set(sim_set, &l.param_names).expect("When showing parameters")
        }
        SubCommand::Copy(l) => {
            let sim_set = get_sim_set_from_input(&l.input_folder)?;
            copy_sim_set(&sim_set, l.input_folder, l.output_folder)
                .expect("When copying simulation");
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
            postprocess_sim_set(&sim_set, l.function)?;
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
