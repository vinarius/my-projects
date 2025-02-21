use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::Result;

pub fn get_command_path(command: &str) -> Option<PathBuf> {
    let path_maybe = env::var("PATH");

    if let Err(error) = path_maybe {
        eprintln!("{error}");

        return None;
    }

    let path = path_maybe.unwrap();
    let path_split_by_directories = path.split(':');

    for directory in path_split_by_directories {
        let command_path = Path::new(directory).join(command);

        if command_path.exists() {
            return Some(command_path);
        }
    }

    None
}

pub fn try_external_program(command: &str, _options: std::str::SplitWhitespace) -> Result<()> {
    let command_path_maybe = get_command_path(command);

    if command_path_maybe.is_none() {
        println!("{command}: not found");
        return Ok(());
    }

    let command_path = command_path_maybe.unwrap();
    let child = Command::new(command_path).spawn()?;
    let _ = child.wait_with_output();

    Ok(())
}
