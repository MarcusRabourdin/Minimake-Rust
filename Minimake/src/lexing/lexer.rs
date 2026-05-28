use std::{io, usize};
use std::fs::File;
use std::path::Path;
use std::io::{BufRead};

pub enum Token {
    Variable(String),
    Rule(String,String),
    Command(String),
    Other(String),
}

pub struct Lexer {
    token_list : Option<Vec<Token>>,
    nb_token : u32
}

pub fn lexer_create() -> Lexer {
    Lexer {
        token_list: Some(Vec::new()),
        nb_token : 0}
}

impl Lexer {
    fn increment_nb_token(&mut self)
    {
        self.nb_token +=1
    }

    fn update_nb_token(&mut self)
    {   
        self.nb_token = self.token_list.iter().count() as u32
    }

    pub fn add_token(&mut self, token : Token)
    {
        match self.token_list {
            Some(ref mut l) => {
                l.push(token);
                self.increment_nb_token();
            }
            None => panic!("Cannot add token bc token_list is None"),
        }
    }

    pub fn get_first(&self) -> &Token {
        match &self.token_list {
            Some(l) => l.first().unwrap(),
            None => panic!("No token left"),
        }
    }
    
    pub fn print(&self) {
        match &self.token_list {
            None => panic!("Cannot print anything bc token_list is None"),
            Some(l) => {
                
                for element in l {
                   match element {
                        Token::Rule(rule, dep) => println!("{}:{}", rule, dep),
                        Token::Command(content) => println!("{}", content),
                        Token::Variable(content) => println!("{}", content),
                        Token::Other(_) => {},
                   } 
                }
            }
            
        }
        
    }

}

fn get_token(line: &String) -> Token
{
    let index = line.find('=');
    if index.unwrap_or(usize::MAX) != usize::MAX
    {
        return Token::Variable(line.to_string()); 
    }

    let index = line.find(':');

    if index.unwrap_or(usize::MAX) != usize::MAX {

        let rule : &str = &line[0..index.unwrap()];
        let mut dep : &str = &line[index.unwrap()+1..];
        if dep == " " { dep = ""; }

        else {dep.to_string().insert(0,' ')}
        return Token::Rule(rule.to_string(),dep.to_string());
    }

    let index = line.find('\t');
    if index.unwrap_or(usize::MAX) != usize::MAX {
        return Token::Command(line.to_string());
    }

    Token::Other(line.to_string())

}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
   where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    
}


pub fn lex(file: &str) -> Lexer {
    
    let mut lexer = lexer_create();
    if let Ok(lines) = read_lines(file) {
        for line in lines.map_while(Result::ok)
        {
            let token = get_token(&line);  
            lexer.add_token(token);
        }
    }
    lexer
}

