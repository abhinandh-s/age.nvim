//! This module defines the configuration structure and logic for the Just plugin.
//!
//! # Configuration Fields

use nvim_oxi::String;
use nvim_oxi::{conversion::FromObject, Dictionary};

#[derive(Debug, Default)]
pub struct Config {
    pub mail: MailConfigs,
    pub public_key: String,
    pub private_key: String,
}

#[derive(Debug, Default)]
pub struct MailConfigs {
    pub default_to: String,
    pub from: String,
    pub email: String,
    pub password: String,
    pub smtp: String,
}

impl Config {
    pub fn from_dict(options: Dictionary) -> Self {
        Config {
            public_key: options
                .get("public_key")
                .and_then(|public_key_obj| String::from_object(public_key_obj.clone()).ok())
                .unwrap_or_else(|| "".into()),

            private_key: options
                .get("private_key")
                .and_then(|private_key_obj| String::from_object(private_key_obj.clone()).ok())
                .unwrap_or_else(|| "".into()),

            mail: {
                MailConfigs {
                    default_to: options
                        .get("default_to")
                        .and_then(|default_to_obj| String::from_object(default_to_obj.clone()).ok())
                        .unwrap_or_else(|| "".into()),
                    from: options
                        .get("from")
                        .and_then(|from_obj| String::from_object(from_obj.clone()).ok())
                        .unwrap_or_else(|| "".into()),
                    email: options
                        .get("email")
                        .and_then(|email_obj| String::from_object(email_obj.clone()).ok())
                        .unwrap_or_else(|| "".into()),
                    password: options
                        .get("password")
                        .and_then(|password_obj| String::from_object(password_obj.clone()).ok())
                        .unwrap_or_else(|| "".into()),
                    smtp: options
                        .get("smtp")
                        .and_then(|smtp_obj| String::from_object(smtp_obj.clone()).ok())
                        .unwrap_or_else(|| "".into()),
                }
            },
        }
    }
}
