use std::fs;
use std::path::PathBuf;

pub fn parse_input(file_path: &Option<PathBuf>) -> Result<String, std::io::Error> {
    if let Some(path) = file_path {
        let file_contents = fs::read_to_string(path)?;
        return Ok(file_contents);
    }

    todo!("read from stdin");
}
