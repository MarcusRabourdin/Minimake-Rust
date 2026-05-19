use std::{fs, path::PathBuf};

fn get_fomated_path(path: &str) -> PathBuf {
    let formated_path =  PathBuf::from(path);
    let fullpath = fs::canonicalize(&formated_path);
    let return_path = fullpath.unwrap();
    println!("{:?}",return_path);
    return_path
}

pub fn is_file(path: &str) -> bool {
    let formated_path = get_fomated_path(path);
    formated_path.is_file()
}


