use std::error::Error;

use anyhow::anyhow;
use anyhow::Result;
use args::StartSimulation;
use args::SubCommand;
use bob::config;
use bob::config::DEFAULT_BOB_CONFIG_NAME;
use bob::copy::copy_sim_set;
use bob::copy_abundances::copy_abundances;
use bob::diff;
use bob::get_data::get_data;
use bob::make::build_sim_set;
use bob::param_value::ParamValue;
use bob::postprocess::plot::replot;
use bob::postprocess::postprocess_sim_set;
use bob::run::run_sim_set;
use bob::sim_params::SimParams;
use bob::sim_set::SimSet;
use bob::unit_utils::nice_time;
use camino::Utf8Path;
use camino::Utf8PathBuf;
use clap::Clap;

use self::args::Opts;

pub mod args;

fn main() -> Result<(), Box<dyn Error>> {
    let a = Opts::parse();
    match a.subcmd {
        SubCommand::Show(l) => {
            let sim_set = get_sim_set_from_input(&l.folder)?;
            show_sim_set(sim_set, &l.param_names, l.all)?;
        }
        SubCommand::Diff(l) => {
            diff::show_sim_diff(&l.folder1, &l.folder2)?;
        }
        SubCommand::ShowOutput(l) => {
            let sim_set = get_sim_set_from_output(&l.output_folder)?;
            show_sim_set(sim_set, &l.param_names, l.all)?;
        }
        SubCommand::Copy(l) => {
            let sim_set = get_sim_set_from_input(&l.input_folder)?;
            copy_sim_set(
                &sim_set,
                l.input_folder,
                l.output_folder,
                l.delete,
                l.symlink_ics,
            )?;
        }
        SubCommand::Build(l) => {
            let sim_set = get_sim_set_from_output(&l.output_folder)?;
            build_sim_set(&sim_set, a.verbose, &l.systype)?;
        }
        SubCommand::Run(l) => {
            let sim_set = get_sim_set_from_output(&l.output_folder)?;
            run_sim_set(&sim_set, a.verbose)?;
        }
        SubCommand::Start(l) => {
            let sim_set = get_sim_set_from_input(&l.input_folder)?;
            start_sim_set(sim_set, &l, a.verbose)?;
        }
        SubCommand::GetData(l) => {
            get_data(&l.source_folder, &l.target_folder)?;
        }
        SubCommand::Post(l) => {
            let sim_set = get_sim_set_from_multiple_outputs(&l.output_folders)?;
            postprocess_sim_set(false, sim_set, &l)?;
        }
        SubCommand::Plot(l) => {
            let sim_set = get_sim_set_from_multiple_outputs(&l.output_folders)?;
            postprocess_sim_set(true, sim_set, &l)?;
        }
        SubCommand::Replot(l) => {
            replot(&l)?;
        }
        SubCommand::CopyAbundances(l) => {
            copy_abundances(&l.sim_abundances, &l.sim_coordinates, &l.snap_output)?;
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
        args.symlink_ics,
    )?;
    build_sim_set(&output_sim_set, verbose, &args.systype)?;
    run_sim_set(&output_sim_set, verbose)
}

fn print_param_value(param: &str, value: &ParamValue) {
    println!("\t{}: {:?}", param, value);
}

fn show_sim_set(sim_set: SimSet, param_names: &[String], all: bool) -> Result<()> {
    let print_param = |sim: &SimParams, param: &str| print_param_value(param, &sim[param]);
    for (i, sim) in sim_set.enumerate() {
        println!("{}:", i);
        if param_names.is_empty() {
            for param in sim.keys() {
                if all || sim_set.varies(param) {
                    print_param(sim, param)
                }
            }
        } else {
            for param in param_names.iter() {
                if config::CALC_PARAMS.contains(&param.as_ref()) {
                    print_calc_param(sim, param);
                } else if sim.contains_key(param) {
                    print_param(sim, param)
                } else {
                    return Err(anyhow!(
                        "Parameter {} not present in parameter files!",
                        param
                    ));
                }
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

fn get_sim_set_from_multiple_outputs(folders: &[Utf8PathBuf]) -> Result<SimSet> {
    let sim_sets: Result<Vec<_>> = folders
        .iter()
        .map(|folder| SimSet::from_output_folder(folder))
        .collect();
    Ok(SimSet::join(sim_sets?.into_iter()))
}
