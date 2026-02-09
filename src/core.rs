use age::secrecy::ExposeSecret;
use std::env::current_dir;
use std::{fs, path};

use nvim_oxi::api::opts::BufDeleteOpts;
use nvim_oxi::{print, Dictionary, Result as OxiResult};

use crate::command::Command;
use crate::crypt::decrypt_file;
use crate::error::Error;
use crate::{config::Config, crypt::encrypt_file};

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
    pub fn handle_command(&mut self, cmd: Command) -> OxiResult<()> {
        match &cmd {
            Command::DecryptFile => {
                let re = self.decrypt_current_file();
                if let Err(err) = re {
                    print!("{}", err);
                }
                Ok(())
            }
            Command::EncryptFile => {
                let re = self.encrypt_current_file();
                if let Err(err) = re {
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

    fn gen_new_key(&self) -> Result<(), Error> {
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

    fn decrypt_current_file(&self) -> Result<(), Error> {
        let binding = self.config.private_key.to_string();
        let private_key = binding.as_str();
        let current_file_bufnr = nvim_oxi::api::get_current_buf();
        let current_file_path = current_file_bufnr.get_name()?;
        let current_file = current_file_path.to_string_lossy();
        let extension = current_file_path
            .extension()
            .map(|e| e.to_string_lossy().to_string());
        match extension {
            Some(ext) if ext == "age" => {
                let name = current_file.rsplit_once(".");
                if let Some((stem_name, _)) = name {
                    if path::Path::new(stem_name).exists() {
                        fs::remove_file(stem_name)?;
                    }
                    let new_scratch_buf = nvim_oxi::api::create_buf(false, true)?;
                    nvim_oxi::api::set_current_buf(&new_scratch_buf)?;
                    let opts = BufDeleteOpts::builder()
                        .force(true) // Force deletion, ignoring unsaved changes
                        .build();
                    nvim_oxi::api::Buffer::delete(current_file_bufnr, &opts)?;
                    decrypt_file(&current_file_path, path::Path::new(stem_name), private_key)
                        .and_then(|_| {
                            let command = format!("edit {stem_name}");
                            nvim_oxi::api::command(&command)?;
                            Ok(())
                        })?;
                }
            }
            Some(_) => {
                print!("Not an age file. Aborting decryption..");
            }
            None => {
                print!("This file have no extension. `.age` extension is needed for decryption");
            }
        }
        Ok(())
    }

    fn encrypt_current_file(&self) -> Result<(), Error> {
        let binding_pub = self.config.public_key.to_string();
        let public_key = binding_pub.as_str();
        validate_public_key(public_key)?;
        let current_file_path = nvim_oxi::api::get_current_buf().get_name()?;
        let cfile = current_file_path.to_string_lossy();
        let list_buf = nvim_oxi::api::list_bufs();
        let d = list_buf.len();
        // if len is one will will create a new buf
        if d == 1 {
            // is a scrach buf may be we can show some
            let new_scratch_buf = nvim_oxi::api::create_buf(false, true)?;
            nvim_oxi::api::set_current_buf(&new_scratch_buf)?;
        } else {
            for buf in list_buf {
                if buf.get_name()?.to_string_lossy() != cfile {
                    nvim_oxi::api::set_current_buf(&buf)?;
                    break;
                }
            }
        }
        let binding = cfile.to_string();
        let extension_result = path::Path::new(&binding).extension();
        match extension_result {
            Some(ext) => {
                let new_extension = ext.to_string_lossy().to_string() + ".age";
                encrypt_file(
                    path::Path::new(&cfile.to_string()),
                    &path::Path::new(&cfile.to_string()).with_extension(new_extension),
                    public_key,
                )
                .and_then(|_| {
                    if self.config.encrypt_and_del {
                        fs::remove_file(current_file_path)?;
                    }
                    Ok(())
                })?;
            }
            None => {
                encrypt_file(
                    path::Path::new(&cfile.to_string()),
                    &path::Path::new(&cfile.to_string()).with_extension("age"),
                    public_key,
                )?;
            }
        }
        Ok(())
    }

    pub fn decrypt_to_string(&self, file_path: String) -> Result<String, Error> {
        let private_key = self.config.private_key.to_string();
        validate_private_key(private_key.as_ref())?;

        let path = path::Path::new(&file_path);
        validate_path(path)?;

        Ok(crate::crypt::decrypt_to_string(path, private_key.as_str())?)
    }

    pub fn decrypt_with_identities(
        &self,
        file_path: String,
        identity_paths: Vec<String>,
    ) -> Result<String, Error> {
        let path = path::Path::new(&file_path);
        validate_path(path)?;

        let mut identities = Vec::new();

        for id_path in identity_paths {
            let p = path::Path::new(&id_path);
            if !p.exists() {
                return Err(Error::Custom(format!("Identity file not found: {id_path}")));
            }

            // Use age::IdentityFile to parse the file (supports age & SSH formats)
            let identity_file =
                age::IdentityFile::from_file(p.to_string_lossy().to_string())?.into_identities()?;

            // IdentityFile implements into_identities()
            identities.extend(identity_file);
        }

        if identities.is_empty() {
            return Err(Error::Custom("No valid identities provided".to_owned()));
        }

        Ok(crate::crypt::decrypt_with_identities(path, identities)?)
    }
}

fn validate_private_key(key: &str) -> Result<(), nvim_oxi::Error> {
    if key.is_empty() {
        return Err(Error::Custom("Private key not configured".to_owned()).into());
    }

    if !key.starts_with("AGE-SECRET-KEY-") {
        return Err(Error::Custom("provided key is not a vaild Private key".to_owned()).into());
    }

    Ok(())
}

fn validate_public_key(key: &str) -> Result<(), nvim_oxi::Error> {
    if key.is_empty() {
        return Err(Error::Custom("Private key not configured".to_owned()).into());
    }

    if !key.starts_with("age") {
        return Err(Error::Custom("provided key is not a vaild Private key".to_owned()).into());
    }

    Ok(())
}

fn validate_path(path: &path::Path) -> Result<(), nvim_oxi::Error> {
    if !path.exists() {
        return Err(Error::Custom(format!("File not found: {}", path.to_string_lossy())).into());
    }

    Ok(())
}
