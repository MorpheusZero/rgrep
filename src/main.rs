use std::env;
use rgrep::process_file;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        process_file(args[1].to_string(), args[2].to_string());
    } else {
        println!("Example: rgrep [FILE_PATH] [PATTERN_TO_MATCH]");
    }
}

