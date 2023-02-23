mod error;

use error::{
    FileError,
    TermError,
    CustomError
};
use std::{
    io::prelude::*,
    fs::File,
    process::exit,
    env
};

struct Args {
    file_path: String,
    search_term: String
}

impl Args {
    fn new(file_path: String, search_term: String) -> Args {
        Args { file_path, search_term }
    }
}

fn main() {
    let args = get_args().map_err(|e| {
        eprintln!("{e}");
        exit(1);
    }).unwrap();

    let file_content = match get_file_contents(args.file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{e}");
            exit(1);
        }
    };

    println!("{file_content}");
}

fn get_args() -> Result<Args, Box<dyn CustomError>> {
    let args: Vec<String> = env::args().collect();
    
    let file_path = match args.get(1) {
        Some(f) => f,
        None => return Err(Box::new(FileError::PathNotGiven))
    };

    let search_term = match args.get(2) {
        Some(t) => t,
        None => return Err(Box::new(TermError::TermNotGiven))
    };

    Ok(Args::new(file_path.to_owned(), search_term.to_owned()))
}

fn get_file_contents(file_path: String) -> Result<String, FileError> {
    let mut file = File::open(file_path).map_err(|e| FileError::FileOpenError(e))?;
    let mut contents = String::new();
    
    file.read_to_string(&mut contents).map_err(|e| FileError::FileOpenError(e))?;

    Ok(contents)
}
