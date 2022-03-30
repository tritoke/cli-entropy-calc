use std::fs;
use entropy::shannon_entropy as shannon;
use size_display::Size;
use std::env;

fn main() {
    let mut args = env::args().peekable();
    let program_name = args.next().unwrap();
    if env::args().count() == 1 {
        println!("Usage: {program_name} <file1> <file2>...");
    }

    for file in std::env::args().skip(1) {
        if let Ok(bytes) = fs::read(&file) {
            let ent = shannon(&bytes);
            println!("{ent:.2} - {file} - {:.2}", Size(bytes.len() as u64));
        }
    }
}
