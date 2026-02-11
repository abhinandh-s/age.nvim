use nvim_oxi::Function;

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
            let (arg_lead, cmd_line, _) = args;

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
            vec![]
        }
    })
}
