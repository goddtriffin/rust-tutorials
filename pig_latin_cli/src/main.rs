use std::io;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let vowels: &[String; 6] = &[
        "a".into(),
        "e".into(),
        "i".into(),
        "o".into(),
        "u".into(),
        "y".into(),
    ];

    loop {
        println!("Pig Latin CLI");
        println!("==========");
        println!("Enter a sentence:");

        // read in choice
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to readline\n");

        // collect unicode words as strings
        let words: &mut Vec<String> = &mut Vec::<String>::new();
        for unicode_word in choice.unicode_words().collect::<Vec<&str>>() {
            words.push(String::from(unicode_word));
        }

        // if no words, get more input from user
        if words.len() == 0 {
            println!("No words detected.");
            println!();
            continue;
        }

        // convert words in-place to pig latin
        pig_latin_ize(vowels, words);
        println!("Pig Latin: {}", words.join(" "));
        println!();
    }
}

fn pig_latin_ize(vowels: &[String; 6], words: &mut Vec<String>) {
    for word in words.iter_mut() {
        let first_grapheme = word
            .graphemes(true)
            .next()
            .unwrap_or_default()
            .to_lowercase();

        if vowels.contains(&first_grapheme) {
            word.push_str("-hay");
            continue;
        }

        if word.len() < 2 {
            word.push_str("ay");
            continue;
        }

        // handle consonant-first pig latin
        let graphemes = word.graphemes(true).collect::<Vec<&str>>();
        // let test = &graphemes[1..]
        *word = format!("{}-{}ay", &graphemes[1..].join(""), &graphemes[0..1].join(""));
    }
}
