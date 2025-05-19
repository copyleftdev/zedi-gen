//! Command-line interface for zedi-gen

use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// Command-line interface for zedi-gen
#[derive(Parser, Debug)]
#[command(
    name = "zedi-gen",
    version,
    about = "Generate synthetic X12 835 healthcare claim data and check conformance",
    long_about = None,
    subcommand_required = true,
)]
pub struct Cli {
    /// Subcommand to execute
    #[command(subcommand)]
    pub command: Command,
}
/// Commands supported by zedi-gen
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Generate synthetic X12 835 healthcare claim data
    Generate(GenerateArgs),
    /// Score conformance of an X12 835 file against the spec
    Conformance(ConformanceArgs),
}

/// Arguments for the generate subcommand
#[derive(Args, Debug)]
pub struct GenerateArgs {
    /// Number of claims to generate
    #[arg(short, long, default_value_t = 1000)]
    pub count: usize,

    /// Anomaly injection rate (0.0 to 100.0)
    #[arg(short, long, default_value_t = 1.0)]
    pub anomaly_rate: f64,

    /// Output file (default: stdout)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Random seed for reproducible output
    #[arg(long)]
    pub seed: Option<u64>,

    /// Data directory for CSV files for realistic generation
    #[arg(long, default_value = "data")]
    pub data_dir: PathBuf,

    /// Output format
    #[arg(long, value_enum, default_value_t = OutputFormat::X12)]
    pub format: OutputFormat,
}

/// Arguments for the conformance subcommand
#[derive(Args, Debug)]
pub struct ConformanceArgs {
    /// Input X12 835 file to check for conformance
    #[arg()]
    pub input_path: PathBuf,
}

/// Supported output formats for the generate command
#[derive(ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    /// X12 EDI format
    X12,
    /// JSON format
    Json,
    /// Pretty-printed JSON (for debugging)
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
