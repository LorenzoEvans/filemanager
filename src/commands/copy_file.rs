use std::io;
use std::path::PathBuf;
use std::fs;
use walkdir::WalkDir;

pub fn cpy_file(
    source_path: PathBuf,
    destination_path: PathBuf,
    file_name_to_search: &Option<PathBuf>,
) -> io::Result<()> {
    let source_file = if source_path.is_file() {
        source_path
    } else if source_path.is_dir() {
        if let Some(name) = file_name_to_search {
            find_file_in_dir(&source_path, name)?
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "file_name is required when source_path is a directory",
            ));
        }
    } else {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Source path not found: {:?}", source_path),
        ));
    };

    let destination = if destination_path.is_dir() {
        let name = source_file.file_name().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "Invalid source file name")
        })?;
        destination_path.join(name)
    } else {
        destination_path
    };

    // Use standard fs::copy for efficiency
    fs::copy(&source_file, &destination)?;

    Ok(())
}

fn find_file_in_dir(dir: &PathBuf, target_name: &PathBuf) -> io::Result<PathBuf> {
    for entry in WalkDir::new(dir).into_iter() {
        let entry = entry.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        if entry.file_type().is_file() {
            if let Some(name) = entry.path().file_name() {
                if name == target_name {
                    return Ok(entry.path().to_path_buf());
                }
            }
        }
    }
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!("File {:?} not found in directory {:?}", target_name, dir),
    ))
}
