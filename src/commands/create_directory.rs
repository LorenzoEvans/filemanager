use std::path::PathBuf;
use std::fs::DirBuilder;
use std::io;

pub fn create_dir(dir_name: PathBuf, parent_dir: Option<PathBuf>) -> io::Result<()>  {
    let mut builder = DirBuilder::new();
    builder.recursive(true); // Usually better to be recursive if we are specifying a path

    let target_path = if let Some(parent) = parent_dir {
        parent.join(dir_name)
    } else {
        dir_name
    };

    builder.create(target_path)?;

    Ok(())
}
