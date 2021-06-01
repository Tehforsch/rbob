use bob::postprocess::{postprocess_args::PostprocessArgs, replot_args::ReplotArgs};
use camino::Utf8PathBuf;
use clap::Clap;

/// BoB. The Builder.
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Toni Peter")]
pub struct Opts {
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long)]
    pub verbose: bool,
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap, Debug)]
pub enum SubCommand {
    Show(ShowSimulationInfo),
    Diff(ShowSimulationDiff),
    ShowOutput(ShowSimulationInfoOutput),
    Copy(CopySimulation),
    Build(BuildSimulation),
    Run(RunSimulation),
    Start(StartSimulation),
    Post(PostprocessArgs),
    Plot(PostprocessArgs),
    Replot(ReplotArgs),
}

/// Read the input directory and show info about the resulting simulations.
#[derive(Clap, Debug)]
pub struct ShowSimulationInfo {
    pub folder: Utf8PathBuf,
    pub param_names: Vec<String>,
}

/// Show the difference in the parameters between two simulation directories
#[derive(Clap, Debug)]
pub struct ShowSimulationDiff {
    /// Utf8Path to the first simulation dir
    pub folder1: Utf8PathBuf,
    /// Utf8Path to the second simulation dir
    pub folder2: Utf8PathBuf,
}

/// Read the output directory and show info about the resulting simulations.
#[derive(Clap, Debug)]
pub struct ShowSimulationInfoOutput {
    pub output_folder: Utf8PathBuf,
    pub param_names: Vec<String>,
}

/// Read the input directory and copy/rewrite the simulation files
#[derive(Clap, Debug)]
pub struct CopySimulation {
    pub input_folder: Utf8PathBuf,
    pub output_folder: Utf8PathBuf,
    #[clap(short, long)]
    pub delete: bool,
}

/// Build arepo for each of the configuration files in the output directory
#[derive(Clap, Debug)]
pub struct BuildSimulation {
    pub output_folder: Utf8PathBuf,
}

/// Run each of the simulations in the output directory
#[derive(Clap, Debug)]
pub struct RunSimulation {
    pub output_folder: Utf8PathBuf,
}

/// Copy, Build and Run in one command
#[derive(Clap, Debug)]
pub struct StartSimulation {
    pub input_folder: Utf8PathBuf,
    pub output_folder: Utf8PathBuf,
    #[clap(short, long)]
    pub delete: bool,
}
