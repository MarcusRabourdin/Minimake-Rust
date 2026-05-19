use std::io;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead};
use std::collections::LinkedList;

enum Type {
    Variable(String),
    Rule(String),
    Command(String),
}

pub struct Node {
    LinkedList<String> : node,
}

pub struct Lexer {
    
}

fn create_lexer() {
    let lexer = Lexer {
    
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
   where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    
}



pub fn lex(file: &str) {
    let lexer = Lexer {};


    if let Ok(lines) = read_lines(file) {
        for line in lines.map_while(Result::ok)
        {

        }
    }
}

