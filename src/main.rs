use std::fs::{self, DirEntry};
use std::path::{self, Path, PathBuf};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct HexArgs {
    #[arg(short, long)]
    target_directory: String,

    #[arg(short, long)]
    output_directory: String,
}

pub fn main() {
    let args = HexArgs::parse();

    let target_directory = path::absolute(&args.target_directory)
        .expect(&format!("🚫 | [ ERROR ] target_directory: \"{:?}\" not found", args.target_directory));

    let output_directory = path::absolute(&args.output_directory)
        .expect(&format!("🚫 | [ ERROR ] output_directory: \"{:?}\" not found", args.output_directory));

    println!("🚀 | [ INFO ] getting files from \"{:?}\". NOW CONVERTING...", args.target_directory);

    let dir_files = get_files_in_directory(&vec![], &target_directory);

    println!("🚀 | [ INFO ] converting files to hex...");

    convert_to_hex_from_names(target_directory, output_directory, dir_files);
    
    println!("🚀 | [ INFO ] converting files to decimal...");
}

fn convert_to_hex_from_names(target: PathBuf, output: PathBuf, file_entries: Vec<DirEntry>) {
    for entry in file_entries {
        let file_name = entry
            .file_name()
            .to_str()
            .unwrap()
            .to_string();

        if let Err(message) = fs::copy(
            format!(
                "{}/{}", 
                target.display(), 
                file_name
            ),
            format!(
                "{}/{}", 
                output.display(), 
                convert_to_hex_from_decimal(&file_name)
            ),
        ) {
            eprintln!("🚫 | [ ERROR ] {}", message);
        } else {
            println!("✅ | [ SUCCESS ] converted \"{}\" to \"{}\"", &file_name, convert_to_hex_from_decimal(&file_name));
        };
    }
}

fn get_files_in_directory(_files: &Vec<DirEntry>, dir: &Path) -> Vec<DirEntry> {
    let mut files: Vec<DirEntry> = Vec::new();

    // IMPLEMENTATION NOTE: DIG FOR SUBDIRECTORIES...

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    dbg!("⚠️ | [ WARN ] found directory: {:?}", entry.file_name());

                    get_files_in_directory(&files, &entry.path());
                } else {
                    files.push(entry);
                }
            }

        }
    }

    files
}


fn convert_to_hex_from_decimal(file: &str) -> String {
    let file_name = file
        .split(".")
        .collect::<Vec<&str>>()[0];

    let mut hex_string = String::new();

    let string_decimal = file_name.parse::<u32>().unwrap();

    hex_string.push_str(&format!("{:X}", string_decimal));

    hex_string
}

