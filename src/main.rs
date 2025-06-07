




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


fn main() -> Result<()> {
    
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    
    let args = Cli::parse();

    
    if let Err(e) = run(args) {
        error!("Error: {}", e);
        process::exit(1);
    }

    Ok(())
}


fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Command::Generate(gen) => {
            let start_time = Instant::now();

            
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

            
            let mut generator = Generator::new(config);

            
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
