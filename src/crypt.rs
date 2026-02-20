// This module provide high level wrapper around the `age` crate.
//
// ## General note
//
// - Functions declared here are supose to be independent.
// - Only decryption functions will form part of public api.
// - While providing `key_file` through cmd args. user is supose
//   to provide full path like `/home/user/.config/age/keys.txt`
//   yet we takes some efforts to convert other forms into full path
//   via `get_full_path` function.
//
// ## Encryption
//
// - We uses ASCII Armor [`age::armor`] by default for every encryption we do.
//   while decrypting `age` is smart enough to know its armored. No additions
//   flag or settings is needed from normal decryption.
//
// ## Overview
//
// `fn encrypt/decrypt => vec<u8>` are the core functions, others are
// wrapper around this for simplicity.

use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;

use crate::error::AgeError;

/// encrypts the obtained plaintext `&[u8]` into ciphertext `Vec<u8>`.
/// with many `Recipient` (not key file/files)
fn encrypt<'a>(
    recipients: impl Iterator<Item = &'a dyn age::Recipient>,
    plaintext: &[u8],
) -> Result<Vec<u8>, AgeError> {
    let encryptor = age::Encryptor::with_recipients(recipients)?;

    let mut encrypted: Vec<u8> = vec![];
    let mut writer = encryptor.wrap_output(age::armor::ArmoredWriter::wrap_output(
        &mut encrypted,
        age::armor::Format::AsciiArmor,
    )?)?;
    writer.write_all(plaintext)?;
    writer.finish().and_then(|armor| armor.finish())?;

    Ok(encrypted)
}

/// encrypts the contents of obtained file `&Path` into ciphertext `Vec<u8>`
/// Recipient's are taken from `key_files`
fn encrypt_file(path: &Path, key_files: Vec<String>) -> Result<Vec<u8>, AgeError> {
    let mut file = std::fs::File::open(path)?;
    let mut plaintext = Vec::new();
    file.read_to_end(&mut plaintext)?;

    let recipients = load_recipients(key_files)?;

    encrypt(
        recipients.iter().map(|r| r.as_ref() as &dyn age::Recipient),
        &plaintext[..],
    )
}

/// encrypts the contents of obtained file `&Path` into ciphertext `String`
/// Recipient's are taken from `key_files`
///
/// same as `encrypt_file` but returns `String`
fn encrypt_path_to_string(path: &Path, key_files: Vec<String>) -> Result<String, AgeError> {
    Ok(String::from_utf8(encrypt_file(path, key_files)?)?)
}

/// encrypts the `String` provided into ciphertext `String`
/// Recipient's are taken from `key_files`
#[allow(dead_code)]
fn encrypt_to_string(plaintext: String, key_files: Vec<String>) -> Result<String, AgeError> {
    let binding = load_recipients(key_files)?;
    let keys = binding.iter().map(|f| f.as_ref() as &dyn age::Recipient);

    let decrypted = encrypt(keys, plaintext.as_bytes())?;

    Ok(String::from_utf8(decrypted)?)
}

/// encrypts the contents of obtained file `&Path` into the output file pointed
/// Recipient's are taken from `key_files`
pub(super) fn encrypt_into_file(
    plaintext: &Path,
    out_path: &Path,
    key_files: Vec<String>,
) -> Result<(), AgeError> {
    let encrypted = encrypt_path_to_string(plaintext, key_files)?;

    // Write encrypted content to the output file
    let mut output_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(out_path)?;
    output_file.write_all(encrypted.as_bytes())?;

    Ok(())
}

// decrypts the obtained ciphertext [any thing that impl `std::io::Read`]
// to the plaintext `Vec<u8>`
/// with many `Recipient` (not key file/files)
//
//  this can manage both bytes and armored
fn decrypt<'a, R: std::io::Read>(
    keys: impl Iterator<Item = &'a dyn age::Identity>,
    encrypted: R,
) -> Result<Vec<u8>, AgeError> {
    let reader = age::armor::ArmoredReader::new(encrypted);
    let decryptor = age::Decryptor::new(reader)?;
    let mut decrypted_bytes = vec![];
    let mut reader = decryptor.decrypt(keys)?;

    reader.read_to_end(&mut decrypted_bytes)?;

    Ok(decrypted_bytes)
}

/// decrypts the encrypted content of file provided into plaintext `Vec<u8>`
/// Identity's are taken from `key_files`
fn decrypt_files(path: &Path, filenames: Vec<String>) -> Result<Vec<u8>, AgeError> {
    let file = std::fs::File::open(path)?;
    let binding = load_identities(filenames)?;
    let keys = binding.iter().map(|f| f.as_ref() as &dyn age::Identity);

    decrypt(keys, file)
}

/// decrypts the encrypted content of file provided into plaintext `String`
/// Identity's are taken from `key_files`
pub(super) fn decrypt_to_string(
    input_path: &Path,
    key_files: Vec<String>,
) -> Result<String, AgeError> {
    let decrypted = decrypt_files(input_path, key_files)?;

    Ok(String::from_utf8(decrypted)?)
}

/// decrypts the `String` provided into plaintext `String`
/// Identity's are taken from `key_files`
pub(super) fn decrypt_from_string(
    encrypted: String,
    key_files: Vec<String>,
) -> Result<String, AgeError> {
    let binding = load_identities(key_files)?;
    let keys = binding.iter().map(|f| f.as_ref() as &dyn age::Identity);

    let decrypted = decrypt(keys, std::io::Cursor::new(encrypted.as_bytes()))?;

    Ok(String::from_utf8(decrypted)?)
}

