#![allow(dead_code)]

use age::{x25519, Encryptor};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::str::FromStr;

pub fn encrypt_file(
    input_path: &Path,
    output_path: &Path,
    pubkey: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Parse the public key
    let recipient = x25519::Recipient::from_str(pubkey)?;

    let mut input_file = File::open(input_path)?;
    let mut plaintext = Vec::new();
    input_file.read_to_end(&mut plaintext)?;

    // Encrypt the plaintext

    let encryptor = Encryptor::with_recipients(std::iter::once(&recipient as &dyn age::Recipient))?;

    let mut encrypted = vec![];
    let mut writer = encryptor.wrap_output(&mut encrypted)?;
    writer.write_all(&plaintext)?;
    writer.finish()?;

    // Write encrypted content to the output file
    let mut output_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(output_path)?;
    output_file.write_all(&encrypted)?;

    Ok(())
}

pub fn decrypt_file(
    input_path: &Path,
    output_path: &Path,
    privkey: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Parse the private key
    let identity = x25519::Identity::from_str(privkey)?;

    let mut encrypted_file = File::open(input_path)?;
    let mut encrypted = Vec::new();
    encrypted_file.read_to_end(&mut encrypted)?;

    // Decrypt the ciphertext
    let decryptor = age::Decryptor::new(&encrypted[..])?;
    let mut decrypted = vec![];
    let mut reader = decryptor.decrypt(std::iter::once(&identity as &dyn age::Identity))?;
    reader.read_to_end(&mut decrypted)?;

    // Write decrypted content to the output file
    let mut output_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(output_path)?;
    output_file.write_all(&decrypted)?;

    Ok(())
}
