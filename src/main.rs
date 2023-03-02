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
    env,
    mem::drop
};

struct Args {
    file_path: String,
    search_term: String
}

#[derive(Debug)]
struct FoundWord {
    index: usize,
    start: usize,
    end: usize
}

impl Args {
    fn new(file_path: String, search_term: String) -> Args {
        Args { file_path, search_term }
    }
}

impl FoundWord {
    fn new(index: usize, start: usize, end: usize) -> FoundWord {
        FoundWord { index, start, end }
    }
}

fn main() {
    let args = get_args().map_err(|e| {
        eprintln!("{e}");
        exit(1);
    }).unwrap();

    let file_content = match read_file_contents(args.file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{e}");
            exit(1);
        }
    };

    let str_file_buffer: Vec<&str> = file_content.split(' ').collect();
    let vec_of_words: Vec<String> = str_file_buffer.iter().map(|word| word.to_string()).collect();
    drop(str_file_buffer);
    
    let vec_of_found_words = search_through_vec(args.search_term, &vec_of_words);

    let mut text_highlighted = String::new();

    vec_of_words.iter().enumerate().for_each(|(index, word)| {
        for found_word in &vec_of_found_words {
            if index == found_word.index {
                let text_buffer = word.to_owned();
                let (start_wrap, _) = text_buffer.split_at(found_word.start);
                let (_, end_wrap) = text_buffer.split_at(found_word.end);
                let highlighted_word = &text_buffer[found_word.start..found_word.end];

                return text_highlighted.push_str(&format!("{start_wrap}\x1b[47m{highlighted_word}\x1b[0m{end_wrap} "));
            }
        }

        text_highlighted.push_str(&format!("{word} "));
    });

    println!("{text_highlighted}");
}

fn search_through_vec(search_term: String, vec_of_words: &Vec<String>) -> Vec<FoundWord> {
    let mut vec_of_found_words: Vec<FoundWord> = Vec::new();

    for (index, word) in vec_of_words.iter().enumerate() {
        if word.len() >= search_term.len() {
            let times_to_loop = word.len() - search_term.len() + 1;
            let mut end = search_term.len();

            for start in 0..times_to_loop {
                let slice = &word[start..end];

                if slice.to_lowercase() == search_term.to_lowercase() {
                    let found_word = FoundWord::new(index, start, end);
                    vec_of_found_words.push(found_word);
                }
                end += 1;
            }
        }
    }
    
    vec_of_found_words
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

fn read_file_contents(file_path: String) -> Result<String, FileError> {
    let mut file = File::open(file_path).map_err(|e| FileError::FileOpenError(e))?;
    let mut contents = String::new();
    
    file.read_to_string(&mut contents).map_err(|e| FileError::FileOpenError(e))?;

    Ok(contents)
}
