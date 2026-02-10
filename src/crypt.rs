#![allow(dead_code)]
// -- NOTE: functions declared here are supose to be independent.

use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::str::FromStr;

pub fn decrypt_into_file<'a>(
    input_path: &Path,
    output_path: &Path,
    keys: impl Iterator<Item = &'a dyn age::Identity>,
) -> Result<(), Box<dyn std::error::Error>> {
    let decrypted = decrypt_to_string(input_path, keys)?;

    // Write decrypted content to the output file
    let mut output_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(output_path)?;

    output_file.write_all(decrypted.as_bytes())?;

    Ok(())
}

pub fn decrypt_to_string<'a>(
    input_path: &Path,
    keys: impl Iterator<Item = &'a dyn age::Identity>,
) -> Result<String, Box<dyn std::error::Error>> {
    // Parse the private key
//    let identity = x25519::Identity::from_str(privkey)?;

    // Decrypt the ciphertext
    let decryptor = decrypt_from_file(input_path, keys.into_iter())?;
    Ok(decryptor)
}

pub fn decrypt_with_identities(
    input_path: &Path,
    identities: Vec<Box<dyn age::Identity>>,
) -> Result<String, Box<dyn std::error::Error>> {
    let encrypted_file = File::open(input_path)?;

    // Decrypt using the provided list of identities
    let decryptor = age::Decryptor::new(encrypted_file)?;

    // We pass the vector of boxed identities as an iterator
    let mut reader = decryptor.decrypt(identities.iter().map(|i| i.as_ref()))?;

    let mut decrypted = String::new();
    reader.read_to_string(&mut decrypted)?;

    Ok(decrypted)
}

// decrypt the obtained ciphertext to the plaintext.
// this can manage both bytes and armored
//
// ```
// let key = age::x25519::Identity::generate();
// let keys: Vec<&dyn age::Identity> = vec![&key];
// ```
fn decrypt<'a, R: std::io::Read>(
    keys: impl Iterator<Item = &'a dyn age::Identity>,
    encrypted: R,
) -> Result<String, crate::error::Error> {
    let reader = age::armor::ArmoredReader::new(encrypted);
    let decryptor = age::Decryptor::new(reader)?;
    let mut decrypted_bytes = vec![];
    let mut reader = decryptor.decrypt(keys)?;

    reader.read_to_end(&mut decrypted_bytes)?;

    Ok(String::from_utf8(decrypted_bytes)?)
}

fn decrypt_from_file<'a>(
    path: &Path,
    keys: impl Iterator<Item = &'a dyn age::Identity>,
) -> Result<String, crate::error::Error> {
    let file = std::fs::File::open(path)?;
    decrypt(keys, file)
}

// decrypt the obtained ciphertext to the plaintext.
//
// ```
// let key = age::x25519::Identity::generate();
// let pubkey = key.to_public();
// let recipients: Vec<&dyn age::Recipient> = vec![&pubkey];
// ```
fn encrypt<'a>(
    recipients: impl Iterator<Item = &'a dyn age::Recipient>,
    plaintext: &[u8],
) -> Result<Vec<u8>, crate::error::Error> {
    let encryptor = age::Encryptor::with_recipients(recipients)?;

    let mut encrypted = vec![];
    let mut writer = encryptor.wrap_output(&mut encrypted)?;
    writer.write_all(plaintext)?;
    writer.finish()?;

    Ok(encrypted)
}

pub fn encrypt_to_armored_string<'a>(
    recipients: impl Iterator<Item = &'a dyn age::Recipient>,
    plaintext: impl Into<&'a [u8]>,
) -> Result<String, Box<dyn std::error::Error>> {
    let encrypted_bytes: Vec<u8> = encrypt(recipients, plaintext.into())?;

    // Prepare a buffer for the ASCII text
    let mut output = Vec::new();

    // Wrap the output in the ArmoredWriter
    let mut armor_writer =
        age::armor::ArmoredWriter::wrap_output(&mut output, age::armor::Format::AsciiArmor)?;

    // Write binary data and—crucially—call finish()
    armor_writer.write_all(&encrypted_bytes)?;
    armor_writer.finish()?;

    // Convert the now-armored Vec<u8> into a String}
    Ok(String::from_utf8(output)?)
}

pub fn encrypt_to_armored_as_bytes<'a>(
    recipients: impl Iterator<Item = &'a dyn age::Recipient>,
    plaintext: impl Into<&'a [u8]>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let encrypted_bytes: Vec<u8> = encrypt(recipients, plaintext.into())?;

    // Prepare a buffer for the ASCII text
    let mut output = Vec::new();

    // Wrap the output in the ArmoredWriter
    let mut armor_writer =
        age::armor::ArmoredWriter::wrap_output(&mut output, age::armor::Format::AsciiArmor)?;

    // Write binary data and—crucially—call finish()
    armor_writer.write_all(&encrypted_bytes)?;
    armor_writer.finish()?;

    // Convert the now-armored Vec<u8> into a String}
    Ok(output)
}

