// src/main.rs
/*
 * Main executable for SmartChainFlow
 */

use clap::Parser;
use smartchainflow::{Result, run};

#[derive(Parser)]
#[command(version, about = "SmartChainFlow - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
