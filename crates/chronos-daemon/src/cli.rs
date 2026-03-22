use clap::{Parser, Subcommand};

/// Top-level CLI structure.
///
/// **Go Parallel:** In Go, you'd use `cobra` to define your root command.
/// Rust's `clap` does this declaratively.
#[derive(Parser, Debug)]
#[command(name = "chronos", about = "Your personal context engine", version)]
pub struct Cli {
    /// Override the default database URL (e.g., sqlite://:memory:).
    /// If not provided, Chronos uses a platform-specific local data directory.
    #[arg(long, global = true)]
    pub db_url: Option<String>,

    /// The command to execute.
    #[command(subcommand)]
    pub command: Commands,
}

/// Subcommands for the Chronos CLI.
///
/// **Go Parallel:** These are like your subcommands in `cobra`.
#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    /// Start the background capture daemon.
    /// This initiates the continuous screen capture and VLM analysis loop.
    Start,

    /// Query the semantic logs stored in the database.
    Query {
        /// Filter logs from this date/time (YYYY-MM-DD or RFC3339).
        #[arg(long)]
        from: Option<String>,

        /// Filter logs up to this date/time (YYYY-MM-DD or RFC3339).
        #[arg(long)]
        to: Option<String>,

        /// Maximum number of results to return (default: 10).
        #[arg(long, default_value = "10")]
        limit: u64,
    },

    /// Show current system status, database location, and log statistics.
    Status,

    /// Pause the screen capture loop (v0.1: stub).
    Pause,

    /// Resume the screen capture loop (v0.1: stub).
    Resume,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_cli_parse_start() {
        let cli = Cli::parse_from(["chronos", "start"]);
        assert_eq!(cli.command, Commands::Start);
    }

    #[test]
    fn test_cli_parse_query_defaults() {
        let cli = Cli::parse_from(["chronos", "query"]);
        match cli.command {
            Commands::Query { from, to, limit } => {
                assert_eq!(from, None);
                assert_eq!(to, None);
                assert_eq!(limit, 10);
            }
            _ => panic!("Expected Query command"),
        }
    }

    #[test]
    fn test_cli_parse_query_with_dates() {
        let cli = Cli::parse_from(["chronos", "query", "--from", "2023-01-01", "--limit", "50"]);
        match cli.command {
            Commands::Query { from, to, limit } => {
                assert_eq!(from, Some("2023-01-01".to_string()));
                assert_eq!(to, None);
                assert_eq!(limit, 50);
            }
            _ => panic!("Expected Query command"),
        }
    }

    #[test]
    fn test_cli_parse_status() {
        let cli = Cli::parse_from(["chronos", "status"]);
        assert_eq!(cli.command, Commands::Status);
    }
}
