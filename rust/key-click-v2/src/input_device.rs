use std::fs;
use std::path::{Path, PathBuf};

pub fn find_keyboard(override_path: Option<&str>) -> Result<PathBuf, ()> {
    if let Some(path) = override_path {
        return Ok(PathBuf::from(path));
    }

    let by_id = Path::new("/dev/input/by-id");

    let mut keyboards: Vec<PathBuf> = fs::read_dir(by_id)
        .map_err(|_| ())?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.ends_with("event-kbd"))
                .unwrap_or(false)
        })
        .collect();

    keyboards.sort();

    match keyboards.len() {
        0 => Err(()),
        1 => {
            eprintln!("keyboard: {}", keyboards[0].display());
            Ok(keyboards.remove(0))
        }
        _ => {
            eprintln!("multiple keyboards found, using first:");
            for kb in &keyboards {
                eprintln!("  {}", kb.display());
            }
            Ok(keyboards.remove(0))
        }
    }
}
