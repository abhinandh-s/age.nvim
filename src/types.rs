use std::borrow::Cow;
use std::fmt::Display;
use std::path::{Path, PathBuf};

/// Guarantees
///
/// 1. File exists
/// 2. It have `.age` extension
///
#[derive(Debug)]
pub(crate) struct ExistingAgeFile<'a>(Cow<'a, Path>);

impl<'a> ExistingAgeFile<'a> {
    pub(crate) fn path(&self) -> &Path {
        &self.0
    }

    pub(crate) fn strip_age(&self) -> PathBuf {
        self.0.with_extension("")
    }
}

/// Gives `.to_string()` for free
impl Display for ExistingAgeFile<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string_lossy())
    }
}

impl TryFrom<PathBuf> for ExistingAgeFile<'_> {
    type Error = &'static str;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        if !path.exists() {
            return Err("File does not exist.");
        }

        if !path
            .extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("age"))
        {
            return Err("File does not have `.age` extension.");
        }

        Ok(Self(Cow::Owned(path)))
    }
}

#[derive(Debug)]
pub(crate) struct ExistingNonAgeFile<'a>(Cow<'a, Path>);

impl<'a> ExistingNonAgeFile<'a> {
    pub(crate) fn path(&self) -> &Path {
        &self.0
    }

    pub(crate) fn append_age(&self) -> PathBuf {
        self.0.with_added_extension("age")
    }
}

impl TryFrom<PathBuf> for ExistingNonAgeFile<'_> {
    type Error = &'static str;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        if !path.exists() {
            return Err("File does not exist.");
        }

        if path
            .extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("age"))
        {
            return Err("File have `.age` extension, it's already encrypted.");
        }

        Ok(Self(Cow::Owned(path)))
    }
}

impl TryFrom<&str> for ExistingNonAgeFile<'_> {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        PathBuf::from(s).try_into()
    }
}

