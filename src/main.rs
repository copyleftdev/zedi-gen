//! zedi-gen: Synthetic X12 835 healthcare claim data generator
//!
//! This tool generates realistic, specification-compliant X12 835 claim data
//! for testing and validation of healthcare claims processing systems.

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

use clap::Parser;
use log::{error, info};
use std::process;
use std::time::Instant;

mod anomalies;
mod claims;
mod cli;
mod config;
mod conformance;
mod errors;
mod generator;
mod population;
mod x12;

use crate::{
    cli::{Cli, Command, ConformanceArgs, GenerateArgs},
    config::Config,
    errors::Result,
    generator::Generator,
};

/// Main entry point for zedi-gen
fn main() -> Result<()> {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // Parse command line arguments
    let args = Cli::parse();

    // Run the application
    if let Err(e) = run(args) {
        error!("Error: {}", e);
        process::exit(1);
    }

    Ok(())
}

/// Run the application with the given CLI arguments
fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Command::Generate(gen) => {
            let start_time = Instant::now();

            // Create configuration from generate arguments
            let config = Config::from_cli(&gen);

            info!(
                "Generating {} claims with {:.2}% anomalies...",
                config.claim_count,
                config.anomaly_rate * 100.0
            );

            if let Some(ref path) = config.output_path {
                info!("Output will be written to: {}", path.display());
            } else {
                info!("Output will be written to stdout");
            }

            // Initialize the generator
            let mut generator = Generator::new(config);

            // Generate the claims
            generator.generate()?;

            let elapsed = start_time.elapsed();
            info!(
                "Generation completed in {:.2} seconds",
                elapsed.as_secs_f64()
            );
            Ok(())
        }
        Command::Conformance(conf) => conformance::run(&conf.input_path),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{Command, GenerateArgs, OutputFormat};
    use std::path::PathBuf;

    #[test]
    fn test_run_with_default_args() {
        let gen = GenerateArgs {
            count: 10,
            anomaly_rate: 1.0,
            output: None,
            seed: Some(42),
            data_dir: PathBuf::from("data"),
            format: OutputFormat::Json,
        };
        let cli = Cli {
            command: Command::Generate(gen),
        };

        assert!(run(cli).is_ok());
    }
}
