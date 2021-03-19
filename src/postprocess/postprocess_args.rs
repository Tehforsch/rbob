use clap::Clap;

use super::post_fn_name::PostFnName;
use camino::Utf8PathBuf;

/// Run the postprocessing scripts
#[derive(Clap, Debug)]
pub struct PostprocessArgs {
    #[clap(short, long)]
    pub show: bool,
    #[clap(short, long)]
    pub showall: bool,
    pub output_folder: Utf8PathBuf,
    #[clap(subcommand)]
    pub function: PostFnName,
}
