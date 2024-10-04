# Hangman - A Rust Command Line Game

This is a simple command-line implementation of the classic Hangman game written in Rust. In this game, one player enters a word, and the other player attempts to guess the word by inputting individual letters. The game runs entirely in the terminal.

## How to Play

1. The game expects the word to be guessed as a command-line argument.
2. In the terminal, enter

cargo run INPUT_TEXT

where INPUT_TEXT will be whichever word you would like  as the prompt. After entering the word, the game will prompt you to guess letters.
3. You will receive feedback on whether or not you guessed the correct letter.
5. You win by guessing all the letters in the word before you run out of attempts.
