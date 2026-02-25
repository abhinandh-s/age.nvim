use age::secrecy::ExposeSecret;
use std::env::current_dir;
use std::{fs, path};

use nvim_oxi::api::opts::BufDeleteOpts;
use nvim_oxi::{print, Dictionary, Result as OxiResult};

use crate::command::Command;
use crate::crypt::{decrypt_to_file, decrypt_to_string};
use crate::error::AgeError;
use crate::types::{ExistingAgeFile, ExistingNonAgeFile};
use crate::{config::Config, crypt::encrypt_to_file};

#[derive(Debug)]
pub struct App {
    config: Config,
}

impl App {
    /// Creates a new `App` instance with the provided configuration.
    ///
    /// This function initializes the application state with the specified `Config`.
    pub fn new(config: Config) -> Self {
        App { config }
    }

    /// Sets up the application with the provided options from a `Dictionary`.
    ///
    /// This function allows the plugin to be reconfigured dynamically, using
    /// a dictionary of options passed from Neovim.
    pub fn setup(&mut self, dict: Dictionary) -> OxiResult<()> {
        let config = Config::from_dict(dict);
        self.config = config;
        Ok(())
    }

    /// Handles commands issued to the plugin.
    ///
    /// Based on the command and argument passed, the corresponding action (such as
    /// setting the font or closing the window) is performed.
    pub fn handle_command(
        &mut self,
        cmd: Command,
        raw_args: Vec<String>,
    ) -> Result<(), crate::error::AgeError> {
        let filenames = if raw_args.is_empty() {
            vec![self.config.key_file.to_string()]
        } else {
            raw_args
        };

        match &cmd {
            Command::DecryptFile => {
                if let Err(err) = self.decrypt_current_file(filenames) {
                    print!("{}", err);
                }
                Ok(())
            }
            // ```vim
            //
            // :Age encrypt " uses public key from config
            // :Age encrypt /path/to/recipents.txt " list for public keys
            //
            // ```
            Command::EncryptFile => {
                if let Err(err) = self.encrypt_current_file(filenames) {
                    print!("{}", err);
                }
                Ok(())
            }
            Command::GenKey => {
                let re = self.gen_new_key();
                if let Err(err) = re {
                    print!("{}", err);
                }
                Ok(())
            }
        }
    }

    fn gen_new_key(&self) -> Result<(), AgeError> {
        let key = age::x25519::Identity::generate();
        let time = chrono::Local::now();
        let formatted_time = time.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

        let contents = format!(
            "# created: {}\n# public key: {}\n{}",
            formatted_time,
            key.to_public(),
            key.to_string().expose_secret()
        );

        std::fs::write(current_dir()?.join("key.txt"), contents)?;
        Ok(())
    }

    fn decrypt_current_file(&self, filenames: Vec<String>) -> Result<(), AgeError> {
        let current_file_bufnr = nvim_oxi::api::get_current_buf();
        let current_file_path = current_file_bufnr.get_name()?;
        let mut current_file = ExistingAgeFile::try_from(current_file_path)?;

        let out_path = current_file.strip_age()?;

        if out_path.as_path().exists() {
            fs::remove_file(out_path.as_path())?;
        }

        decrypt_to_file(current_file.path(), out_path.as_path(), filenames)?;

        let new_scratch_buf = nvim_oxi::api::create_buf(false, true)?;
        nvim_oxi::api::set_current_buf(&new_scratch_buf)?;

        let opts = BufDeleteOpts::builder()
            .force(true) // Force deletion, ignoring unsaved changes
            .build();

        // we are deleting the buffer not the file.
        nvim_oxi::api::Buffer::delete(current_file_bufnr, &opts)?;

        let command = format!(
            "edit {}",
            out_path.display().to_string().replace(' ', "\\ ")
        );
        nvim_oxi::api::command(&command)?;

        Ok(())
    }

    fn encrypt_current_file(&self, filenames: Vec<String>) -> Result<(), AgeError> {
        let current_file_path = nvim_oxi::api::get_current_buf().get_name()?;
        let mut current_file = ExistingNonAgeFile::try_from(current_file_path)?;
        let list_buf = nvim_oxi::api::list_bufs();

        let d = list_buf.len();
        // if len is one will will create a new buf
        if d == 1 {
            // is a scrach buf may be we can show some
            let new_scratch_buf = nvim_oxi::api::create_buf(false, true)?;
            nvim_oxi::api::set_current_buf(&new_scratch_buf)?;
        } else {
            for buf in list_buf {
                if buf.get_name()?.to_string_lossy() != current_file.to_string() {
                    nvim_oxi::api::set_current_buf(&buf)?;
                    break;
                }
            }
        }

        let new_file = current_file.append_age()?;

        encrypt_to_file(current_file.path(), new_file.as_path(), filenames)?;

        if self.config.encrypt_and_del {
            fs::remove_file(current_file.path())?;
        }

        Ok(())
    }

    pub fn decrypt_to_string(&self, file_path: String) -> Result<String, AgeError> {
        let path = path::Path::new(&file_path);
        validate_path(path)?;

        // Logic: If private_key_file is set, use that. Otherwise use the string.
        if !self.config.key_file.is_empty() {
            let id_file = self.config.key_file.to_string();
            return decrypt_to_string(path, vec![id_file]);
        }

        Err(AgeError::new(
            "the field `key_file` in config is missing".to_owned(),
        ))
    }

    pub fn decrypt_from_string(&self, encrypted: String) -> Result<String, AgeError> {
        // Logic: If private_key_file is set, use that. Otherwise use the string.
        if !self.config.key_file.is_empty() {
            let id_file = self.config.key_file.to_string();
            return crate::crypt::decrypt_from_string(encrypted, vec![id_file]);
        }

        Err(AgeError::new(
            "the field `key_file` in config is missing".to_owned(),
        ))
    }

    pub fn decrypt_with_identities(
        &self,
        file_path: String,
        key_files: Vec<String>,
    ) -> Result<String, AgeError> {
        let path = path::Path::new(&file_path);
        validate_path(path)?;

        decrypt_to_string(path, key_files)
    }
}

fn validate_path(path: &path::Path) -> Result<(), AgeError> {
    if !path.exists() {
        return Err(AgeError::new(format!(
            "File not found: {}",
            path.to_string_lossy()
        )));
    }

    Ok(())
}
