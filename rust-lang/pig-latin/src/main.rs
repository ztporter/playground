use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    print!("enter phrase to convert: ");
    io::stdout().flush().expect("unable to flush stdout");

    io::stdin().read_line(&mut input).expect("error reading input");

    let pig_latin = pig_latin_from_english(&input);

    println!("pig latin: {}", pig_latin);
}

fn pig_latin_from_english (english: &String) -> String {
    let mut pig_latin = String::new();
    for word in english.split_whitespace() {
        let pig_latin_word = get_pig_latin_word(&word);
        pig_latin.push_str(&pig_latin_word);
        pig_latin.push_str(" ");
    }
    pig_latin
}

fn get_pig_latin_word (word: &str) -> String {
    match &word[0..1] {
        "a"|"e"|"i"|"o"|"u" => format!("{}-hay", word),
        _ => format!("{}-{}ay", &word[1..], &word[0..1])
    }
}
