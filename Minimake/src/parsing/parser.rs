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
    in_rule : bool,

}

fn parser_create() -> Parser {
    Parser {
        rules : Vec::new(),
        variables : Vec::new(),
        in_rule : false,
    }
}


pub fn parse(lexer : &Lexer) -> Parser {
    let mut parser = parser_create();
    let list = lexer.token_list;
    match list {
        None => panic!("there is no token_list"),
        Some(l) => {
             for element in l {
                match element {
                    Token::Variable(var) => { parser.add_variable(var); },
                    Token::Rule(target,dep ) => { parser.add_rule(target,dep); },
                    Token::Recipe(recipe) => { parser.add_recipe(recipe); },
                    Token::Other(other) => {},
                }
            }
        }
        
    }
    parser    
}

