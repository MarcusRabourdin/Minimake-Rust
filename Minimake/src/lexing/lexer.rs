use std::io;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead};

enum Token {
    Variable(String),
    Rule(String),
    Command(String),
    Other(String),
}

struct Lexer {
    token_list : Option<Vec<Token>>,
    nb_token : u32
}

impl Lexer {
    pub fn create() -> Lexer {
        Lexer {
            token_list: None,
            nb_token : 0}
    }

    fn increment_nb_token(mut self)
    {
        self.nb_token +=1
    }

    fn update_nb_token(mut self)
    {   
        self.nb_token = self.token_list.iter().count() as u32
    }

    pub fn add_token(mut self, token : Token)
    {
        match self.token_list {
            Some(ref mut l) => {
                l.push(token);
                self.increment_nb_token();
            }
            None => panic!("Cannot add token bc token_list is None"),
        }
    }

    pub fn get_first(self) -> Token {
        match self.token_list {
            Some(l) => l.first().unwrap(),
            None => panic!("No token left"),
        }
    }

}

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

