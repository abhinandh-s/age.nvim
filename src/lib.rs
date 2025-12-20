use std::{cell::RefCell, rc::Rc};

use nvim_oxi::{
    api::{create_user_command, err_writeln, opts::CreateCommandOpts, types::*},
    Dictionary, Function,
};

use config::Config;

use self::{
    command::{completion, Command},
    core::App,
};

mod command;
mod config;
mod core;
mod crypt;
mod error;

#[nvim_oxi::plugin]
fn age() -> Result<Dictionary, nvim_oxi::Error> {
    let config = Config::default();

    let app = Rc::new(RefCell::new(App::new(config)));

    let age_cmd = {
        let app_handle_cmd = Rc::clone(&app);

        move |args: CommandArgs| -> Result<(), nvim_oxi::Error> {
            let binding = match args.args {
                Some(a) => a,
                None => "".to_owned(),
            };

            let mut split_args = binding.split_whitespace();
            let action = split_args.next().unwrap_or("").to_owned();

            let command = Command::from_str(&action);

            match command {
                Some(command) => {
                    app_handle_cmd.borrow_mut().handle_command(command)?;
                }
                None => err_writeln(&format!("Unknown command: {action}")),
            };
            Ok(())
        }
    };

    let opts = CreateCommandOpts::builder()
        .desc("Age command")
        .complete(CommandComplete::CustomList(completion()))
        .nargs(CommandNArgs::Any)
        .build();

    create_user_command("Age", age_cmd, &opts)?;

    let app_setup = Rc::clone(&app);
    let exports: Dictionary = Dictionary::from_iter::<
        [(&str, Function<Dictionary, Result<(), nvim_oxi::Error>>); 1],
    >([(
        "setup",
        Function::from_fn(move |dict: Dictionary| -> Result<(), nvim_oxi::Error> {
            app_setup.borrow_mut().setup(dict)
        }),
    )]);

    Ok(exports)
}
