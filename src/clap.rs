//! Integration with `clap`

use std::path::PathBuf;

use clap::Parser;

/// Commandline arguments
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Args {
	#[arg(short, long)]
	/// Optional argument to the path of a conduwuit config TOML file
	pub config: Option<PathBuf>,
}

/// Parse commandline arguments into structured data
pub fn parse() -> Args { Args::parse() }
