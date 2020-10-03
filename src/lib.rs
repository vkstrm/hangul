use std::io::{self, Write};
use std::collections::HashSet;

mod characters;
use characters::Character;

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
            let mut char_vec: Vec<Character> = Vec::new();
            if args.all {
                char_vec.append(&mut characters::consonants());
                char_vec.append(&mut characters::vowels());
            } else {
                if args.consonants {
                    char_vec.append(&mut characters::consonants());
                }
                if args.vowels {
                    char_vec.append(&mut characters::vowels()); 
                }
            }
            start_quiz(&char_vec);
        },
        Mode::Show => {
            show();
        }
    }


}

fn show() {
    let consonants = characters::consonants();
    let vowels = characters::vowels();
    print_chars("Consonants", &consonants);
    print_chars("Vowels", &vowels);
}

fn print_chars(kind: &str, characters: &Vec<Character>) {
    println!("--- {} ---", kind);
    let mut count = 0;
    for character in characters {
        print!("{} : {}  \t", character.character, character.readings.join(", "));
        count += 1;
        if count == 5 {
            print!("\n");
            io::stdout().flush().unwrap();
            count = 0;
        }
    }
    println!("\n");
}

fn start_quiz(char_vec: &Vec<Character>) {
    let mut correct_keys = HashSet::<&String>::new();
    loop {
        for character in char_vec {
            if correct_keys.contains(&character.character) {
                continue;
            }

            print!("{} ?: ", character.character);
            io::stdout().flush().unwrap();
            
            let answer = get_answer().unwrap();
            if answer.trim() == "quit" {
                println!("{}/{}", correct_keys.len(), char_vec.len());
                return
            }

            if answer.trim() == "score" {
                println!("{}/{}", correct_keys.len(), char_vec.len());
            }

            if character.readings.contains(&String::from(answer.trim())) {
                println!("Correct");
                correct_keys.insert(&character.character);
            } else {
                println!("Wrong!");
            }
        }

        if correct_keys.len() == char_vec.len() {
            break;
        }
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

