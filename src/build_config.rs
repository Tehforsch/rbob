use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct BuildConfig {
    pub debug_build: bool,
    pub features: Vec<String>,
    #[clap(long)]
    pub run_example: Option<String>,
}
