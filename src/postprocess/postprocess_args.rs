use clap::Clap;

use super::post_fn_name::PostFnName;
use camino::Utf8PathBuf;

/// Run the postprocessing scripts
#[derive(Clap, Debug)]
pub struct PostprocessArgs {
    #[clap(long, global = true)]
    pub show: bool,
    #[clap(short, long, global = true)]
    pub showall: bool,
    pub output_folder: Utf8PathBuf,
    pub plot_template: Option<String>,
    #[clap(subcommand)]
    pub function: PostFnName,
}
