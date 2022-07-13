use chrono::{DateTime, Local};
use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    rename_file(&args.path);
}

fn rename_file(path: &std::path::PathBuf) {
    let buff = fs::read(&path).unwrap();
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
            println!("Image has no exif information, trying to take from file metadata...");
            let metadata = fs::metadata(&path).unwrap();
            let datetime: DateTime<Local> = metadata.created().unwrap().into();
            datetime.format("%Y_%m_%d-%H_%M_%S").to_string()
        }
        Err(rexif::ExifError::FileTypeUnknown) => {
            println!("Not an image, trying to take from file metadata...");
            let metadata = fs::metadata(&path).unwrap();
            let datetime: DateTime<Local> = metadata.created().unwrap().into();
            datetime.format("%Y_%m_%d-%H_%M_%S").to_string()
        }
        Err(error) => return println!("Unexpected error: {}", error),
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
