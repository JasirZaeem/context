use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap()]
pub struct ContextArgs {
    #[clap(short, long)]
    pub pwd: Option<PathBuf>,

    #[clap(short, long)]
    pub config: Option<PathBuf>,

    #[clap(default_value = "")]
    pub operation: Vec<String>,
}