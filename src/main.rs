pub mod args;
pub mod config;
pub mod param_file;
pub mod param_value;
pub mod sim_params;
pub mod simulation_set;

use crate::args::Opts;
use crate::config::DEFAULT_BOB_CONFIG_NAME;

use anyhow::{anyhow, Result};
use args::SubCommand;
use clap::Clap;
use sim_params::SimParams;
use simulation_set::SimSet;
use std::{error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let a = Opts::parse();
    match a.subcmd {
        SubCommand::Show(l) => {
            let sim_set = get_sim_set(&l.folder)?;
            show_sim_set(sim_set, &l.param_names).expect("When showing parameters")
        }
    }
    Ok(())
}

fn show_sim_set(sim_set: SimSet, param_names: &Vec<String>) -> Result<()> {
    let print_param = |sim: &SimParams, param: &str| println!("\t{}: {:?}", param, sim[param]);
    for (i, sim) in sim_set.iter().enumerate() {
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

fn get_sim_set(folder: &Path) -> Result<SimSet> {
    let config_file_path = folder.join(DEFAULT_BOB_CONFIG_NAME);
    SimSet::from_file(&config_file_path, folder)
}
