use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file: &str = match &args[..] {
        [_, file] => file,
        _ => {
            println!("Invalid arguments");
            process::exit(1);
        }
    };
    let src: String = match fs::read_to_string(file) {
        Ok(src) => src,
        Err(err) => {
            eprintln!("Failed to read {} [{}]", file, err);
            process::exit(1);
        }
    };
    println!("{}", src);
    let icerunner: icerunner::IceRunner = match src.parse() {
        Ok(icerunner) => icerunner,
        Err(_) => {
            println!("Failed to parse IceRunner");
            process::exit(1);
        }
    };
    println!("Parse done");
    println!("{}", icerunner);
    match puzzle::solve(icerunner) {
        None => println!("no solution"),
        Some((mvs, _)) => {
            println!("solution:");
            let mut last_obj = None;
            for (obj, dir) in &mvs {
                if last_obj != Some(obj) {
                    last_obj = Some(obj);
                    print!("{}", obj)
                }
                print!("{}", dir);
            }
            println!()
        }
    }
}
