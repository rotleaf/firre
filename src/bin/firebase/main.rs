use clap::Parser;
use firre::{firebase::auth::requests::core_refresh_token, utils::Ret};

use crate::args::{AuthCommand, Cli, Cmd};

mod args;

fn main() -> Ret<()> {
    let cli = Cli::parse();

    let res = match cli.command {
        Cmd::Auth { command } => match command {
            AuthCommand::RefreshToken(args) => {
                core_refresh_token(&cli.api_key, &args.refresh_token, None)
            }
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