/// Gives `.to_string()` for free
impl Display for ExistingNonAgeFile<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string_lossy())
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use std::path::{Path, PathBuf};
    use std::sync::OnceLock;

    use tempfile::TempDir;

    use crate::error::AgeError;
    use crate::types::{ExistingAgeFile, ExistingNonAgeFile};

    static TEST_DIR: OnceLock<PathBuf> = OnceLock::new();

    fn test_dir() -> &'static PathBuf {
        TEST_DIR.get_or_init(|| {
            let base = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| "".into());
            PathBuf::from(base).join("tests/types/some_dirs")
        })
    }

    macro_rules! test_age_file {
        ($str:literal) => {
            ExistingAgeFile::try_from(test_dir().join($str))
        };
    }

    macro_rules! test_non_age_file {
        ($str:literal) => {
            ExistingNonAgeFile::try_from(test_dir().join($str))
        };
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn existing_age_file_t() -> Result<(), AgeError> {
        let ext_path_01 = test_age_file!("file.txt.age");
        let ext_path_02 = test_age_file!("doesntextstfile.txt.age");
        let ext_path_03 = test_age_file!("doesntextstfile.txt");
        let ext_path_04 = test_age_file!("file.txt");

        assert!(ext_path_01.is_ok());
        assert!(ext_path_02.is_err());
        assert!(ext_path_03.is_err());
        assert!(ext_path_04.is_err());

        let binding = ext_path_01?;
        let strip_age = binding.strip_age();
        assert_eq!(strip_age, Path::new(test_dir().join("file.txt").as_path()));
        Ok(())
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn existing_non_age_file_t() -> Result<(), AgeError> {
        let ext_path_01 = test_non_age_file!("file.txt.age");
        let ext_path_02 = test_non_age_file!("doesntextstfile.txt.age");
        let ext_path_03 = test_non_age_file!("doesntextstfile.txt");
        let ext_path_04 = test_non_age_file!("file.txt");

        assert!(ext_path_01.is_err());
        assert!(ext_path_02.is_err());
        assert!(ext_path_03.is_err());
        assert!(ext_path_04.is_ok());

        let binding = ext_path_04?;
        let strip_age = binding.append_age();
        assert_eq!(
            strip_age,
            Path::new(test_dir().join("file.txt.age").as_path())
        );
        Ok(())
    }

    struct Fixture {
        dir: TempDir,
    }

    impl Fixture {
        fn new() -> Self {
            let dir = tempfile::tempdir().expect("failed to create temp dir");

            std::fs::write(dir.path().join("real.txt"), b"plaintext").unwrap();
            std::fs::write(dir.path().join("real.txt.age"), b"ciphertext").unwrap();
            std::fs::write(dir.path().join("empty.age"), b"").unwrap();

            Self { dir }
        }

        fn path(&self, name: &str) -> PathBuf {
            self.dir.path().join(name)
        }

        // A path that is guaranteed not to exist
        fn ghost(&self, name: &str) -> PathBuf {
            self.dir.path().join(name)
        }
    }

    // ## ExistingAgeFile
    //
    // Invariants:
    //   1. File must exist on disk
    //   2. File must have `.age` extension

    #[test]
    fn age_file_accepts_existing_age_file() {
        let f = Fixture::new();
        assert!(ExistingAgeFile::try_from(f.path("real.txt.age")).is_ok())
    }

    #[test]
    fn age_file_rejects_nonexistent_file() {
        let f = Fixture::new();
        let result = ExistingAgeFile::try_from(f.ghost("ghost.txt.age"));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "File does not exist.");
    }

    #[test]
    fn age_file_rejects_existing_non_age_file() {
        let f = Fixture::new();
        let result = ExistingAgeFile::try_from(f.path("real.txt"));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "File does not have `.age` extension.");
    }

    #[test]
    fn age_file_rejects_nonexistent_non_age_file() {
        let f = Fixture::new();
        // nonexistent AND wrong extension - should fail on existence first
        let result = ExistingAgeFile::try_from(f.ghost("ghost.txt"));
        assert!(result.is_err());
    }

    #[test]
    fn age_file_extension_check_is_case_insensitive() {
        let f = Fixture::new();

        // Create files with mixed-case .AGE extension
        std::fs::write(f.path("upper.AGE"), b"data").unwrap();
        std::fs::write(f.path("mixed.Age"), b"data").unwrap();

        assert!(ExistingAgeFile::try_from(f.path("upper.AGE")).is_ok());
        assert!(ExistingAgeFile::try_from(f.path("mixed.Age")).is_ok());
    }

    #[test]
    fn age_file_path_returns_original_path() {
        let f = Fixture::new();
        let expected = f.path("real.txt.age");
        let age_file = ExistingAgeFile::try_from(expected.clone()).unwrap();

        assert_eq!(age_file.path(), expected.as_path());
    }

    #[test]
    fn age_file_strip_age_removes_extension() {
        let f = Fixture::new();
        let age_file = ExistingAgeFile::try_from(f.path("real.txt.age")).unwrap();
        let stripped = age_file.strip_age();

        assert_eq!(stripped, f.path("real.txt"));
        assert_eq!(stripped.extension().unwrap(), "txt");
    }

    #[test]
    fn age_file_strip_age_on_bare_age_extension() {
        // "empty.age" -> "empty" (no remaining extension)
        let f = Fixture::new();
        let age_file = ExistingAgeFile::try_from(f.path("empty.age")).unwrap();
        let stripped = age_file.strip_age();

        assert_eq!(stripped.extension(), None);
        assert_eq!(stripped.file_name().unwrap(), "empty");
    }

    #[test]
    fn age_file_display() {
        let f = Fixture::new();
        let path = f.path("real.txt.age");
        let age_file = ExistingAgeFile::try_from(path.clone()).unwrap();

        assert_eq!(age_file.to_string(), path.to_string_lossy());
    }

    // ## ExistingNonAgeFile
    //
    // Invariants:
    //   1. File must exist on disk
    //   2. File must NOT have `.age` extension

    #[test]
    fn non_age_file_accepts_existing_plain_file() {
        let f = Fixture::new();
        assert!(ExistingNonAgeFile::try_from(f.path("real.txt")).is_ok());
    }

    #[test]
    fn non_age_file_rejects_existing_age_file() {
        let f = Fixture::new();
        let result = ExistingNonAgeFile::try_from(f.path("real.txt.age"));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "File have `.age` extension, it's already encrypted."
        );
    }

    #[test]
    fn non_age_file_rejects_nonexistent_file() {
        let f = Fixture::new();
        let result = ExistingNonAgeFile::try_from(f.ghost("ghost.txt"));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "File does not exist.");
    }

    #[test]
    fn non_age_file_from_str_matches_from_pathbuf() {
        let f = Fixture::new();
        let path = f.path("real.txt");

        let from_pathbuf = ExistingNonAgeFile::try_from(path.clone());
        let from_str = ExistingNonAgeFile::try_from(path.to_str().unwrap());

        // Both should succeed and point to the same path
        assert!(from_pathbuf.is_ok());
        assert!(from_str.is_ok());
        assert_eq!(from_pathbuf.unwrap().path(), from_str.unwrap().path());
    }

    #[test]
    fn non_age_file_from_str_rejects_age_file() {
        let f = Fixture::new();
        let path = f.path("real.txt.age");
        let result = ExistingNonAgeFile::try_from(path.to_str().unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn non_age_file_append_age_adds_extension() {
        let f = Fixture::new();
        let file = ExistingNonAgeFile::try_from(f.path("real.txt")).unwrap();
        let appended = file.append_age();

        assert_eq!(appended, f.path("real.txt.age"));
        assert_eq!(appended.extension().unwrap(), "age");
    }

    #[test]
    fn non_age_file_path_returns_original_path() {
        let f = Fixture::new();
        let expected = f.path("real.txt");
        let file = ExistingNonAgeFile::try_from(expected.clone()).unwrap();

        assert_eq!(file.path(), expected.as_path());
    }

    #[test]
    fn non_age_file_display() {
        let f = Fixture::new();
        let path = f.path("real.txt");
        let file = ExistingNonAgeFile::try_from(path.clone()).unwrap();

        assert_eq!(file.to_string(), path.to_string_lossy());
    }

    // ## Roundtrip
    //
    // These test that strip_age and append_age are true inverses,
    // which is a core correctness property of the whole encrypt/decrypt flow.

    #[test]
    fn strip_then_append_is_identity() {
        let f = Fixture::new();
        let original = f.path("real.txt.age");

        let stripped = ExistingAgeFile::try_from(original.clone())
            .unwrap()
            .strip_age();

        // stripped path doesn't exist yet, so we create it to test append
        std::fs::write(&stripped, b"plaintext").unwrap();

        let appended = ExistingNonAgeFile::try_from(stripped).unwrap().append_age();

        assert_eq!(appended, original);
    }

    #[test]
    fn append_then_strip_is_identity() {
        let f = Fixture::new();
        let original = f.path("real.txt");

        let appended = ExistingNonAgeFile::try_from(original.clone())
            .unwrap()
            .append_age();

        // appended path doesn't exist yet, so create it
        std::fs::write(&appended, b"ciphertext").unwrap();

        let stripped = ExistingAgeFile::try_from(appended).unwrap().strip_age();

        assert_eq!(stripped, original);
    }
}
