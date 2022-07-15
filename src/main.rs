use chrono::{DateTime, Local};
use clap::Parser;
use std::fs;

/// Program to rename file to its creation date (uses exif if possible)
#[derive(Parser)]
struct Cli {
    /// Path to file/directory to rename
    path: std::path::PathBuf,

    /// Use if need to rename all files within directory
    #[clap(short = 'd', long = "directory")]
    is_directory: bool,
}

fn main() {
    let args = Cli::parse();

    if !args.is_directory {
        return rename_file(&args.path, None);
    }

    let files = get_files_from_dir(&args.path);
    for (index, file) in files.iter().enumerate() {
        rename_file(&file, Some(&index))
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

fn rename_file(path: &std::path::PathBuf, index: Option<&usize>) {
    let buff = fs::read(&path);
    let buff = match buff {
        Ok(buff) => buff,
        Err(error) => {
            if error.kind() == std::io::ErrorKind::NotFound {
                return println!("No such file or directory");
            }
            return println!("{}", error);
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
        Err(error) => return println!("Unexpected error: {}", error),
    };

    if creation_date == *"" {
        println!("No date found");
        return;
    }

    //rename the file
    let file_path = String::from(path.parent().unwrap().to_str().unwrap());
    let extension = path.extension().unwrap().to_str().unwrap();
    let mut new_filename = file_path.clone() + "/" + &creation_date + "." + extension;
    if std::path::Path::new(&new_filename).exists() {
        new_filename = file_path + "/" + &creation_date + "(" + &index.unwrap_or(&0usize).to_string() + ")." + extension;
    }
    println!("{}", &new_filename);
    fs::rename(&path, new_filename).unwrap()
}

#[test]
fn renames_file() {
    let testing_dir = std::path::Path::new("./tmp/renames_file");
    if testing_dir.exists() {
        fs::remove_dir_all(&testing_dir).unwrap();
    }

    // create temporary directory to test renaming in
    fs::create_dir_all(&testing_dir).unwrap();

    let file_path = std::path::PathBuf::from(&testing_dir).join("test_image.jpg");

    fs::copy("./test_image.jpg", &file_path).unwrap();

    rename_file(&file_path, None);

    assert!(testing_dir.to_path_buf().join("2008_07_31-10_05_49.jpg").exists());

    // delete the temporary directory
    fs::remove_dir_all(testing_dir).unwrap();
}

#[test]
fn renames_dir() {
    let testing_dir = std::path::Path::new("./tmp/renames_dir");
    if testing_dir.exists() {
        fs::remove_dir_all(&testing_dir).unwrap();
    }

    fs::create_dir_all(&testing_dir).unwrap();

    let file_path = std::path::PathBuf::from(&testing_dir).join("test_image.jpg");
    let second_file_path = std::path::PathBuf::from(&testing_dir).join("second_test_image.jpg");

    fs::copy("./test_image.jpg", &file_path).unwrap();
    fs::copy("./test_image.jpg", &second_file_path).unwrap();

    let files = get_files_from_dir(&std::path::PathBuf::from(&testing_dir));
    for (index, file) in files.iter().enumerate() {
        rename_file(&file, Some(&index))
    }

    assert!(testing_dir.to_path_buf().join("2008_07_31-10_05_49.jpg").exists());
    assert!(testing_dir.to_path_buf().join("2008_07_31-10_05_49(1).jpg").exists());

    fs::remove_dir_all(testing_dir).unwrap();
}