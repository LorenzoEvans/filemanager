use std::fs::File;
use std::path::PathBuf;
use std::io;

pub fn create_file(file_name: PathBuf, directory: Option<PathBuf>) -> io::Result<()> {
    let full_path = if let Some(dir) = directory {
        dir.join(file_name)
    } else {
        file_name
    };

    File::create(full_path)?;
    Ok(())
}
