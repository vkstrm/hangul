use std::io::{self, Write};
use std::collections::HashMap;
use std::collections::HashSet;

mod characters;

type CharMap = HashMap<&'static str, Vec<&'static str>>;

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

pub fn handle_options(args: &Options) {
    match args.mode {
        Mode::Quiz => {
            let mut map = CharMap::new();
            if args.all {
                merge(&mut map, &characters::consonants());
                merge(&mut map, &characters::vowels());
            } else {
                if args.consonants {
                    merge(&mut map, &characters::consonants());
                }
                if args.vowels {
                    merge(&mut map, &characters::vowels()); 
                }
            }
            start_quiz(&map);
        },
        Mode::Show => {
            show();
        }
    }


}

fn show() {
    // TODO Make in order
    let consonants = characters::consonants();
    let vowels = characters::vowels();
    print_map("Consonants", &consonants);
    print_map("Vowels", &vowels);
}

fn print_map(kind: &str, map: &CharMap) {
    println!("--- {} ---", kind);
    let mut count = 0;
    for (key, value) in map {
        print!("{} : {}  \t", key, value.join(", "));
        count += 1;
        if count == 5 {
            print!("\n");
            io::stdout().flush().unwrap();
            count = 0;
        }
    }
    println!("\n");
}

fn start_quiz(quiz_map: &CharMap) {
    let mut correct_keys = HashSet::<&str>::new();
    loop {
        for (key, value) in quiz_map {
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

