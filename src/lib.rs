use std::io::{self, Write};
use std::collections::HashMap;
use std::collections::HashSet;

mod characters;

type CharMap = HashMap<&'static str, Vec<&'static str>>;

#[derive(PartialEq)]
pub enum Mode {
    Show,
    Quiz
}

pub struct Options {
    pub mode: Mode,
    pub all: bool,
    pub vowels: bool,
    pub consonants: bool,
}


pub fn start_quiz(args: &Options) {
    let mut quiz_map: CharMap = CharMap::new();

    if args.all {
        merge(&mut quiz_map, &characters::consonants())
    } else {
        if args.consonants {
            merge(&mut quiz_map, &characters::consonants())
        }
        if args.vowels {
            merge(&mut quiz_map, &characters::vowels())
        }
    }

    if args.mode == Mode::Show {
        return
    }

    let mut correct_keys = HashSet::<&str>::new();
    loop {
        for (key, value) in &quiz_map {
            if correct_keys.contains(key) {
                continue;
            }

            print!("{} ?: ", key);
            io::stdout().flush().unwrap();
            
            let answer = get_answer().unwrap();
            if answer.trim() == "quit" {
                println!("{}/{}", correct_keys.len(), quiz_map.len());
                return
            }

            if answer.trim() == "score" {
                println!("{}/{}", correct_keys.len(), quiz_map.len());
            }

            if value.contains(&answer.trim()) {
                println!("Correct");
                correct_keys.insert(key);
            } else {
                println!("Wrong!");
            }
        }

        if correct_keys.len() == quiz_map.len() {
            break;
        }
    }
}

fn merge(into: &mut CharMap, from: &CharMap) {
    for (key, value) in from.into_iter() {
        into.insert(
            key, 
            Vec::from(value.clone())
        );
    }
}

fn get_answer() -> std::result::Result<String, io::Error> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            Ok(input)
        }
        Err(error) => {
            Err(error)
        }
    }   
}

