use std::fs;
use std::path::PathBuf;
use std::io;

pub fn mv_file(src_file: PathBuf, target_dir: PathBuf) -> io::Result<()> {
    if !src_file.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Source file not found: {:?}", src_file),
        ));
    }

    // If target_dir is actually a directory, we should move the file INTO it
    let destination = if target_dir.is_dir() {
        let file_name = src_file.file_name().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "Invalid source file name")
        })?;
        target_dir.join(file_name)
    } else {
        target_dir
    };

    match fs::rename(&src_file, &destination) {
        Ok(_) => Ok(()),
        Err(_e) => {
            // Fallback for cross-device moves or other errors where rename fails but copy might work
            fs::copy(&src_file, &destination)?;
            fs::remove_file(&src_file)?;
            Ok(())
        }
    }
}
