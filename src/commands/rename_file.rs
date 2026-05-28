use std::{fs, path::PathBuf, io};

pub fn rn_file(old_name: PathBuf, new_name: PathBuf) -> io::Result<()> {
    fs::rename(old_name, new_name)?;
    Ok(())
}
