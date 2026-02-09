// -- NOTE: functions declared here are supose to be independent.

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
    let decrypted = decrypt_to_string(input_path, privkey)?;

    // Write decrypted content to the output file
    let mut output_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(output_path)?;

    output_file.write_all(decrypted.as_bytes())?;

    Ok(())
}

pub fn decrypt_to_string(
    input_path: &Path,
    privkey: &str,
) -> Result<String, Box<dyn std::error::Error>>  {
    // Parse the private key
    let identity = x25519::Identity::from_str(privkey)?;
    let encrypted_file = File::open(input_path)?;

    // Decrypt the ciphertext
    let decryptor = age::Decryptor::new(encrypted_file)?;
    let mut decrypted = String::new();
    let mut reader = decryptor.decrypt(std::iter::once(&identity as &dyn age::Identity))?;
    reader.read_to_string(&mut decrypted)?;

    Ok(decrypted)
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
