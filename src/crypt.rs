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
pub(super) fn encrypt_to_file(
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
pub(super) fn decrypt_to_file(
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

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod test {

    use age::secrecy::ExposeSecret;
    use std::path::PathBuf;
    use tempfile::TempDir;

    use crate::{
        crypt::{
            decrypt_from_string, decrypt_to_file, decrypt_to_string, encrypt_path_to_string,
            encrypt_to_file, encrypt_to_string, get_full_path,
        },
        error::AgeError,
    };

    #[test]
    fn into_file() -> Result<(), AgeError> {
        let filenames = vec!["tests/test_key.txt".to_owned()];
        let input = std::path::Path::new("tests/some/dir/file.txt");
        let encrypted = std::path::Path::new("tests/some/dir/file.txt.age");
        let decrypted = std::path::Path::new("tests/some/dir/file_decrypted.txt");

        encrypt_to_file(input, encrypted, filenames.clone())?;
        decrypt_to_file(encrypted, decrypted, filenames)?;

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

    // ----------------------------------------------------------------
    // Test Fixture
    // ----------------------------------------------------------------
    //
    // Generates a real age x25519 key at test time and writes it to
    // a temp directory alongside test plaintext files.
    //
    // Layout:
    //   <tmp>/
    //     key.txt              <- generated age identity file
    //     plaintext.txt        <- "Hello, age!\n"
    //     empty.txt            <- ""
    //     binary.bin           <- raw bytes including nulls
    //     multiline.txt        <- multiple lines
    //
    struct Fixture {
        dir: TempDir,
        pub key_path: PathBuf,
    }

    impl Fixture {
        fn new() -> Self {
            let dir = tempfile::tempdir().expect("failed to create temp dir");

            // Generate a fresh age identity
            let identity = age::x25519::Identity::generate();
            let public_key = identity.to_public().to_string();
            let time = chrono::Local::now();
            let key_contents = format!(
                "# created: {}\n# public key: {}\n{}",
                time.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
                public_key,
                identity.to_string().expose_secret()
            );

            let key_path = dir.path().join("key.txt");
            std::fs::write(&key_path, key_contents).unwrap();

            // Test files
            std::fs::write(dir.path().join("plaintext.txt"), b"Hello, age!\n").unwrap();
            std::fs::write(dir.path().join("empty.txt"), b"").unwrap();
            std::fs::write(
                dir.path().join("multiline.txt"),
                b"line one\nline two\nline three\n",
            )
            .unwrap();
            // binary with null bytes and high bytes
            std::fs::write(
                dir.path().join("binary.bin"),
                [0x00, 0xFF, 0x1B, 0x0A, 0x41, 0x00],
            )
            .unwrap();

            Self { dir, key_path }
        }

        fn path(&self, name: &str) -> PathBuf {
            self.dir.path().join(name)
        }

        fn key_files(&self) -> Vec<String> {
            vec![self.key_path.to_string_lossy().to_string()]
        }

        fn read(&self, name: &str) -> String {
            std::fs::read_to_string(self.path(name)).unwrap()
        }
    }

    // ----------------------------------------------------------------
    // encrypt_to_file / decrypt_to_file  (file -> file roundtrip)
    // ----------------------------------------------------------------

    #[test]
    fn file_roundtrip_plaintext() -> Result<(), AgeError> {
        let f = Fixture::new();
        let input = f.path("plaintext.txt");
        let encrypted = f.path("plaintext.txt.age");
        let decrypted = f.path("plaintext_out.txt");

        encrypt_to_file(&input, &encrypted, f.key_files())?;
        decrypt_to_file(&encrypted, &decrypted, f.key_files())?;

        assert_eq!(
            std::fs::read_to_string(&input).unwrap(),
            std::fs::read_to_string(&decrypted).unwrap()
        );
        Ok(())
    }

    #[test]
    fn file_roundtrip_multiline() -> Result<(), AgeError> {
        let f = Fixture::new();
        let input = f.path("multiline.txt");
        let encrypted = f.path("multiline.txt.age");
        let decrypted = f.path("multiline_out.txt");

        encrypt_to_file(&input, &encrypted, f.key_files())?;
        decrypt_to_file(&encrypted, &decrypted, f.key_files())?;

        assert_eq!(
            f.read("multiline.txt"),
            std::fs::read_to_string(&decrypted).unwrap()
        );
        Ok(())
    }

    #[test]
    fn file_roundtrip_empty_file() -> Result<(), AgeError> {
        let f = Fixture::new();
        let input = f.path("empty.txt");
        let encrypted = f.path("empty.txt.age");
        let decrypted = f.path("empty_out.txt");

        encrypt_to_file(&input, &encrypted, f.key_files())?;
        decrypt_to_file(&encrypted, &decrypted, f.key_files())?;

        assert_eq!(
            std::fs::read(&input).unwrap(),
            std::fs::read(&decrypted).unwrap()
        );
        Ok(())
    }

    #[test]
    fn encrypted_file_is_not_plaintext() -> Result<(), AgeError> {
        let f = Fixture::new();
        let input = f.path("plaintext.txt");
        let encrypted = f.path("plaintext.txt.age");

        encrypt_to_file(&input, &encrypted, f.key_files())?;

        let raw = std::fs::read_to_string(&encrypted).unwrap();

        // should be ASCII armored age format
        assert!(raw.contains("-----BEGIN AGE ENCRYPTED FILE-----"));
        assert!(raw.contains("-----END AGE ENCRYPTED FILE-----"));

        // must not contain the original plaintext
        assert!(!raw.contains("Hello, age!"));
        Ok(())
    }

    #[test]
    fn encrypted_files_are_nondeterministic() -> Result<(), AgeError> {
        // age uses ephemeral keys - two encryptions of the same plaintext
        // must produce different ciphertext
        let f = Fixture::new();
        let input = f.path("plaintext.txt");
        let enc1 = f.path("enc1.age");
        let enc2 = f.path("enc2.age");

        encrypt_to_file(&input, &enc1, f.key_files())?;
        encrypt_to_file(&input, &enc2, f.key_files())?;

        let c1 = std::fs::read(&enc1).unwrap();
        let c2 = std::fs::read(&enc2).unwrap();
        assert_ne!(c1, c2, "age encryption must be nondeterministic");
        Ok(())
    }

    #[test]
    fn decrypt_with_wrong_key_fails() {
        let f = Fixture::new();
        let wrong = Fixture::new(); // different key

        let input = f.path("plaintext.txt");
        let encrypted = f.path("plaintext.txt.age");
        let decrypted = f.path("out.txt");

        encrypt_to_file(&input, &encrypted, f.key_files()).unwrap();

        // decrypt using a completely different key
        let result = decrypt_to_file(&encrypted, &decrypted, wrong.key_files());
        assert!(result.is_err(), "decryption with wrong key must fail");
    }

    #[test]
    fn encrypt_nonexistent_file_fails() {
        let f = Fixture::new();
        let missing = f.path("does_not_exist.txt");
        let out = f.path("out.age");

        let result = encrypt_to_file(&missing, &out, f.key_files());
        assert!(result.is_err());
    }

    #[test]
    fn decrypt_nonexistent_file_fails() {
        let f = Fixture::new();
        let missing = f.path("does_not_exist.age");
        let out = f.path("out.txt");

        let result = decrypt_to_file(&missing, &out, f.key_files());
        assert!(result.is_err());
    }

    // ----------------------------------------------------------------
    // encrypt_path_to_string / decrypt_from_string  (string roundtrip)
    // ----------------------------------------------------------------

    #[test]
    fn string_roundtrip_from_file() -> Result<(), AgeError> {
        let f = Fixture::new();
        let input = f.path("plaintext.txt");
        let original = f.read("plaintext.txt");

        let encrypted = encrypt_path_to_string(&input, f.key_files())?;
        let decrypted = decrypt_from_string(encrypted, f.key_files())?;

        assert_eq!(original, decrypted);
        Ok(())
    }

    #[test]
    fn string_roundtrip_from_string() -> Result<(), AgeError> {
        let f = Fixture::new();
        let plaintext = "top secret value 🔑\n".to_owned();

        let encrypted = encrypt_to_string(plaintext.clone(), f.key_files())?;
        let decrypted = decrypt_from_string(encrypted, f.key_files())?;

        assert_eq!(plaintext, decrypted);
        Ok(())
    }

    #[test]
    fn string_roundtrip_empty_string() -> Result<(), AgeError> {
        let f = Fixture::new();

        let encrypted = encrypt_to_string("".to_owned(), f.key_files())?;
        let decrypted = decrypt_from_string(encrypted, f.key_files())?;

        assert_eq!("", decrypted);
        Ok(())
    }

    #[test]
    fn string_roundtrip_unicode() -> Result<(), AgeError> {
        let f = Fixture::new();
        let plaintext = "日本語テスト\nمرحبا\n🦀🔐\n".to_owned();

        let encrypted = encrypt_to_string(plaintext.clone(), f.key_files())?;
        let decrypted = decrypt_from_string(encrypted, f.key_files())?;

        assert_eq!(plaintext, decrypted);
        Ok(())
    }

    #[test]
    fn string_roundtrip_large_payload() -> Result<(), AgeError> {
        let f = Fixture::new();
        // 1MB of data
        let plaintext = "a".repeat(1024 * 1024);

        let encrypted = encrypt_to_string(plaintext.clone(), f.key_files())?;
        let decrypted = decrypt_from_string(encrypted, f.key_files())?;

        assert_eq!(plaintext, decrypted);
        Ok(())
    }

    #[test]
    fn encrypted_string_is_ascii_armored() -> Result<(), AgeError> {
        let f = Fixture::new();
        let encrypted = encrypt_to_string("secret".to_owned(), f.key_files())?;

        assert!(encrypted.contains("-----BEGIN AGE ENCRYPTED FILE-----"));
        assert!(encrypted.contains("-----END AGE ENCRYPTED FILE-----"));
        Ok(())
    }

    #[test]
    fn decrypt_from_string_with_wrong_key_fails() {
        let f = Fixture::new();
        let wrong = Fixture::new();

        let encrypted = encrypt_to_string("secret".to_owned(), f.key_files()).unwrap();
        let result = decrypt_from_string(encrypted, wrong.key_files());

        assert!(result.is_err());
    }

    #[test]
    fn decrypt_from_string_with_garbage_fails() {
        let f = Fixture::new();
        let garbage = "this is not an age encrypted file".to_owned();

        let result = decrypt_from_string(garbage, f.key_files());
        assert!(result.is_err());
    }

    // ----------------------------------------------------------------
    // decrypt_to_string
    // ----------------------------------------------------------------

    #[test]
    fn decrypt_to_string_matches_original() -> Result<(), AgeError> {
        let f = Fixture::new();
        let input = f.path("plaintext.txt");
        let encrypted = f.path("plaintext.txt.age");
        let original = f.read("plaintext.txt");

        encrypt_to_file(&input, &encrypted, f.key_files())?;
        let decrypted = decrypt_to_string(&encrypted, f.key_files())?;

        assert_eq!(original, decrypted);
        Ok(())
    }

    // ----------------------------------------------------------------
    // Multiple recipients
    // ----------------------------------------------------------------
    //
    // age supports encrypting to multiple recipients so any one of their
    // keys can decrypt. Test that this works correctly.
    //

    #[test]
    fn encrypt_to_multiple_recipients_any_key_can_decrypt() -> Result<(), AgeError> {
        let alice = Fixture::new();
        let bob = Fixture::new();

        // encrypt with both alice and bob's keys
        let both_keys = vec![
            alice.key_path.to_string_lossy().to_string(),
            bob.key_path.to_string_lossy().to_string(),
        ];

        let plaintext = "shared secret".to_owned();
        let encrypted = encrypt_to_string(plaintext.clone(), both_keys)?;

        // alice can decrypt
        let dec_alice = decrypt_from_string(encrypted.clone(), alice.key_files())?;
        assert_eq!(plaintext, dec_alice);

        // bob can also decrypt
        let dec_bob = decrypt_from_string(encrypted.clone(), bob.key_files())?;
        assert_eq!(plaintext, dec_bob);

        Ok(())
    }

    #[test]
    fn third_party_cannot_decrypt_multi_recipient() -> Result<(), AgeError> {
        let alice = Fixture::new();
        let bob = Fixture::new();
        let eve = Fixture::new(); // not a recipient

        let both_keys = vec![
            alice.key_path.to_string_lossy().to_string(),
            bob.key_path.to_string_lossy().to_string(),
        ];

        let encrypted = encrypt_to_string("secret".to_owned(), both_keys)?;
        let result = decrypt_from_string(encrypted, eve.key_files());

        assert!(result.is_err());
        Ok(())
    }

    // ----------------------------------------------------------------
    // get_full_path
    // ----------------------------------------------------------------

    #[test]
    fn get_full_path_absolute_path() {
        let f = Fixture::new();
        let abs = f.key_path.to_string_lossy().to_string();

        let result = get_full_path(&abs);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), f.key_path);
    }

    #[test]
    fn get_full_path_nonexistent_fails() {
        let f = Fixture::new();
        let missing = f.path("no_such_file.txt").to_string_lossy().to_string();

        assert!(get_full_path(&missing).is_err());
    }

    #[test]
    fn get_full_path_directory_fails() {
        // directories are not files - should return Err
        let f = Fixture::new();
        let dir = f.dir.path().to_string_lossy().to_string();

        assert!(get_full_path(&dir).is_err());
    }

    #[test]
    fn get_full_path_tilde_expansion() {
        // Only meaningful if HOME is set, which it always is in CI/dev
        let home = match std::env::var("HOME") {
            Ok(h) => h,
            Err(_) => return, // skip if HOME not set
        };

        // Create a real file under HOME to test tilde expansion
        let tmp = tempfile::NamedTempFile::new_in(&home).unwrap();
        let filename = tmp
            .path()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let tilde_path = format!("~/{}", filename);

        let result = get_full_path(&tilde_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), tmp.path());
    }

    #[test]
    fn get_full_path_empty_string_fails() {
        assert!(get_full_path("").is_err());
    }
}
