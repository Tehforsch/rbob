use bob::postprocess::postprocess_args::PostprocessArgs;
use bob::postprocess::replot_args::ReplotArgs;
use bob::systype::Systype;
use camino::Utf8PathBuf;
use clap::Clap;

/// BoB. The Builder.
#[derive(Clap, Debug)]
#[clap(version = "0.1.0", author = "Toni Peter")]
pub struct Opts {
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, global = true)]
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
    GetData(GetData),
    CopyAbundances(CopyAbundances),
    Gui(GuiArgs),
}

/// Read the input directory and show info about the resulting simulations.
#[derive(Clap, Debug)]
pub struct ShowSimulationInfo {
    pub folder: Utf8PathBuf,
    pub param_names: Vec<String>,
    #[clap(short, long)]
    pub all: bool,
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
    #[clap(short, long)]
    pub all: bool,
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
    pub systype: Option<Systype>,
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
    pub systype: Option<Systype>,
}

/// Copy all the relevant files (snapshots and parameter files) from a simulation to another dir
#[derive(Clap, Debug)]
pub struct GetData {
    pub source_folder: Utf8PathBuf,
    pub target_folder: Utf8PathBuf,
}

/// Combine two snapshots into one by using the abundances from the first
/// and everything else from the second. The abundances will be set by a
/// nearest neighbour lookup: For each cell in the second snapshot, the
/// abundances are set to the value of the abundances in the closest cell
/// in the first snapshot.
/// The result is written into a third file
#[derive(Clap, Debug)]
pub struct CopyAbundances {
    pub sim_abundances: Utf8PathBuf,
    pub sim_coordinates: Utf8PathBuf,
    pub snap_output: Utf8PathBuf,
}

/// Run the Gui
#[derive(Clap, Debug)]
pub struct GuiArgs {
    pub path: Utf8PathBuf,
}
