use staticanalyzer::rs2dot;
use std::{env, fs};
use std::path::{Path, PathBuf};

fn main() -> () {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please set a rust file path");
    }
    let path = Path::new(&args[1]);
    let results = rs2dot(path);
    let mut target_name = PathBuf::new();

    match args.len() {
        2 => {
            let target_dir = path.parent().unwrap();
            target_name.push(target_dir.join(path.file_stem().unwrap()));
            target_name.set_extension(".dot");
        },
        4 => {
            if &args[2] != "-o" {
                panic!("current options has [-o], please enter the right option");
            }
            let target_dir = Path::new(&args[3]);
            target_name.push(target_dir.join(path.file_stem().unwrap()));
            target_name.set_extension(".dot");
        }
        _ => {
            panic!("Please set a rust file path");
        }
    }

    
    match fs::write(&target_name, results) {
        Ok(_) => (),
        Err(e) => {
            println!("target_name is {:?}", target_name);
            println!("{:?}", e);
        }
    }
}