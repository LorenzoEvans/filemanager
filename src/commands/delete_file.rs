use std::fs;
use std::path::PathBuf;
use std::io;

pub fn del_file(file_path: PathBuf, directory: Option<PathBuf>) -> io::Result<()> {
    let full_path = if let Some(dir) = directory {
        dir.join(file_path)
    } else {
        file_path
    };

    if !full_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Path not found: {:?}", full_path),
        ));
    }

    if full_path.is_dir() {
        fs::remove_dir_all(full_path)?;
    } else {
        fs::remove_file(full_path)?;
    }

    Ok(())
}
