use crate::protocol::command::Command;
use crate::types::error::ParseError;

pub fn parse(input: &str) -> Result<Command, ParseError> {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    
    if tokens.is_empty() {
        return Err(ParseError::EmptyCommand);
    }

    match tokens.as_slice() {

        [] => Err(ParseError::EmptyCommand),

        ["SET", key, value] => Ok(Command::Set {
            key: (*key).to_string(),
            value: (*value).to_string(),
        }),

        ["GET", key] => Ok(Command::Get {
            key: (*key).to_string(),
        }),

        ["DELETE", key] => Ok(Command::Delete {
                key: (*key).to_string(),
        }),

        ["EXISTS", key] => Ok(Command::Exists {
                key: (*key).to_string(),
        }),

        [command, ..] if matches!(*command, "SET" | "GET" | "DELETE" | "EXISTS") => {
            Err(ParseError::InvalidArgumentCommand)
        }
        
        _ => Err(ParseError::UnknownCommand),
    }
}
