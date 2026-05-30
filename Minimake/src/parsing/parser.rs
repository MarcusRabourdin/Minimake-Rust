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


pub fn parse(lexer : &Lexer) -> Result<Parser,i32> {
    let mut parser
    = parser_create();
    let list = &lexer.token_list;
    match list {
        None => panic!("there is no token_list"),
        Some(l) => {
             for element in l {
                match element {
                    Token::Variable(var) => { 
                        if parser.add_variable(var) != Ok(0) {
                            return Err(2);
                        }
                    },

                    Token::Rule(target,dep ) => {
                        if parser.add_rule(target,dep) != Ok(0) {
                            return Err(2);
                        }
                    },

                    Token::Recipe(recipe) => {
                       if parser.add_recipe(recipe) != Ok(0) {
                            return Err(2);
                        }
                    },

                    Token::Other(_) => {},
                }
            }
        }
        
    }
    Ok(parser)    
}

impl Parser {
    pub fn add_variable(&mut self, var: &str) -> Result<i32,&str>
    {
        self.in_rule = false;

        let equal_indice = var.find('=').unwrap_or(0xc0ffe);
        if equal_indice == 0xc0ffe { return Err("there is no = in this rule");};

        let name = &var[..equal_indice].trim();
        if var.to_string().len() <= equal_indice+1 { return Err("there is no content") };

        let content = &var[equal_indice+1..].trim_start();

        let variable = Variable { name : name.to_string(), content : content.to_string(), };
        self.variables.push(variable); 

        Ok(0)
    }

    pub fn add_rule(&mut self, target : &str, dep : &str) -> Result <i32, &str>
    {
        self.in_rule = true;
        
        Ok(0)
    }

    pub fn add_recipe(&mut self, recipe : &str) -> Result<i32,&str> {
        
        if !self.in_rule {
            return Err("trying to add a recipe outside a rule");
        }

        Ok(0)

    }
}
