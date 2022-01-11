use camino::Utf8PathBuf;
use clap::Clap;

use super::post_fn_name::PostFnName;

/// Run the postprocessing scripts
#[derive(Clap, Debug)]
pub struct PostprocessArgs {
    #[clap(short, long, global = true)]
    pub showall: bool,
    #[clap(long, global = true)]
    pub select: Option<Vec<usize>>,
    pub output_folders: Vec<Utf8PathBuf>,
    #[clap(long)]
    pub plot_template: Option<String>,
    #[clap(subcommand)]
    pub function: PostFnName,
}
