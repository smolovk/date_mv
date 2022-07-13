use chrono::{DateTime, Local};
use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let buff = fs::read(&args.path).unwrap();
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
                //TODO: Format this string
                value.to_string()
            } else {
                println!("No date found");
                String::from("")
            }
        }
        Err(rexif::ExifError::JpegWithoutExif(_)) => {
            println!("Image has no exif information, trying to take from file metadata...");
            let metadata = fs::metadata(&args.path).unwrap();
            let datetime: DateTime<Local> = metadata.created().unwrap().into();
            datetime.format("%Y_%m_%d-%H_%M_%S").to_string()
        }
        Err(rexif::ExifError::FileTypeUnknown) => {
            println!("Not an image, trying to take from file metadata...");
            let metadata = fs::metadata(&args.path).unwrap();
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
    let file_path = String::from(args.path.parent().unwrap().to_str().unwrap());
    let extension = args.path.extension().unwrap().to_str().unwrap();
    let new_filename = file_path + "/" + &creation_date + "." + extension;
    println!("{}", &new_filename);
    fs::rename(&args.path, new_filename).unwrap()
}
