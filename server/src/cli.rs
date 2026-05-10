use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Token management.
    Token {
        #[command(subcommand)]
        cmd: TokenCmd,
    },
}

#[derive(Subcommand)]
pub enum TokenCmd {
    /// Generate a token, print plaintext, and exit.
    Issue {
        /// admin = full access (used by CLI/admin tools).
        /// user  = data access only (used by browser extension/clients).
        role: RoleArg,
        /// Human-readable label (shown in `token list` later).
        #[arg(long)]
        name: String,
        /// Optional TTL in days. Omitted = never expires.
        #[arg(long)]
        ttl_days: Option<i64>,
    },
}

#[derive(Copy, Clone, ValueEnum)]
pub enum RoleArg {
    Admin,
    User,
}
