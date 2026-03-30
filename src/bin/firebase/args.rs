use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "firre")]
pub struct Cli {
    #[arg(short = 'k', long = "key")]
    pub api_key: String,
    #[command(subcommand)]
    pub command: Cmd,
}

#[derive(Subcommand)]
pub enum Cmd {
    Auth {
        #[command(subcommand)]
        command: AuthCommand,
    },
}

#[derive(Subcommand)]
pub enum AuthCommand {
    RefreshToken(RefreshTokenArgs),
}

#[derive(Args)]
pub struct RefreshTokenArgs {
    #[arg(short = 't', long = "token")]
    pub refresh_token: String,
}
