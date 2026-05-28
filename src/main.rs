mod commands;
extern crate walkdir;
use clap::{Parser, Subcommand};
use commands::copy_file::cpy_file;
use commands::delete_file::del_file;
use commands::move_file::mv_file;
use commands::properties::file_prop;
use commands::rename_file::rn_file;
use commands::create_directory::create_dir;
use commands::create_file::create_file;
use std::{
    io,
    path::PathBuf,
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[clap(about = "Rename a file")]
    Rename {
        #[clap(value_parser)]
        old_name: PathBuf,
        #[clap(value_parser)]
        new_name: PathBuf,
    },
    #[clap(about = "Create a new file")]
    Create {
        #[clap(value_parser)]
        file_name: PathBuf,
        #[clap(short, long, value_parser)]
        directory: Option<PathBuf>,
    },
    #[clap(about = "Delete a file or directory")]
    Delete {
        #[clap(value_parser)]
        file_path: PathBuf,
        #[clap(short, long, value_parser)]
        directory: Option<PathBuf>,
    },
    #[clap(about = "Move a file")]
    Move {
        #[clap(short, long, value_parser)]
        source_directory: PathBuf,
        #[clap(short, long, value_parser)]
        destination_directory: PathBuf,
    },
    #[clap(about = "Copy a file")]
    Copy {
        #[clap(value_parser)]
        file_name: Option<PathBuf>,
        #[clap(short, long, value_parser)]
        source_path: PathBuf,
        #[clap(short, long, value_parser)]
        destination_path: PathBuf,
    },
    #[clap(about = "Get file properties")]
    Properties {
        #[clap(value_parser)]
        file_name: PathBuf,
    },
    #[clap(about = "Create a new directory")]
    CreateDirectory {
        #[clap(value_parser)]
        directory_name: PathBuf,
        #[clap(short, long, value_parser)]
        parent_directory: Option<PathBuf>,
    },
    #[clap(about = "Rename a directory")]
    RenameDirectory {
        #[clap(value_parser)]
        old_dir: PathBuf,
        #[clap(value_parser)]
        new_dir: PathBuf,
    },
    #[clap(about = "Delete a directory")]
    DeleteDirectory {
        #[clap(value_parser)]
        directory_name: PathBuf,
    },
    #[clap(about = "List files in a directory")]
    ListFiles {
        #[clap(short, long, value_parser)]
        directory: Option<PathBuf>,
    },
    #[clap(about = "List sub-directories of a directory")]
    ListDirectories {
        #[clap(short, long, value_parser)]
        directory: Option<PathBuf>,
    },
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    
    let result = match cli.command {
        Command::Rename { old_name, new_name } => {
            rn_file(old_name, new_name).map(|_| println!("File renamed successfully."))
        }
        Command::Create { file_name, directory } => {
            create_file(file_name, directory).map(|_| println!("File created successfully."))
        }
        Command::Delete { file_path, directory } => {
            del_file(file_path, directory).map(|_| println!("Deleted successfully."))
        }
        Command::Move { source_directory, destination_directory } => {
            mv_file(source_directory, destination_directory).map(|_| println!("Moved successfully."))
        }
        Command::Copy { source_path, destination_path, file_name } => {
            cpy_file(source_path, destination_path, &file_name).map(|_| println!("Copied successfully."))
        }
        Command::Properties { file_name } => {
            file_prop(file_name)
        }
        Command::CreateDirectory { directory_name, parent_directory } => {
            create_dir(directory_name, parent_directory).map(|_| println!("Directory created successfully."))
        }
        Command::RenameDirectory { old_dir, new_dir } => {
            rn_file(old_dir, new_dir).map(|_| println!("Directory renamed successfully."))
        }
        Command::DeleteDirectory { directory_name } => {
            del_file(directory_name, None).map(|_| println!("Directory deleted successfully."))
        }
        _ => {
            println!("Command not yet implemented.");
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}
