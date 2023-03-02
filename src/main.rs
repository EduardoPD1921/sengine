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

#[derive(Debug, Clone, Copy)]
struct FoundWord {
    index: usize,
    start: usize,
    end: usize
}

const START_ANSI_CODE_SIZE: usize = 5;
const END_ANSI_CODE_SIZE: usize = 4;

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
    
    let vec_of_found_words = search_through_vec(&args.search_term, &vec_of_words);

    let mut highlighted_text: String = String::new();
    vec_of_words.iter().enumerate().for_each(|(index, word)| {
        let mut found_word_chunks: Vec<FoundWord> = Vec::new();
        for found_chunk in vec_of_found_words.iter() {
            if index == found_chunk.index {
                found_word_chunks.push(*found_chunk);
            }
        }

        let mut word_to_add = word.to_owned();
        for (index, found_word_chunk) in found_word_chunks.iter().enumerate() {
            word_to_add.insert_str(found_word_chunk.start + ((START_ANSI_CODE_SIZE * index) + (END_ANSI_CODE_SIZE * index)), "\x1b[47m");
            word_to_add.insert_str(found_word_chunk.end + ((START_ANSI_CODE_SIZE * index) + (END_ANSI_CODE_SIZE * index)) + START_ANSI_CODE_SIZE, "\x1b[0m");
        }
        
        highlighted_text.push_str(&format!("{word_to_add} "));
    });

    println!("{highlighted_text}");
    println!("{} matches for '{}'.", vec_of_found_words.len(), args.search_term);
}

fn search_through_vec(search_term: &String, vec_of_words: &Vec<String>) -> Vec<FoundWord> {
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
