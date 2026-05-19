use std::{env, fmt::Result};

use Minimake::file_gestion::file_verification::is_file;

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
    
    //let makefile_data = lex(&file);

    Ok(())
}