/// decrypts the contents of obtained file `&Path` into the output file pointed
/// Identity's are taken from `key_files`
pub(super) fn decrypt_into_file(
    input_path: &Path,
    output_path: &Path,
    filenames: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let decrypted = decrypt_to_string(input_path, filenames)?;

    // Write decrypted content to the output file
    let mut output_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(output_path)?;

    output_file.write_all(decrypted.as_bytes())?;

    Ok(())
}

/// get all Recipient's from provided `key_files`
fn load_recipients(
    key_files: Vec<String>,
) -> Result<Vec<Box<dyn age::Recipient + Send + 'static>>, AgeError> {
    let mut output: Vec<Box<dyn age::Recipient + Send + 'static>> = Vec::new();
    for path in key_files {
        let full_path = get_full_path(&path)?.to_string_lossy().to_string();
        output.extend(age::IdentityFile::from_file(full_path)?.to_recipients()?);
    }
    Ok(output)
}

/// get all Identity's from provided `key_files`
fn load_identities(
    filenames: Vec<String>,
) -> Result<Vec<Box<dyn age::Identity>>, Box<dyn std::error::Error>> {
    let mut output = Vec::new();
    for filename in filenames {
        let full_path = get_full_path(&filename)?.to_string_lossy().to_string();
        output.extend(age::IdentityFile::from_file(full_path)?.into_identities()?);
    }
    Ok(output)
}

/// tries to converts users input: ~/some/file.txt => /home/user/some/file.txt
fn get_full_path(input: &str) -> Result<std::path::PathBuf, AgeError> {
    let mut path_buf = std::path::PathBuf::new();

    // 1. expand Tilde
    if input.starts_with("~/") {
        let home = std::env::var("HOME")?;
        path_buf.push(home);
        path_buf.push(
            input
                .strip_prefix("~/")
                .ok_or(AgeError::new("Can't strip ~/from path".to_owned()))?,
        );
    } else {
        path_buf.push(input);
    }

    // 2. Use canonicalize to:
    //    - Turn relative into absolute
    //    - Resolve ".." and "."
    //    - Check if the file actually exists
    let absolute_path = std::fs::canonicalize(&path_buf)?;
    if absolute_path.is_file() {
        return Ok(absolute_path);
    }
    Err(AgeError::new("Can't parse path".to_owned()))
}

#[cfg(test)]
mod test {

    use crate::crypt::{
        decrypt_from_string, decrypt_into_file, decrypt_to_string, encrypt_into_file,
        encrypt_path_to_string, encrypt_to_string, get_full_path,
    };
    use crate::error::AgeError;

    #[test]
    fn into_file() -> Result<(), AgeError> {
        let filenames = vec!["tests/test_key.txt".to_owned()];
        let input = std::path::Path::new("tests/some/dir/file.txt");
        let encrypted = std::path::Path::new("tests/some/dir/file.txt.age");
        let decrypted = std::path::Path::new("tests/some/dir/file_decrypted.txt");

        encrypt_into_file(input, encrypted, filenames.clone())?;
        decrypt_into_file(encrypted, decrypted, filenames)?;

        let original = std::fs::read_to_string(input)?;
        let result = std::fs::read_to_string(decrypted)?;

        assert_eq!(original, result);

        Ok(())
    }
    #[test]
    fn to_and_as_string() -> Result<(), AgeError> {
        let key_files = vec!["tests/test_key.txt".to_owned()];
        let input = std::path::Path::new("tests/some/dir/file.txt");
        let encrypted = std::path::Path::new("tests/some/dir/file.txt.age");
        let original = std::fs::read_to_string(input)?;

        let e = encrypt_path_to_string(input, key_files.clone())?;
        let df = decrypt_from_string(e, key_files.clone())?;
        assert_eq!(original, df);

        let d = decrypt_to_string(encrypted, key_files.clone())?;
        assert_eq!(original, d);

        let enc = encrypt_to_string("Some secret text.\n".to_owned(), key_files.clone())?;
        let ed = decrypt_from_string(enc, key_files.clone())?;
        assert_eq!(original, ed);

        Ok(())
    }

    #[test]
    #[ignore = "Only works on my machine"]
    fn path_fix() {
        let shortcut = "~/git/test_01/test/some/file.txt";
        let rooted = "/home/abhi/git/test_01/test/some/file.txt";
        let deep_reletive = "./test/some/file.txt";
        let deep_reletive_02 = "../test_01/test/some/file.txt";
        let reletive = "test/some/file.txt";

        for i in [shortcut, rooted, reletive, deep_reletive, deep_reletive_02] {
            assert!(get_full_path(i).is_ok())
        }
    }

    #[test]
    #[ignore = "Only works on my machine"]
    fn identities_file() -> Result<(), AgeError> {
        let key_files = vec!["~/.config/sops/age/keys.txt".to_owned()];

        let enc = encrypt_to_string("plaintext".to_owned(), key_files.clone())?;

        let dec = decrypt_from_string(enc, key_files.clone())?;

        assert_eq!("plaintext".to_owned(), dec);

        Ok(())
    }
}
