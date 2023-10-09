use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct BuildConfig {
    pub features: Vec<String>,
    #[clap(long)]
    pub run_example: Option<String>,
    pub profile: String,
}
