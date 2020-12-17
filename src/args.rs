use clap::Clap;
use std::path::PathBuf;

/// BoB. The Builder.
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Toni Peter")]
pub struct Opts {
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: i32,
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap, Debug)]
pub enum SubCommand {
    Show(ShowSimulationInfo),
    ShowOutput(ShowSimulationInfoOutput),
    Copy(CopySimulation),
    Build(BuildSimulation),
}

/// Read the input directory and show info about the resulting simulations.
#[derive(Clap, Debug)]
pub struct ShowSimulationInfo {
    pub folder: PathBuf,
    pub param_names: Vec<String>,
}

/// Read the output directory and show info about the resulting simulations.
#[derive(Clap, Debug)]
pub struct ShowSimulationInfoOutput {
    pub output_folder: PathBuf,
    pub param_names: Vec<String>,
}

/// Read the input directory and copy/rewrite the simulation files
#[derive(Clap, Debug)]
pub struct CopySimulation {
    pub input_folder: PathBuf,
    pub output_folder: PathBuf,
}

/// Read the input directory and copy/rewrite the simulation files
#[derive(Clap, Debug)]
pub struct BuildSimulation {
    pub output_folder: PathBuf,
}
