use clap::{App, Arg, AppSettings};

extern crate hangul;

fn main() {
    let matches = App::new("Hangul")
        .version("1.0")
        .about("Practice Hangul with a little quiz")
        .setting(AppSettings::ArgRequiredElseHelp)
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
        )
        .get_matches();

    match matches.subcommand() {
        ("quiz", Some(sub_m)) => {
            let options = hangul::Options {
                mode: hangul::Mode::Quiz,
                all: sub_m.is_present("consonants") 
                    && sub_m.is_present("vowels") 
                    && sub_m.is_present("composites"),
                consonants: sub_m.is_present("consonants"),
                vowels: sub_m.is_present("vowels"),
            };
            hangul::handle_options(&options);
        },
        ("show", Some(_)) => {
            let options = hangul::Options {
                mode: hangul::Mode::Show,
                all: false,
                consonants: false,
                vowels: false,
            };
            hangul::handle_options(&options);
        },
        _ => {

        }
    }
}