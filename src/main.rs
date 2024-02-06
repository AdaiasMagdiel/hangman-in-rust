use std::io;
use std::io::Write;

fn main() {
    let word = "hangman";
    let enc_word = word.chars().map(|_| '_').collect::<Vec<char>>();
    let mut guess = String::new();

    println!("Welcome to the Hangman game in Rust.");
    println!("The mysterious word contains {} letter.", word.len());
    println!("Word: {}\n", enc_word.iter().collect::<String>());

    print!("Type one letter: ");
    io::stdout().flush().expect("Error when flush your stdout.");
    io::stdin().read_line(&mut guess)
        .expect("Error when read user input.");

    println!("You guess: {}", guess);
}
