use std::collections::HashMap;
use std::string::String;
use fancy_regex::Regex;

fn double_letters_multiplier(word: &String) -> f64 {
    let re = Regex::new(r"([a-z])\1").unwrap();
    let result = re.is_match(word);

    let mut mult : f64 = 1.0;

    let matcher = result.unwrap();
    if matcher {
        mult = 0.5;
    }
    return mult;
}

fn letter_score(letter : char) -> i32 {
    match letter {
        'E' => 0,
        'A' | 'I' | 'O' | 'U' => 10,
        'N' | 'S' | 'T' => 20,
        'Y' => 30,
        'H' | 'R' | 'D' | 'L' => 50,
        'C' | 'M' | 'F' => 80,
        'W' | 'G' | 'P' => 130,
        'B' | 'V' | 'K' => 210,
        'Q' | 'J' | 'X' | 'Z' => 340,
        _ => 0
    }
}

fn word_length_multiplier(word: &String) -> f64 {
    let multiplier = 10.0 / (word.chars().count() as f64).log2();
    multiplier
}

fn find_word_score(word : String) -> i32 {
    let mut score : i32 = 0;
    for c in word.to_uppercase().chars() {
        score += letter_score(c);
    }
    println!("{}", score);
    score = score * word_length_multiplier(&word) as i32;
    println!("{}", score);
    score = (score as f64 * double_letters_multiplier(&word)) as i32;
    println!("{}", score);
    score
}

fn main() {
    let result = std::fs::read_to_string("test_words.txt");
    let content = match result {
        Ok(content) => {content},
        Err(error) => { panic!("{}", error); }
    };

    let mut word_scores : HashMap<&str, i32> = HashMap::new();

    for line in content.lines() {
        word_scores.insert(line, find_word_score(line.to_string()));
    }

    for e in word_scores {
        println!("{} {}", e.0, e.1);
    }
}