pub fn encrypt_file(
    input_path: &Path,
    output_path: &Path,
    recipients: Vec<Box<dyn age::Recipient>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut input_file = File::open(input_path)?;
    let mut plaintext = Vec::new();
    input_file.read_to_end(&mut plaintext)?;

    let encrypted =
        encrypt_to_armored_as_bytes(recipients.iter().map(|x| x.as_ref()), &plaintext[..])?;

    // Write encrypted content to the output file
    let mut output_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(output_path)?;
    output_file.write_all(&encrypted)?;

    Ok(())
}

pub fn parse_identities_args(
    input: Vec<String>,
) -> Result<Vec<age::x25519::Identity>, crate::error::Error> {
    let mut output = Vec::new();
    for part in input {
        if part.starts_with("age1") || part.starts_with("ssh-") {
            output.extend(parse_identities(&part)?.into_iter());
        } else {
            let path = std::path::Path::new(&part);
            let con = path.canonicalize()?;
            print!("canonicalized path: {}", con.display());
            if path.exists() {
                let ctx = std::fs::read_to_string(path)?;
                let v = ctx.split_whitespace().collect::<Vec<&str>>();
                // Use age::IdentityFile to parse the file (supports age & SSH formats)
                for i in v {
                    crate::crypt::parse_identities(i)?.iter().for_each(|d| {
                        output.push(d.to_owned());
                    });
                }
            } else {
                return Err(crate::error::Error::Other(format!(
                    "Arg '{}' is not a valid key or file path",
                    part
                )));
            }
        }
    }
    Ok(output)
}

pub fn parse_identities(input: &str) -> Result<Vec<age::x25519::Identity>, crate::error::Error> {
    let mut output = Vec::new();

    for line in input.lines() {
        if !line.starts_with("#") {
            let parts = line.split_whitespace();
            for part in parts {
                if part.starts_with("AGE-SECRET-KEY-") {
                    output.push(age::x25519::Identity::from_str(part)?);
                }
            }
        }
    }
    Ok(output)
}

pub fn parse_recipients(input: &str) -> Result<Vec<age::x25519::Recipient>, crate::error::Error> {
    let mut output = Vec::new();

    let parts = input.split_whitespace();
    for part in parts {
        if part.starts_with("age1") {
            output.push(age::x25519::Recipient::from_str(part)?);
        }
    }
    Ok(output)
}

#[cfg(test)]
mod test {
    use super::encrypt;
    use crate::crypt::{
        decrypt_from_file, encrypt_to_armored_string, parse_identities, parse_recipients,
    };

    #[test]
    fn core() -> Result<(), crate::error::Error> {
        let key = age::x25519::Identity::generate();
        let pubkey = key.to_public();
        let pubkeys: Vec<&dyn age::Recipient> = vec![&pubkey];

        let plaintext = b"Hello world!";
        let encrypted = encrypt(pubkeys.clone().into_iter(), plaintext)?;

        let keys: Vec<&dyn age::Identity> = vec![&key];

        let decrypted = super::decrypt(keys.clone().into_iter(), &encrypted[..])?;

        let st: String = "Hello world!".into();
        assert_eq!(decrypted, st);

        Ok(())
    }

    #[test]
    fn core_02() -> Result<(), crate::error::Error> {
        let mut pubkeys: Vec<&dyn age::Recipient> = vec![];
        let mut keys: Vec<&dyn age::Identity> = vec![];

        // Normal encryption
        let path_01 = std::path::Path::new("tests/i_am_groot.txt.age");
        // Armored encryption
        let path_02 = std::path::Path::new("tests/i_am_groot_armored.txt.age");

        // key.txt
        let binding = parse_identities(include_str!("../tests/test_key.txt"))?;
        binding.iter().for_each(|id| {
            keys.push(id);
        });

        let path_01_decrypted = decrypt_from_file(path_01, keys.clone().into_iter())?;
        assert_eq!(path_01_decrypted, String::from("I am groot.\n"));

        let path_02_decrypted = decrypt_from_file(path_02, keys.clone().into_iter())?;
        assert_eq!(path_02_decrypted, String::from("I am groot.\n"));

        let parsed_pub_keys = parse_recipients(include_str!("../tests/test_key.txt"))?;
        parsed_pub_keys.iter().for_each(|re| {
            pubkeys.push(re);
        });

        let path_02_encrypted =
            encrypt_to_armored_string(pubkeys.clone().into_iter(), path_01_decrypted.as_bytes())?;
        let path_02_ctx = include_str!("../tests/i_am_groot_armored.txt.age");

        const STARTS_WITH: &str =
            "-----BEGIN AGE ENCRYPTED FILE-----\nYWdlLWVuY3J5cHRpb24ub3JnL3YxCi0+IFgyNTUxOS";
        assert_eq!(
            String::from_utf8(path_02_encrypted.into())?.starts_with(STARTS_WITH),
            path_02_ctx.to_owned().starts_with(STARTS_WITH)
        );

        let path_01_encrypted = encrypt(pubkeys.clone().into_iter(), path_01_decrypted.as_bytes())?;
        let path_01_ctx = include_bytes!("../tests/i_am_groot.txt.age");

        assert_eq!(path_01_encrypted[..32], path_01_ctx.to_vec()[..32]);

        Ok(())
    }
}
