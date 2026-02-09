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
                let completions = vec!["decrypt".into(), "encrypt".into(), "genkey".into()];
                completions
                    .into_iter()
                    .filter(|c: &String| c.starts_with(&arg_lead))
                    .collect::<Vec<_>>()
            }
        }
    })
}
