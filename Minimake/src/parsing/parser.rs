use crate::lexing::lexer::Lexer;
use crate::lexing::lexer::Token;
struct Rule {
    target : String,
    dep : Vec<String>,
    recip : Vec<String>,
}

struct Variable {
    name : String,
    content : String,
}

pub struct Parser {
    rules : Vec<Rule>,
    variables : Vec<Variable>,

}

fn parser_create() -> Parser {
    Parser {
        rules : Vec::new(),
        variables : Vec::new(),
    }
}


pub fn parse(lexer : &Lexer) -> Parser {
    let mut parser = parser_create();
    lexer.token_list.iter().for_each(|token| {
    

    }); 

    parser
    
}
