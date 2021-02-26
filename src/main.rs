use std::io;

fn main() {
    println!("Welcome to pig latin translator");
    println!("Please enter the work you want to translate: ");
    let mut word = String::new();
    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read the input ");

    let new_word = pig_latin(word.clone());
    println!(
        "Your word {} translated into pig latin as {}", word, new_word
    );
}

fn pig_latin(word: String) -> String {
    if check_vowel(word.clone()) {
        return vowel(word.clone());
    } else {
        return non_vowel(word.clone());
    }
}

fn check_vowel(word: String) -> bool {
    let vowels = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    let first_letter = word.chars().nth(0).expect("A none value occured");

    for item in vowels.iter() {
        if first_letter == *item {
            return true;
        }
    }
    return false;
}

fn non_vowel(word: String) -> String {
    let first_letter = word.chars().nth(0).expect("None value occured");

    let mut new_word = String::new();
    for (i, chr) in word.chars().enumerate() {
        if i != 0 {
            new_word.push(chr);
        }
    }
    new_word.push('-');
    new_word.push(first_letter);
    new_word.push_str("ay");

    return new_word;
}

fn vowel(word: String) -> String {
    let mut new_word = String::new();

    for chr in word.chars() {
        new_word.push(chr);
    }
    new_word.push_str("-hay");
    return new_word;
}
