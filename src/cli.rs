

use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;


#[derive(Parser, Debug)]
#[command(
    name = "zedi-gen",
    version,
    about = "Generate synthetic X12 835 healthcare claim data and check conformance",
    long_about = None,
    subcommand_required = true,
)]
pub struct Cli {
    
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    
    Generate(GenerateArgs),
    
    Conformance(ConformanceArgs),
}


#[derive(Args, Debug)]
pub struct GenerateArgs {
    
    #[arg(short, long, default_value_t = 1000)]
    pub count: usize,

    
    #[arg(short, long, default_value_t = 1.0)]
    pub anomaly_rate: f64,

    
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    
    #[arg(long)]
    pub seed: Option<u64>,

    
    #[arg(long, default_value = "data")]
    pub data_dir: PathBuf,

    
    #[arg(long, value_enum, default_value_t = OutputFormat::X12)]
    pub format: OutputFormat,
}


#[derive(Args, Debug)]
pub struct ConformanceArgs {
    
    #[arg()]
    pub input_path: PathBuf,
}


#[derive(ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    
    X12,
    
    Json,
    
    JsonPretty,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_cli_generate_parsing() {
        let cli = Cli::parse_from([
            "zedi-gen",
            "generate",
            "--count",
            "100",
            "--anomaly-rate",
            "5.0",
        ]);
        match cli.command {
            Command::Generate(args) => {
                assert_eq!(args.count, 100);
                assert_eq!(args.anomaly_rate, 5.0);
                assert!(args.output.is_none());
                assert_eq!(args.data_dir, PathBuf::from("data"));
            }
            _ => panic!("Expected Generate command"),
        }
    }

    #[test]
    fn test_cli_conformance_parsing() {
        let cli = Cli::parse_from(["zedi-gen", "conformance", "file.edi"]);
        match cli.command {
            Command::Conformance(args) => {
                assert_eq!(args.input_path, PathBuf::from("file.edi"));
            }
            _ => panic!("Expected Conformance command"),
        }
    }
}
