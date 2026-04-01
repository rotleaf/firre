use clap::Parser;
use firre::{firebase::auth::requests::*, utils::Ret};

use crate::args::{AuthCommand, Cli, Cmd, FirestoreCommand};

mod args;

fn main() -> Ret<()> {
    let cli = Cli::parse();

    let res = match cli.command {
        Cmd::Auth { command } => match command {
            AuthCommand::RefreshToken(args) => {
                core_refresh_token(&cli.api_key, &args.refresh_token, None)
            }
            AuthCommand::EmailPassSignIn(args) => {
                core_email_pwd_sign_in(&cli.api_key, &args.email, &args.password, None)
            }
            AuthCommand::EmailPassSignUp(args) => {
                core_email_pwd_sign_up(&cli.api_key, &args.email, &args.password, None)
            }
        },
        Cmd::Firestore { command } => match command {
            FirestoreCommand::Get(args) => firre::firebase::firestore::requests::core_get(
                &args.auth_token,
                &args.project,
                &args.path,
                None,
            ),
            FirestoreCommand::Patch(args) => firre::firebase::firestore::requests::core_patch(
                &args.auth_token,
                &args.project,
                &args.path,
                &args.field_type,
                &args.field_path,
                &args.field_value,
                None,
            ),
        },
    };

    match res {
        Ok(js) => println!("{js}"),
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
    Ok(())
}
