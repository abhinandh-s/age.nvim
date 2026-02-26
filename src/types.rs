use std::borrow::Cow;
use std::fmt::Display;
use std::path::{Path, PathBuf};

/// Guarantees
///
/// 1. File exists
/// 2. It have `.age` extension
///
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
            return Err("File does not exists.");
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
            return Err("File does not exists.");
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
        let path = Path::new(s).to_path_buf();
        if !path.exists() {
            return Err("File does not exists.");
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

/// Gives `.to_string()` for free
impl Display for ExistingNonAgeFile<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string_lossy())
    }
}

#[cfg(test)]
mod test {
    use std::path::{Path, PathBuf};
    use std::sync::OnceLock;

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
}
