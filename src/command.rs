use nvim_oxi::Function;

#[derive(Debug)]
pub enum Command {
    EncryptFile,
    DecryptFile,
    #[cfg(feature = "mail")]
    SentMail,
    NewFileName(Option<String>),
}

/// Parses a command and its argument from strings.
///
/// This function takes a command string and an optional argument, and returns
/// a corresponding `Command` variant if the input is valid. The `set_font` command
/// accepts an optional argument, while other commands may require or ignore arguments.
///
/// # Arguments
///
/// * `cmd` - A string representing the command.
/// * `arg` - An optional argument for the command. For example, a font family name for `set_font`.
///
/// # Returns
///
/// Returns `Some(Command)` if the input matches a known command. Returns `None` if the command is unrecognized.
impl Command {
    pub fn from_str(cmd: &str, arg: Option<&str>) -> Option<Self> {
        match cmd {
            "" => {
                let filename = arg.map(|s| s.to_string());
                Some(Command::NewFileName(filename))
            }
            #[cfg(feature = "mail")]
            "sent" => Some(Command::SentMail),
            "d" => Some(Command::DecryptFile),
            "e" => Some(Command::EncryptFile),
            "decrypt" => Some(Command::DecryptFile),
            "encrypt" => Some(Command::EncryptFile),
            "new" => {
                let filename = arg.map(|s| s.to_string());
                Some(Command::NewFileName(filename))
            }
            _ => None,
        }
    }
}

pub fn completion() -> Function<(String, String, usize), Vec<String>> {
    Function::from_fn({
        move |args: (String, String, usize)| {
            let (arg_lead, cmd_line, cursor_pos) = args;

            let split_cmd_line: Vec<&str> = cmd_line.split_whitespace().collect();
            let args_after_command = &split_cmd_line[1..];

            let mut current_arg_index = 0;

            for (index, &arg) in args_after_command.iter().enumerate() {
                if let Some(start_pos) = cmd_line.find(arg) {
                    let end_pos = start_pos + arg.len();
                    if cursor_pos >= start_pos && cursor_pos <= end_pos {
                        current_arg_index = index;
                        break;
                    }
                }
            }

            if current_arg_index > 0 {
                vec![]
            } else {
                let completions = vec![
                    "new".into(),
                    "decrypt".into(),
                    "encrypt".into(),
                    #[cfg(feature = "mail")]
                    "sent".into(),
                ];
                completions
                    .into_iter()
                    .filter(|c: &String| c.starts_with(&arg_lead))
                    .collect::<Vec<_>>()
            }
        }
    })
}
