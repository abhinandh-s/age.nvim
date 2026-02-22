use std::borrow::Cow;
use std::ffi::OsStr;
use std::fmt::Display;
use std::path::{Path, PathBuf};

/// Guarantees
///
/// 1. File exists
/// 2. It have `.age` extension
///
pub(crate) struct ExistingAgeFile<'a>(Cow<'a, Path>);

impl<'a> ExistingAgeFile<'a> {
    #[allow(dead_code)]
    pub(crate) fn new<S>(s: &'a S) -> Option<Self>
    where
        S: AsRef<OsStr> + ?Sized,
    {
        let path = Path::new(s);

        if !path.exists() {
            return None;
        }

        if !path
            .extension()
            .map(|e| e.to_string_lossy().contains("age"))?
        {
            return None;
        }

        Some(Self(Cow::Borrowed(path)))
    }

    pub(crate) fn path(&self) -> &Path {
        &self.0
    }

    // Guarantees
    //
    // 1. File exists
    // 2. It have `.age` extension
    //
    pub(crate) fn stem_name(&self) -> &str {
        self.0.file_stem().and_then(|s| s.to_str()).unwrap_or("")
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

        if path
            .extension()
            .map(|e| e.eq_ignore_ascii_case("age"))
            .is_none()
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
}

impl TryFrom<PathBuf> for ExistingNonAgeFile<'_> {
    type Error = String;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        if !path.exists() {
            return Err("File does not exists.".to_owned());
        }

        if path
            .extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("age"))
        {
            let err = format!(
                "input: {}\nFile have `.age` extension, it's already encrypted.",
                path.display()
            );
            return Err(err);
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
    use crate::types::ExistingAgeFile;

    #[test]
    #[allow(clippy::unwrap_used)]
    fn existing_age_file_t() {
        let s = std::env::var("PWD").unwrap();
        let true_01 = s.clone() + "/tests/some/dir/file.txt.age";
        let false_01 = s.clone() + "/tests/some/dir/doesntextstfile.txt.age";
        let false_02 = s.clone() + "/tests/some/dir/doesntextstfile.txt";
        let false_03 = s.clone() + "/tests/some/dir/file.txt";
        let ext_path_01 = ExistingAgeFile::new(&true_01);
        let ext_path_02 = ExistingAgeFile::new(&false_01);
        let ext_path_03 = ExistingAgeFile::new(&false_02);
        let ext_path_04 = ExistingAgeFile::new(&false_03);
        assert!(ext_path_01.is_some());
        assert!(ext_path_02.is_none());
        assert!(ext_path_03.is_none());
        assert!(ext_path_04.is_none());
    }
}
