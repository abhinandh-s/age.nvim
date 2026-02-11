//! This module defines the configuration structure and logic for the Age plugin.
//!
//! # Configuration Fields

use nvim_oxi::String;
use nvim_oxi::{conversion::FromObject, Dictionary};

#[derive(Debug, Default)]
pub struct Config {
    pub key_file: String,
    // pub key_files: Vec<String>,
    pub encrypt_and_del: bool,
}

impl Config {
    pub fn from_dict(options: Dictionary) -> Self {
        Config {
            key_file: options
                .get("key_file")
                .and_then(|key_file_obj| String::from_object(key_file_obj.clone()).ok())
                .unwrap_or_else(|| "".into()),

            // key_files: options
            //     .get("key_files")
            //     .and_then(|key_files_obj| Vec::from_object(key_files_obj.clone()).ok())
            //     .unwrap_or_default(),

                encrypt_and_del: options
                .get("encrypt_and_del")
                .and_then(|encrypt_and_del| bool::from_object(encrypt_and_del.clone()).ok())
                .unwrap_or(false),
        }
    }
}
