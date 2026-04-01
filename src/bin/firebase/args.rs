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
    Firestore {
        #[command(subcommand)]
        command: FirestoreCommand,
    },
}

#[derive(Subcommand)]
pub enum FirestoreCommand {
    Get(FirestoreGetArgs),
    Patch(FirestorePatchArgs),
}

#[derive(Args)]
pub struct FirestoreGetArgs {
    /// Firestore project ID
    #[arg(short = 'p', long = "project")]
    pub project: String,

    /// Firebase ID token
    #[arg(short = 't', long = "token")]
    pub auth_token: String,

    /// Document path e.g. users/uid123
    #[arg(short = 'd', long = "doc")]
    pub path: String,
}

#[derive(Args)]
pub struct FirestorePatchArgs {
    #[arg(short = 'p', long = "project")]
    pub project: String,

    #[arg(short = 't', long = "token")]
    pub auth_token: String,

    #[arg(short = 'd', long = "doc")]
    pub path: String,

    /// Field path e.g. "name" or "address.city"
    #[arg(short = 'f', long = "field")]
    pub field_path: String,

    /// Field type: stringValue | integerValue | doubleValue | booleanValue | nullValue
    #[arg(long = "type")]
    pub field_type: String,

    /// Field value
    #[arg(long = "value")]
    pub field_value: String,
}

#[derive(Subcommand)]
pub enum AuthCommand {
    RefreshToken(RefreshTokenArgs),
    EmailPassSignIn(EmailPassSignInArgs),
    EmailPassSignUp(EmailPassSignUpArgs),
}

#[derive(Args)]
pub struct EmailPassSignUpArgs {
    #[arg(help = "Email Address")]
    pub email: String,
    #[arg(help = "Password")]
    pub password: String,
}

#[derive(Args)]
pub struct EmailPassSignInArgs {
    #[arg(help = "Email Address")]
    pub email: String,
    #[arg(help = "Password")]
    pub password: String,
}

#[derive(Args)]
pub struct RefreshTokenArgs {
    #[arg(short = 't', long = "token")]
    pub refresh_token: String,
}
