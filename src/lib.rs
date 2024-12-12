#![allow(clippy::arc_with_non_send_sync)]
#![deny(clippy::unwrap_used, unused_variables)]

use std::sync::{Arc, Mutex};

use nvim_oxi::{
    api::{create_user_command, err_writeln, opts::CreateCommandOpts, types::*},
    Dictionary, Error as OxiError, Function, Result as OxiResult,
};

use config::Config;
use error::PluginError;

use self::{
    command::{completion, Command},
    core::App,
};

mod command;
mod config;
mod core;
mod crypt;
mod error;
#[cfg(feature = "mail")]
mod mail;

#[nvim_oxi::plugin]
fn just() -> OxiResult<Dictionary> {
    let config = Config::default();

    let app = Arc::new(Mutex::new(App::new(config)));

    let just_cmd = {
        let app_handle_cmd = Arc::clone(&app);

        move |args: CommandArgs| -> OxiResult<()> {
            let binding = match args.args {
                Some(a) => a,
                None => "".to_string(),
            };

            let mut split_args = binding.split_whitespace();
            let action = split_args.next().unwrap_or("").to_string();
            let argument = split_args.next().map(|s| s.to_string());

            let command = Command::from_str(&action, argument.as_deref());

            match command {
                Some(command) => {
                    if let Ok(mut app_lock) = app_handle_cmd.lock() {
                        app_lock.handle_command(command)?;
                    } else {
                        err_writeln("Failed to acquire lock on app");
                    }
                }
                None => err_writeln(&format!("Unknown command: {}", action)),
            };
            Ok(())
        }
    };

    let opts = CreateCommandOpts::builder()
        .desc("Just command")
        .complete(CommandComplete::CustomList(completion()))
        .nargs(CommandNArgs::Any)
        .build();

    create_user_command("Just", just_cmd, &opts)?;

    let app_setup = Arc::clone(&app);
    let exports: Dictionary =
        Dictionary::from_iter::<[(&str, Function<Dictionary, Result<(), OxiError>>); 1]>([(
            "setup",
            Function::from_fn(move |dict: Dictionary| -> OxiResult<()> {
                match app_setup.lock() {
                    Ok(mut app) => app.setup(dict),
                    Err(e) => {
                        err_writeln(&format!(
                            "Failed to acquire lock on app during setup: {}",
                            e
                        ));
                        Err(PluginError::Custom("Lock error during setup".into()).into())
                    }
                }
            }),
        )]);

    Ok(exports)
}
