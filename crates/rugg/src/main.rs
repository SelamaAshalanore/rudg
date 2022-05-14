use rugg::rs2dot;
use std::{env, fs};
use std::path::{Path, PathBuf};
use clap::{arg, command};

fn main() {
    let matches = command!()
        .arg(arg!([file] "Rust source code file path"))
        .arg(
            arg!(
                -o --output <DIR> "Sets a custom output directory"
            )
            .required(false)
            .allow_invalid_utf8(true),
        )
        .get_matches();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(file) = matches.value_of("file") {
        let path = Path::new(file);
        let results = rs2dot(path);
        let mut target_name = PathBuf::new();

        if let Some(raw_config) = matches.value_of_os("output") {
            // use output indicated directory path
            target_name.push(Path::new(raw_config).join(path.file_stem().unwrap()));
        } else {
            // use same directory as source file's
            target_name.push(path.parent().unwrap().join(path.file_stem().unwrap()));
        }

        target_name.set_extension(".dot");
        match fs::write(&target_name, results) {
            Ok(_) => (),
            Err(e) => {
                println!("target_name is {:?}", target_name);
                println!("{:?}", e);
            }
        }
    }
}