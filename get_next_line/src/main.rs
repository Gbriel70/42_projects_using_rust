use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::env;

struct GNL {
    reader: BufReader<File>,
}

impl GNL {
    fn new(filename: &str) -> Result<Self, io::Error> {
        let file = File::open(filename) ?;
        Ok (Self {
            reader: BufReader::new(file),
        })
    }

    fn get_next_line(&mut self) -> Option<String> {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(0) => None,
            Ok(_) => {
                if line.ends_with('\n') {
                    line.pop();
                }
                Some(line)
            }
            Err(_) => None,
        }
    }
    
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return;
    }

    let filename = &args[1];

    let mut gnl = match GNL::new(filename) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    while let Some(line) = gnl.get_next_line() {
        println!("{}", line);
    }
}
