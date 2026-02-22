use std::path::Path;

use nvim_oxi::Function;

use crate::error::AgeError;

#[derive(Debug)]
pub enum Command {
    EncryptFile,
    DecryptFile,
    GenKey,
}

/// Parses a command and its argument from strings.
///
/// This function takes a command string and an optional argument, and returns
/// a corresponding `Command` variant if the input is valid.
impl Command {
    pub fn from_str(cmd: &str) -> Option<Self> {
        match cmd {
            "" => None,
            "d" => Some(Command::DecryptFile),
            "e" => Some(Command::EncryptFile),
            "g" => Some(Command::GenKey),
            "decrypt" => Some(Command::DecryptFile),
            "encrypt" => Some(Command::EncryptFile),
            "genkey" => Some(Command::GenKey),
            _ => None,
        }
    }
}

pub fn completion() -> Function<(String, String, usize), Vec<String>> {
    Function::from_fn({
        move |args: (String, String, usize)| {
            let (arg_lead, cmd_line, _cursor_pos) = args;

            //  ["Age", "encrypt", ""]
            //  arguments[0] = "Age"
            //  arguments[1] = "encrypt"
            //  arguments[2..] = Otional file paths
            let arguments: Vec<&str> = cmd_line.split_whitespace().collect();

            let is_first_arg = if cmd_line.ends_with(' ') {
                arguments.len() < 2
            } else {
                arguments.len() <= 2
            };

            if is_first_arg {
                let completions = vec!["decrypt".into(), "encrypt".into(), "genkey".into()];

                return completions
                    .into_iter()
                    .filter(|c: &String| c.starts_with(&arg_lead))
                    .collect::<Vec<_>>();
            }
            // should we provide file paths ???
            let last_arg = arguments
                .last()
                .map(|s| s.to_string())
                .unwrap_or("".to_owned());
            get_key_files(last_arg).unwrap_or_default()
        }
    })
}

fn get_key_files(arg: String) -> Result<Vec<String>, AgeError> {
    let input = match arg.starts_with('~') {
        true => expand_tilde(arg).to_string_lossy().to_string(),
        false => {
            if arg.starts_with('/') {
                arg
            } else {
                std::env::current_dir()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()
            }
        }
    };

    let mut key_files = Vec::new();

    let walker = walkdir::WalkDir::new(input).into_iter();

    for entry in walker.filter_entry(|e| !should_skip(e)) {
        let filenane = entry?.path().to_path_buf();
        let name = filenane
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
        if name.ends_with("key.txt") || name.ends_with("keys.txt") {
            key_files.push(filenane.to_string_lossy().to_string());
        }
    }
    Ok(key_files)
}

fn should_skip(entry: &walkdir::DirEntry) -> bool {
    for blacklisted in [".git", ".cargo", ".cache", "node_modules", ".DS_Store"] {
        let s = entry
            .file_name()
            .to_str()
            .map(|s| s.starts_with(blacklisted))
            .unwrap_or(false);
        if s {
            return true;
        }
    }

    false
}

fn expand_tilde<P: AsRef<Path>>(path: P) -> std::path::PathBuf {
    let p = path.as_ref();
    if !p.starts_with("~") {
        return p.to_path_buf();
    }

    if let Some(home_dir) = std::env::home_dir() {
        if p == Path::new("~") {
            return home_dir;
        }

        if let Ok(suffix) = p.strip_prefix("~") {
            return Path::new(&home_dir).join(suffix);
        }
    }

    p.to_path_buf()
}
