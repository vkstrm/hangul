use std::collections::HashMap;

pub fn consonants() -> HashMap<&'static str, Vec<&'static str>> {
    let consonants: HashMap<&'static str, Vec<&'static str>> = 
    [
        ("ㄱ",vec!["g"]),
        ("ㄴ",vec!["n"]),
        ("ㄷ",vec!["d"]),
        ("ㄹ",vec!["r","l"]),
        ("ㅁ",vec!["m"]),
        ("ㅂ",vec!["b"]),
        ("ㅅ",vec!["s"]),
        ("ㅇ",vec!["ng"]),
        ("ㅈ",vec!["j"]),
        ("ㅊ",vec!["ch"]),
        ("ㅋ",vec!["k"]),
        ("ㅌ",vec!["t"]),
        ("ㅍ",vec!["p"]),
        ("ㅎ",vec!["h"]),
        ("ㄲ",vec!["kk"]),
        ("ㄸ",vec!["tt"]),
        ("ㅃ",vec!["pp"]),
        ("ㅆ",vec!["ss"]),
        ("ㅉ",vec!["jj"]),
    ].iter().cloned().collect();
    consonants
}

pub fn vowels() -> HashMap<&'static str, Vec<&'static str>> {
    let vowels: HashMap<&'static str, Vec<&'static str>> = 
    [
        ("ㅏ",vec!["a"]),
        ("ㅑ",vec!["ya"]),
        ("ㅓ",vec!["eo"]),
        ("ㅕ",vec!["yeo"]),
        ("ㅗ",vec!["o"]),
        ("ㅛ",vec!["yo"]),
        ("ㅜ",vec!["u"]),
        ("ㅠ",vec!["yu"]),
        ("ㅡ",vec!["eu"]),
        ("ㅣ",vec!["i"]),
        ("ㅐ",vec!["ae"]),
        ("ㅒ",vec!["yae"]),
        ("ㅔ",vec!["e"]),
        ("ㅖ",vec!["ye"]),
        ("ㅘ",vec!["wa"]),
        ("ㅙ",vec!["wae"]),
        ("ㅚ",vec!["oe"]),
        ("ㅝ",vec!["weo"]),
        ("ㅞ",vec!["we"]),
        ("ㅟ",vec!["wi"]),
        ("ㅢ",vec!["ui"])
    ].iter().cloned().collect();
    vowels
}