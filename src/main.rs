use std::io;
use std::io::Write;

fn reveal_enc_word(word: &str, enc_word: &mut Vec<char>, letter: char) -> bool {
    let mut is_in_string = false;

    for (idx, str_letter) in word.chars().enumerate() {
        if str_letter.to_lowercase().next().unwrap() == letter.to_lowercase().next().unwrap() {
            enc_word[idx] = str_letter;
            is_in_string = true;
        }
    }

    is_in_string
}

fn main() {
    let word = "hangman";
    let mut enc_word: Vec<char> = vec!['_'; word.len()];
    let mut guess;
    let mut letter: char;
    let mut chances = 5;

    println!("Welcome to the Hangman game in Rust.");
    println!("The mystery word contains {} letters.", word.len());
    println!("Word: {}\n", enc_word.iter().collect::<String>());

    loop {
        guess = String::new();

        print!("Type a single letter: ");
        io::stdout().flush().expect("Error flushing your stdout.");
        io::stdin().read_line(&mut guess).expect("Error reading user input.");
        letter = guess.trim().chars().next().expect("No letter provided.");

        println!("Your guess: {}", letter);
        println!("Remaining chances: {}", chances);

        let is_in_string = reveal_enc_word(&word, &mut enc_word, letter);
        if is_in_string {
            println!("Word: {}\n", enc_word.iter().collect::<String>());
        } else {
            chances -= 1;
        }

        if chances == 0 {
            println!("You lost!");
            println!("The mysterious word is {}", word);
            break;
        }

        if enc_word.iter().collect::<String>() == word {
            println!("You win! With {} rounds.", 5 - chances);
            break;
        }
    }
}
