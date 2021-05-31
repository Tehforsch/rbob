use self::args::Opts;
use bob::make::build_sim_set;
use bob::postprocess::postprocess_sim_set;
use bob::run::run_sim_set;
use bob::{config, diff, param_value::ParamValue, unit_utils::nice_time};
use bob::{config::DEFAULT_BOB_CONFIG_NAME, config_file::ConfigFile};
use bob::{copy::copy_sim_set, postprocess::plot::replot};

use anyhow::{anyhow, Result};
use args::SubCommand;
use bob::sim_params::SimParams;
use bob::sim_set::SimSet;
use camino::Utf8Path;
use clap::Clap;
use std::error::Error;

pub mod args;

fn main() -> Result<(), Box<dyn Error>> {
    let a = Opts::parse();
    let config_file = ConfigFile::read()?.expanduser()?;
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
            copy_sim_set(&sim_set, l.input_folder, l.output_folder, l.delete)?;
        }
        SubCommand::Build(l) => {
            let sim_set = get_sim_set_from_output(&l.output_folder)?;
            build_sim_set(&sim_set, a.verbose)?;
        }
        SubCommand::Run(l) => {
            let sim_set = get_sim_set_from_output(&l.output_folder)?;
            run_sim_set(&sim_set, a.verbose)?;
        }
        SubCommand::Start(l) => {
            let sim_set = get_sim_set_from_input(&l.input_folder)?;
            start_sim_set(
                sim_set,
                &l.input_folder,
                &l.output_folder,
                l.delete,
                a.verbose,
            )?;
        }
        SubCommand::Post(l) => {
            let sim_set = get_sim_set_from_output(&l.output_folder)?;
            postprocess_sim_set(&config_file, &sim_set, &l)?;
        }
        SubCommand::Replot(l) => {
            replot(&config_file, &l)?;
        }
    }
    Ok(())
}

fn start_sim_set(
    sim_set: SimSet,
    input_folder: &Utf8Path,
    output_folder: &Utf8Path,
    delete: bool,
    verbose: bool,
) -> Result<()> {
    let output_sim_set = copy_sim_set(&sim_set, input_folder, output_folder, delete)?;
    build_sim_set(&output_sim_set, verbose)?;
    run_sim_set(&output_sim_set, verbose)
}

fn print_param_value(param: &str, value: &ParamValue) {
    println!("\t{}: {:?}", param, value);
}

fn show_sim_set(sim_set: SimSet, param_names: &[String]) -> Result<()> {
    let print_param = |sim: &SimParams, param: &str| print_param_value(param, &sim[param]);
    for (i, sim) in sim_set.enumerate() {
        println!("{}:", i);
        if param_names.is_empty() {
            for param in sim.keys() {
                print_param(sim, param)
            }
        } else {
            for param in param_names.iter() {
                if config::CALC_PARAMS.contains(&param.as_ref()) {
                    print_calc_param(sim, param);
                }
                if !sim.contains_key(param) {
                    return Err(anyhow!(
                        "Parameter {} not present in parameter files!",
                        param
                    ));
                }
                print_param(sim, param)
            }
        }
    }
    Ok(())
}

fn print_calc_param(sim: &SimParams, param: &str) {
    match param {
        "timeUnit" => {
            let value = nice_time(sim.units.length / sim.units.velocity);
            print_param_value(param, &ParamValue::Str(value));
        }
        _ => unreachable!(),
    }
}

fn get_sim_set_from_input(folder: &Utf8Path) -> Result<SimSet> {
    let config_file_path = folder.join(DEFAULT_BOB_CONFIG_NAME);
    SimSet::from_bob_file_and_input_folder(&config_file_path, folder)
}

fn get_sim_set_from_output(folder: &Utf8Path) -> Result<SimSet> {
    SimSet::from_output_folder(folder)
}
