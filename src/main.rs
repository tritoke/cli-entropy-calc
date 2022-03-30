use std::fs;
use entropy::shannon_entropy as shannon;
use size_display::Size;
use std::env;

fn main() {
    let mut args = env::args().peekable();
    let program_name = args.next().unwrap();
    if args.peek().is_some() {
        println!("Usage: {program_name} <file1> <file2>...");
    }

    for file in args {
        match fs::read(&file) {
            Ok(bytes) => {
                let ent = shannon(&bytes);
                println!("{ent:.2} - {file} - {:.2}", Size(bytes.len() as u64));
            },
            Err(e) => {
                eprintln!("[*] Failed to read {file} - {e:?}")
            }
        }
    }
}
