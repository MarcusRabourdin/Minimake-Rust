use std::io;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
   where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    
}



pub fn lex(file: &str) {

    if let Ok(lines) = read_lines(file) {
        for line in lines.map_while(Result::ok)
        {

        }
    }
}

