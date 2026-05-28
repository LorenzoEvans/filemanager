use file_format::{FileFormat, Kind};
use std::{fmt, path::PathBuf, io};

#[derive(Debug)]
pub struct FileProperty {
    file_name: String,
    is_dir: bool,
    is_file: bool,
    file_ext: String,
    file_kind: Kind,
    file_format: FileFormat,
    byte_len: u64,
}

impl fmt::Display for FileProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file_name = &self.file_name;
        writeln!(f, "The properties of {} are: ", file_name)?;
        writeln!(f, "  Is a directory: {}", self.is_dir)?;
        writeln!(f, "  Is a file:      {}", self.is_file)?;
        writeln!(f, "  Extension:      {}", self.file_ext)?;
        writeln!(f, "  Kind:           {:?}", self.file_kind)?;
        writeln!(f, "  Format:         {}", self.file_format)?;
        writeln!(f, "  Byte length:    {}", self.byte_len)?;
        Ok(())
    }
}

pub fn file_prop(file: PathBuf) -> io::Result<()> {
    let metadata = file.metadata()?;
    let byte_len = metadata.len();

    let format = FileFormat::from_file(&file)?;
    let file_kind = format.kind();

    let file_ext = file.extension()
        .and_then(|s| s.to_str())
        .unwrap_or("None")
        .to_string();
    
    let is_dir = file.is_dir();
    let is_file = file.is_file();
    
    let file_name = file.file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("Unknown")
        .to_string();

    let file_property = FileProperty {
        file_name,
        is_dir,
        is_file,
        file_ext,
        file_kind,
        file_format: format,
        byte_len,
    };

    println!("{file_property}");

    Ok(())
}
