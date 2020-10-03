pub struct Character {
    pub character: String,
    pub readings: Vec<String>
}

impl Character {
    pub fn new(character: &str, readings: Vec<String>) -> Character {
        Character {
            character: String::from(character),
            readings: readings  
        }
    }
}

pub fn consonants() -> Vec<Character> {
    let consonants = vec![
        Character::new("ㄱ",vec![String::from("g")]),
        Character::new("ㄴ",vec![String::from("n")]),
        Character::new("ㄷ",vec![String::from("d")]),
        Character::new("ㄹ",vec![String::from("r"),String::from("l")]),
        Character::new("ㅁ",vec![String::from("m")]),
        Character::new("ㅂ",vec![String::from("b")]),
        Character::new("ㅅ",vec![String::from("s")]),
        Character::new("ㅇ",vec![String::from("ng")]),
        Character::new("ㅈ",vec![String::from("j")]),
        Character::new("ㅊ",vec![String::from("ch")]),
        Character::new("ㅋ",vec![String::from("k")]),
        Character::new("ㅌ",vec![String::from("t")]),
        Character::new("ㅍ",vec![String::from("p")]),
        Character::new("ㅎ",vec![String::from("h")]),
        Character::new("ㄲ",vec![String::from("kk")]),
        Character::new("ㄸ",vec![String::from("tt")]),
        Character::new("ㅃ",vec![String::from("pp")]),
        Character::new("ㅆ",vec![String::from("ss")]),
        Character::new("ㅉ",vec![String::from("jj")]),
    ];
    consonants
}

pub fn vowels() -> Vec<Character> {
    let vowels = vec![
        Character::new("ㅏ",vec![String::from("a")]),
        Character::new("ㅑ",vec![String::from("ya")]),
        Character::new("ㅓ",vec![String::from("eo")]),
        Character::new("ㅕ",vec![String::from("yeo")]),
        Character::new("ㅗ",vec![String::from("o")]),
        Character::new("ㅛ",vec![String::from("yo")]),
        Character::new("ㅜ",vec![String::from("u")]),
        Character::new("ㅠ",vec![String::from("yu")]),
        Character::new("ㅡ",vec![String::from("eu")]),
        Character::new("ㅣ",vec![String::from("i")]),
        Character::new("ㅐ",vec![String::from("ae")]),
        Character::new("ㅒ",vec![String::from("yae")]),
        Character::new("ㅔ",vec![String::from("e")]),
        Character::new("ㅖ",vec![String::from("ye")]),
        Character::new("ㅘ",vec![String::from("wa")]),
        Character::new("ㅙ",vec![String::from("wae")]),
        Character::new("ㅚ",vec![String::from("oe")]),
        Character::new("ㅝ",vec![String::from("weo")]),
        Character::new("ㅞ",vec![String::from("we")]),
        Character::new("ㅟ",vec![String::from("wi")]),
        Character::new("ㅢ",vec![String::from("ui")])
    ];
    vowels
}