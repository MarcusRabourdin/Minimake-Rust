use std::{env, fmt::Result};

use Minimake::file_gestion::file_verification::is_file;
use Minimake::lexing::lexer::*;
use Minimake::parsing::parser::parse;

fn main() -> Result<> { 
     
    let _args: Vec<String> = env::args().collect(); 
    let argc = _args.len();
    if argc < 2 {
        println!("No argument");
        return Err(std::fmt::Error) ;
    }

    let file = _args[1].clone();
    let exist = is_file(&file);

    if !exist
    {
        println!("{:?} is not a file",file); 
        return Err(std::fmt::Error);
    }
    
    let makefile_data = lex(&file);
    match makefile_data {
        Ok(data) => {    
            //data.print();

            let parsed_data = parse(&data);
            match parsed_data {
                Ok(data) => {
                    data.print();    
                },
                Err(_) => return Err(std::fmt::Error),
            }

            },
        Err(_) => return Err(std::fmt::Error),
    }


    Ok(())
}
