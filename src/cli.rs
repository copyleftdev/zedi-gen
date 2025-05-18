//! Command-line interface for zedi-gen

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

/// Command-line interface for zedi-gen
#[derive(Parser, Debug)]
#[command(
    name = "zedi-gen",
    version,
    about = "Generate synthetic X12 835 healthcare claim data",
    long_about = None
)]
pub struct Cli {
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
    
    /// Output format
    #[arg(long, value_enum, default_value_t = OutputFormat::X12)]
    pub format: OutputFormat,
}

/// Supported output formats
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
    
    #[test]
    fn test_cli_parsing() {
        let cli = Cli::parse_from(["zedi-gen", "--count", "100", "--anomaly-rate", "5.0"]);
        assert_eq!(cli.count, 100);
        assert_eq!(cli.anomaly_rate, 5.0);
        assert!(cli.output.is_none());
    }
}
