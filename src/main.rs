use clap::Parser;
use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

#[derive(Parser)]

struct Args {
    word: String,
}

fn main() {
    let args = Args::parse();
    let _secret_word = args.word.to_lowercase();

    // initializing start state
    let mut guessed_word = vec!['_'; _secret_word.len()];
    let mut attempts = 6;
    let mut wrong_guesses: Vec<char> = vec![];

    // allows us to capture single key process
    enable_raw_mode().expect("Failed to enable raw mode");

    while attempts > 0 && guessed_word.contains(&'_'){
        // displays current state
        println!("Current word: {}", guessed_word.iter().collect::<String>());
        println!("Wrong guess: {:?}", wrong_guesses);
        println!("Attempts left: {}", attempts);
        print!("Enter your guess: ");

        if let Event::Key(key_event) = read().expect("Failed to read key") {
            match key_event.code {
                KeyCode::Char(guess_char) => {
                    let guess_char = guess_char.to_ascii_lowercase();

                    // check if the character is in  the secret word
                    if _secret_word.contains(guess_char) {
                        for (i, ch) in _secret_word.chars().enumerate(){
                            if ch == guess_char {
                                guessed_word[i] = guess_char;
                            }
                        }
                    } else {
                        // if it is then iterate through and add it in
                        // if not then add it to wrong guess and attempts - 1
                        // only decrement the attempts if that guess has not been made before
                        if !wrong_guesses.contains(&guess_char) {
                            wrong_guesses.push(guess_char);
                            attempts = attempts - 1;
                            println!("\nWrong guess! Attempts left: {}", attempts);
                        }
                        else {
                            println!("\nYou've already guessed that letter.");
                        }
                    }
                }
                KeyCode::Esc => {
                    println!("\nGame aborted.");
                    break;
                }
                _ => {
                    println!("\nPlease enter a valid letter.");
                }
            }
        }
    }

    disable_raw_mode().expect("Failed to disable raw mode");

    // check if there are anymore underscores still in the guessed_word
    if guessed_word.contains(&'_') {
        // if there are then they lost
        println!("You lost! The word was: {}", _secret_word);
    } else {
        // if not then they won 
        println!("You guessed it! The word is: {}", _secret_word);
    }
}
