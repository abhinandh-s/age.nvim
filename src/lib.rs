use std::{cell::RefCell, rc::Rc};

use nvim_oxi::{
    api::{create_user_command, err_writeln, opts::CreateCommandOpts, types::*},
    Dictionary, Function, Object,
};

use self::{
    command::{completion, Command},
    config::Config,
    core::App,
};

mod command;
mod config;
mod core;
mod crypt;
mod error;
mod types;

#[nvim_oxi::plugin]
fn age() -> Result<Dictionary, nvim_oxi::Error> {
    let config = Config::default();
    let app = Rc::new(RefCell::new(App::new(config)));

    // -- `:Age` command
    let age_cmd = {
        let app_handle_cmd = Rc::clone(&app);

        move |args: CommandArgs| -> Result<(), nvim_oxi::Error> {
            let binding = args.args.unwrap_or_default();
            let mut parts = binding.split_whitespace();

            let action = parts.next().unwrap_or_default();
            let command = Command::from_str(action);
            let raw_args = parts.map(|s| s.to_owned()).collect::<Vec<String>>();

            match command {
                Some(command) => {
                    app_handle_cmd
                        .borrow_mut()
                        .handle_command(command, raw_args)?;
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

    // -- setup function for config
    //
    // ```lua
    //
    //  config = function()
    //    require('age').setup({
    //      key_file = vim.fn.expand("~/.config/sops/age/keys.txt"),
    //      encrypt_and_del = true,
    //    })
    //  end
    //
    // ```
    let app_setup = Rc::clone(&app);
    let mut exports: Dictionary = Dictionary::from_iter::<
        [(&str, Function<Dictionary, Result<(), nvim_oxi::Error>>); 1],
    >([(
        "setup",
        Function::from_fn(move |dict: Dictionary| -> Result<(), nvim_oxi::Error> {
            app_setup.borrow_mut().setup(dict)
        }),
    )]);

    // # Api 01
    //
    // ```lua
    //
    // local age = require("age")
    //
    // ---------
    // -- api 01
    // ---------
    //
    // -- assuming `age.setup()` is configured with `key_file`
    //
    // -- Load the secret
    // local path = vim.fn.expand("~/.config/nvim/top_secret.txt.age")
    //
    // local secret = age.decrypt_to_string(path):gsub("%s+", "")
    // print(secret)
    //
    // ```
    //
    let age_api_01 = Rc::clone(&app);
    exports.insert(
        "decrypt_to_string",
        Object::from(
            Function::<String, Result<String, nvim_oxi::Error>>::from_fn(
                move |file_path: String| {
                    age_api_01
                        .borrow()
                        .decrypt_to_string(file_path)
                        .map_err(|err| err.into()) // AgeError into nvim_oxi::Error
                },
            ),
        ),
    );

    // # Api 02
    //
    // ```lua
    //
    // local age = require("age")
    //
    // ---------
    // -- api 02
    // ---------
    //
    // -- does not require `age.setup()`
    //
    // local secret_02 = age.decrypt_to_string_with_identities(
    //   vim.fn.expand("~/.config/nvim/top_secret.txt.age"),
    //   {
    //     vim.fn.expand("~/.local/share/age/key.txt"),
    //   }
    // )
    //
    // print(secret_02)
    //
    // ```
    //
    let age_api_02 = Rc::clone(&app);
    exports.insert(
        "decrypt_to_string_with_identities",
        Object::from(Function::<
            (String, Vec<String>),
            Result<String, nvim_oxi::Error>,
        >::from_fn(move |(file_path, identity_paths)| {
            age_api_02
                .borrow()
                .decrypt_with_identities(file_path, identity_paths)
                .map_err(|err| err.into()) // AgeError into nvim_oxi::Error
        })),
    );

    // # Api 03
    //
    // ```lua
    //
    // local age = require("age")
    //
    // ---------
    // -- api 03
    // ---------
    // local enc = "-----BEGIN AGE ENCRYPTED FILE-----\nYsdfuhsdulfgdfkoryephvcguew==\n-----END AGE ENCRYPTED FILE-----"
    //
    // -- assuming `age.setup()` is configured with `key_file`
    //
    // local secret_03 = age.decrypt_from_string(enc)
    //
    // print(secret_03)
    //
    // ```
    //
    let age_api_03 = Rc::clone(&app);
    exports.insert(
        "decrypt_from_string",
        Object::from(
            Function::<String, Result<String, nvim_oxi::Error>>::from_fn(move |ctx| {
                age_api_03
                    .borrow()
                    .decrypt_from_string(ctx)
                    .map_err(|err| err.into()) // AgeError into nvim_oxi::Error
            }),
        ),
    );

    Ok(exports)
}
