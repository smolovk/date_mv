use chrono::{DateTime, Local};
use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,

    #[clap(short = 'd', long = "directory")]
    is_directory: bool,
}

fn main() {
    let args = Cli::parse();

    if !args.is_directory{
        return rename_file(&args.path);
    }

    let files = get_files_from_dir(&args.path);
    for file in files {
        rename_file(&file)
    }
}

fn get_files_from_dir(path: &std::path::PathBuf) -> Vec<std::path::PathBuf> {
    let mut files: Vec<std::path::PathBuf> = Vec::new();
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            files.push(path);
        }
    }
    files
}

fn rename_file(path: &std::path::PathBuf) {
    let buff = fs::read(&path);
    let buff = match buff {
        Ok(buff) => buff,
        Err(error) => {
            if error.kind() == std::io::ErrorKind::NotFound {
                return println!("No such file or directory")
            }
            return println!("{}", error)
        }
    };
    let exif = rexif::parse_buffer(&buff);

    let creation_date = match exif {
        Ok(f) => {
            let mut date: Option<rexif::TagValue> = None;
            for entry in f.entries {
                if entry.tag == rexif::ExifTag::DateTime {
                    date = Some(entry.value);
                    break;
                }
            }
            if let Some(value) = date {
                let date_string = value.to_string();
                let datetime =
                    chrono::NaiveDateTime::parse_from_str(&date_string, "%Y:%m:%d %H:%M:%S")
                        .unwrap();
                datetime.format("%Y_%m_%d-%H_%M_%S").to_string()
            } else {
                println!("No date found");
                String::from("")
            }
        }
        Err(rexif::ExifError::JpegWithoutExif(_)) => {
            let metadata = fs::metadata(&path).unwrap();
            let datetime: DateTime<Local> = metadata.created().unwrap().into();
            datetime.format("%Y_%m_%d-%H_%M_%S").to_string()
        }
        Err(rexif::ExifError::FileTypeUnknown) => {
            let metadata = fs::metadata(&path).unwrap();
            let datetime: DateTime<Local> = metadata.created().unwrap().into();
            datetime.format("%Y_%m_%d-%H_%M_%S").to_string()
        }
        Err(error) => return println!("Unexpected error: {}", error)
    };

    if creation_date == *"" {
        println!("No date found");
        return;
    }

    //rename the file
    let file_path = String::from(path.parent().unwrap().to_str().unwrap());
    let extension = path.extension().unwrap().to_str().unwrap();
    let new_filename = file_path + "/" + &creation_date + "." + extension;
    println!("{}", &new_filename);
    fs::rename(&path, new_filename).unwrap()
}
