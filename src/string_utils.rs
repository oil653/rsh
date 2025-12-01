pub fn split_args(command: &String) -> Vec<String> {
    let mut iter = command.trim().chars().peekable();

    let mut args: Vec<String> = Vec::new();
    let mut buffer: String = String::new();

    let mut in_single_quote = false;
    let mut in_double_quote = false;

    let mut is_escaping = false;

    while let Some(char) = iter.next() {
        if is_escaping {
            buffer.push(char);
            is_escaping = false;
            continue;
        }

        if char == '\\' && !in_single_quote { 
            if in_double_quote {
                match iter.peek() {
                    Some('\"') | Some('\\') => {
                        buffer.push(iter.next().unwrap());
                        continue;
                    },
                    _ => {
                        buffer.push('\\');
                        continue;
                    }
                }
            } else {
                is_escaping = true;
                continue;
            }
        }

        // ===
        if char == '\"' && !in_single_quote {
            in_double_quote = !in_double_quote;
        } 
        else if char == '\'' {
            if in_single_quote {
                // In single quotes, close them
                in_single_quote = false;
            } else if !in_double_quote {
                // Outside of single quotes, open one
                in_single_quote = true;
            } else {
                // Magic
                buffer.push('\'');
            }
            continue;
        } 
        else if char.is_whitespace() && !in_single_quote && !in_double_quote {
            if !buffer.is_empty(){
                args.push(std::mem::take(&mut buffer));
            }
        } 
        else {
            buffer.push(char);
        }
    }
    args.push(std::mem::take(&mut buffer));
    args
}