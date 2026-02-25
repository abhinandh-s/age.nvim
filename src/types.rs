#![deny(clippy::case_sensitive_file_extension_comparisons)]

use std::borrow::Cow;
use std::fmt::Display;
use std::path::{Path, PathBuf};

use crate::error::AgeError;

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

    pub(crate) fn strip_age(&mut self) -> Result<PathBuf, AgeError> {
        Ok(self.0.with_extension(""))
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

    pub(crate) fn append_age(&mut self) -> Result<PathBuf, AgeError> {
        let mut new_path = self.0.to_path_buf();
        let new_name = &self
            .0
            .file_name()
            .map(|name| {
                let mut n = name.to_os_string();
                n.push(".age");
                n
            })
            .ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidInput, "Path has no filename")
            })?;

        new_path.set_file_name(new_name);
        self.0 = new_path.into();

        Ok(self.0.to_path_buf())
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

    #[test]
    #[allow(clippy::unwrap_used)]
    fn existing_age_file_t() -> Result<(), AgeError> {
        let ext_path_01 = ExistingAgeFile::try_from(test_dir().join("file.txt.age"));
        let ext_path_02 = ExistingAgeFile::try_from(test_dir().join("doesntextstfile.txt.age"));
        let ext_path_03 = ExistingAgeFile::try_from(test_dir().join("doesntextstfile.txt"));
        let ext_path_04 = ExistingAgeFile::try_from(test_dir().join("file.txt"));

        assert!(ext_path_01.is_ok());
        assert!(ext_path_02.is_err());
        assert!(ext_path_03.is_err());
        assert!(ext_path_04.is_err());

        let mut binding = ext_path_01?;
        let strip_age = binding.strip_age()?;
        assert_eq!(strip_age, Path::new(test_dir().join("file.txt").as_path()));
        Ok(())
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn existing_non_age_file_t() -> Result<(), AgeError> {
        let ext_path_01 = ExistingNonAgeFile::try_from(test_dir().join("file.txt.age"));
        let ext_path_02 = ExistingNonAgeFile::try_from(test_dir().join("doesntextstfile.txt.age"));
        let ext_path_03 = ExistingNonAgeFile::try_from(test_dir().join("doesntextstfile.txt"));
        let ext_path_04 = ExistingNonAgeFile::try_from(test_dir().join("file.txt"));

        assert!(ext_path_01.is_err());
        assert!(ext_path_02.is_err());
        assert!(ext_path_03.is_err());
        assert!(ext_path_04.is_ok());

        let mut binding = ext_path_04?;
        let strip_age = binding.append_age()?;
        assert_eq!(
            strip_age,
            Path::new(test_dir().join("file.txt.age").as_path())
        );
        Ok(())
    }
}
