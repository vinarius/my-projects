use std::str::FromStr;

pub enum BuiltInCommand {
    Echo,
    Exit,
    r#Type,
}

impl FromStr for BuiltInCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "echo" => Ok(BuiltInCommand::Echo),
            "exit" => Ok(BuiltInCommand::Exit),
            "type" => Ok(BuiltInCommand::Type),
            _ => Err(()),
        }
    }
}
