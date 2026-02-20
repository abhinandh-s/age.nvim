#[derive(Debug)]
pub struct AgeError(String);

impl AgeError {
    pub fn new(err: String) -> Self {
        Self(err)
    }
}

impl std::fmt::Display for AgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<AgeError> for nvim_oxi::Error {
    fn from(value: AgeError) -> Self {
        nvim_oxi::Error::Api(nvim_oxi::api::Error::Other(value.to_string()))
    }
}

impl From<&str> for AgeError {
    fn from(msg: &str) -> Self {
        AgeError(if msg.to_lowercase().starts_with("error") {
            msg.to_owned()
        } else {
            format!("Error: {}", msg)
        })
    }
}

impl From<Box<dyn std::error::Error>> for AgeError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        AgeError(err.to_string())
    }
}

macro_rules! impl_age_err {
    ($($from:path),* $(,)?) => {
        impl std::error::Error for AgeError {}
        $(
            impl From<$from> for AgeError {
                fn from(err: $from) -> Self {
                    let string = err.to_string();
                    AgeError(
                        if string.to_lowercase().starts_with("error") {
                            string
                        } else {
                            format!("Error: {}", string)
                        }
                    )
                }
            }
        )*
    }
}

impl_age_err![
    nvim_oxi::Error,
    nvim_oxi::api::Error,
    std::io::Error,
    std::env::VarError,
    std::str::Utf8Error,
    std::string::FromUtf8Error,
    age::EncryptError,
    age::DecryptError,
];
