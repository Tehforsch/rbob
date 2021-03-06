use clap::Clap;
use std::path::PathBuf;

use super::post_fn_name::PostFnName;

/// Run the postprocessing scripts
#[derive(Clap, Debug)]
pub struct PostprocessArgs {
    #[clap(short, long)]
    pub show: bool,
    pub output_folder: PathBuf,
    #[clap(subcommand)]
    pub function: PostFnName,
}
