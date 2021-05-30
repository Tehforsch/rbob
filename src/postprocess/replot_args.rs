use camino::Utf8PathBuf;
use clap::Clap;

#[derive(Clap, Debug)]
pub struct ReplotArgs {
    pub folder: Utf8PathBuf,
}
