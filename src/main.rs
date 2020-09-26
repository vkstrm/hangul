use clap::{App, Arg};

extern crate hangul;

fn main() {
    let matches = App::new("hangul")
        .version("1.0")
        .about("Help with hangul")
        .subcommand(App::new("quiz")
            .about("Practice the characters")
            .arg(
                Arg::with_name("consonants")
                    .short('c')
                    .about("Practice the consonants")
                    .takes_value(false)
            )
            .arg(
                Arg::with_name("vowel")
                    .short('v')
                    .about("Practice the vowels")
                    .takes_value(false)
            )
        )
        .subcommand(App::new("show")
            .about("Show the character charts")
            .arg(
                Arg::with_name("consonants")
                    .short('c')
                    .takes_value(false)
            )
            .arg(
                Arg::with_name("vowel")
                    .short('v')
                    .about("Practice the vowels")
                    .takes_value(false)
            )
        )
        .get_matches();

    match matches.subcommand() {
        ("show", Some(sub_m)) => {
            let options = hangul::Options {
                mode: hangul::Mode::Show,
                all: sub_m.is_present("consonants") 
                    && sub_m.is_present("vowels") 
                    && sub_m.is_present("composites"),
                consonants: sub_m.is_present("consonants"),
                vowels: sub_m.is_present("vowels"),
            };
            hangul::start_quiz(&options);
        },
        ("quiz", Some(sub_m)) => {
            let options = hangul::Options {
                mode: hangul::Mode::Show,
                all: sub_m.is_present("consonants") 
                    && sub_m.is_present("vowels") 
                    && sub_m.is_present("composites"),
                consonants: sub_m.is_present("consonants"),
                vowels: sub_m.is_present("vowels"),
            };
            hangul::start_quiz(&options);
        },
        _ => {
            println!("use --help")
        }
    }
}