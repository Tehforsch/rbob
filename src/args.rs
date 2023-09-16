use camino::Utf8PathBuf;
use clap::Parser;
use rbob::systype::Systype;

/// Rbob. The Builder for subsweep
#[derive(Parser, Debug)]
#[clap(version = "0.1.0", author = "Toni Peter")]
pub struct Opts {
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, global = true)]
    pub verbose: bool,
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    Copy(CopySimulation),
    Build(BuildSimulation),
    Run(RunSimulation),
    Start(StartSimulation),
}

/// Read the input directory and copy/rewrite the simulation files
#[derive(Parser, Debug)]
pub struct CopySimulation {
    pub input_folder: Utf8PathBuf,
    pub output_folder: Utf8PathBuf,
    #[clap(short, long)]
    pub delete: bool,
    #[clap(short, long)]
    pub symlink_ics: bool,
}

/// Build arepo for each of the configuration files in the output directory
#[derive(Parser, Debug)]
pub struct BuildSimulation {
    pub output_folder: Utf8PathBuf,
    pub systype: Option<Systype>,
    #[clap(short, long)]
    pub debug_build: bool,
}

/// Run each of the simulations in the output directory
#[derive(Parser, Debug)]
pub struct RunSimulation {
    pub output_folder: Utf8PathBuf,
}

/// Copy, Build and Run in one command
#[derive(Parser, Debug)]
pub struct StartSimulation {
    pub input_folder: Utf8PathBuf,
    pub output_folder: Utf8PathBuf,
    #[clap(short, long)]
    pub delete: bool,
    pub systype: Option<Systype>,
    #[clap(short, long)]
    pub symlink_ics: bool,
    #[clap(long)]
    pub debug_build: bool,
    #[clap(long)]
    pub run_example: Option<String>,
}
