mod game;
mod input;
mod levels;

use crate::{input::get_string, levels::Level};
use colored::*;
use core::num;
use game::Game;
use std::{env, io::Chain, iter::zip};

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    let starting_level = get_string(
        "what level would you like to start at (just press enter to start at beginning): ",
    )
    .parse::<u32>()
    .unwrap_or(1);

    clear();

    println!("{}", "Instructions".underline());
    println!("you will be given a list of variables whose values are unknown");
    println!(
        "you can insert these variables into different functions to try to ascertain their values"
    );
    println!("type any of the available variables to insert it into the underlines locations");
    println!("when you would like to guess the value of a variable do {{variable}}={{value}}");
    println!("try to win with fewest wrong guesses and function evaluations");
    println!("(note: if not otherwise specified, all variables will have values between 1 and 9 inclusive");
    println!("(second note: once you beat the game don't just spam enter or else the game will close and you won't get to see your final score)");
    println!();

    let mut wrong_guesses: u32 = 0;
    let mut function_evals: u32 = 0;
    let mut flawless_levels: u32 = 0;

    for level_index in starting_level.. {
        // let mut level_index = 1;  // change later
        let Some(level) = &Level::get_level(level_index)
        else {
            break;
        };
        let mut game = game::Game::new(level);
        get_string("press enter to start next level: ");

        let mut flawless = true;
        let arg_colors = level.arg_colors();
        let mut prev_guess = None;
        // while any falses are in the game.known vec
        'gameloop: while game.known().iter().any(|&b| !b) {
            let mut letters: Vec<char> = vec![];
            // let mut values: Vec<i32> = vec![];

            if let Some(false) = prev_guess {
                flawless = false;
                wrong_guesses += 1;
            }
            if let Some(used_letters) = game.used_letters_mut() {
                used_letters.clear();
            }
            for selected_arg_index in 0..arg_colors.len() {
                // //todo remove debug
                // println!("{}", letters.len().to_string().red());
                // println!("{:?}", letters);
                // if selected_arg_index < letters.len() {
                //     println!("skipping");
                //     continue;
                // }
                print_info(&game, prev_guess);
                // now the funcstring should be done being printed
                print_colored_func_string(&letters, &game, selected_arg_index);
                println!();

                let user_input: UserInput = get_input(&mut game, selected_arg_index);
                let input = match user_input {
                    UserInput::Letter(letter) => vec![letter],
                    UserInput::ManyLetters(many_letters) => many_letters,
                    UserInput::Guess(accuracy) => {
                        prev_guess = Some(accuracy);
                        continue 'gameloop;
                    }
                };

                for letter in input {
                    letters.push(letter);
                    println!("{:?}", letters)
                }

                // if letters.len() == level.num_args() {
                //     break;
                // }
            }
            let values = letters_to_values(&letters, game.values());
            let answer = (level.func())(&values);
            game.history.push((letters, answer));
            function_evals += 1;
        }
        clear();
        flawless_levels += if flawless { 1 } else { 0 };
        println!("you did it! you beat the level!");
        println!("wrong guesses so far = {}", wrong_guesses.to_string().red());
        println!(
            "levels beaten without any wrong guesses = {}",
            flawless_levels.to_string().green()
        );
        println!(
            "functions evaluations so far = {}",
            function_evals.to_string().blue()
        );
    }
    clear();
    println!("{}", "CONGRATULATIONS!".bright_white().underline());
    get_string("");
    println!();
    println!("you beat the game!");
    get_string("");
    println!(
        "you beat {} levels without any wrong guesses",
        flawless_levels.to_string().green()
    );
    get_string("");
    if starting_level > 1 {
        println!(
            "(but you also skipped {} levels)",
            (starting_level - 1).to_string().red()
        )
    };
    get_string("");
    println!(
        "you made {} function evaluations",
        function_evals.to_string().blue()
    );
    get_string("");
    println!(
        "you made a total of {} wrong guesses",
        wrong_guesses.to_string().red()
    );

    print!("press enter to close game");
    get_string("");
}

fn letters_to_values(letters: &[char], true_values: &[i32]) -> Vec<i32> {
    letters
        .iter()
        .map(|&c| char_to_index(c).expect("these should be preverified values"))
        .map(|idx| true_values[idx])
        .collect()
}

enum UserInput {
    Letter(char),
    ManyLetters(Vec<char>),
    Guess(bool),
}

fn get_input(game: &mut Game, num_letters_used_so_far: usize) -> UserInput {
    loop {
        let input = input::get_string("enter guess or number: ")
            .split_whitespace()
            .collect::<String>();

        match input.split_once('=') {
            Some((var, value)) => {
                let (Ok(c), Ok(value)) = (var.parse::<char>(), value.parse::<i32>())
                else {
                    println!("Error: format is {{char}} = {{num}}");
                    continue;
                };
                let Some(value_index) = char_to_index(c)
                else {
                    println!("Error: character used not a letter");
                    continue;
                };
                let Some(&true_value) =  game.value_at(value_index)
                else {
                    println!("Error: letter not in game");
                    continue;
                };
                if true_value == value {
                    game.known_mut()[value_index] = true;
                    return UserInput::Guess(true);
                } else {
                    return UserInput::Guess(false);
                }
            }
            // there is no = in the expression
            None => {
                if let Ok(letter) = input.parse::<char>() {
                    match char_to_index(letter) {
                        Some(index) if index < game.level().num_values() => {
                            if let Some(used_letters) = game.used_letters_mut() {
                                if used_letters.contains(&letter) {
                                    println!("error: variable already used");
                                    continue;
                                } else {
                                    used_letters.push(letter)
                                }
                            }

                            return UserInput::Letter(letter);
                        }
                        Some(_) => println!("error: this letter is not in this level"),
                        None => println!("error: this is not a lowercase letter"),
                    }
                // input is not a single letter
                } else {
                    // TODO: Fix this clusterfuck
                    continue;
                    let mut checked_so_far = vec![];
                    for letter in input.chars() {
                        match char_to_index(letter) {
                            Some(index) if index < game.level().num_values() => {
                                if let Some(used_letters) = game.used_letters_mut() {
                                    if used_letters.contains(&letter) {
                                        println!(
                                            "error: input contains variable that was already used"
                                        );
                                        continue;
                                    } else if checked_so_far.contains(&letter) {
                                        println!("error: can't use the same variable twice");
                                        continue;
                                    } else {
                                        checked_so_far.push(letter)
                                    }
                                }
                            }
                            Some(_) => println!("error: used aletter that is not in this level"),
                            None => {
                                println!("error: contains character that is not a lowercase letter")
                            }
                        }
                    }
                    if checked_so_far.len() + num_letters_used_so_far > game.level().num_args() {
                        println!(
                            "too many letters (used {} when only {} more can be used",
                            checked_so_far.len(),
                            game.level().num_args() - num_letters_used_so_far
                        );
                        continue;
                    }
                    return UserInput::ManyLetters(checked_so_far);
                }
            }
        }
    }
}

/**
Converts a character to the letter of the alphabet it is \
Example: a -> Some(1), b -> Some(2) etc \
Uppercase letters and not letters map to None
*/
fn char_to_index(c: char) -> Option<usize> {
    ASCII_LOWER.iter().position(|&e| e == c)
}

fn print_colored_func_string(letters: &[char], game: &Game, selected_arg_index: usize) {
    let level = game.level();
    let arg_color = level.arg_colors();
    for token in level.func_string().split(' ') {
        let Some(hash_index) = token.find('#')
        else {
            print!("{token} ");
            continue;
        };

        let arg_index: usize = (token[hash_index + 1..])
            .trim_end_matches(')')
            .trim_end_matches(',')
            .parse()
            .expect("something that isn't a number follows a hash in func_string");

        let color = arg_color[arg_index];

        let arg_string: String = match letters.len() > arg_index {
            true => {
                let letter = letters[arg_index];
                let index = char_to_index(letter).expect("should be preverified");
                if game.known()[index] {
                    game.values()[index].to_string()
                } else {
                    letter.to_string()
                }
            }
            false => "?".to_owned(),
        };

        let colored_arg = match arg_index == selected_arg_index {
            true => arg_string.color(color).underline(),
            false => arg_string.color(color),
        };

        let num_digits = arg_index.to_string().chars().count();

        print!("{}", token[..hash_index].to_string());
        print!("{}", colored_arg);
        print!("{}", token[hash_index + num_digits + 1..].to_string());
        print!(" ")
    }
}

fn print_history(game: &Game) {
    let history = &game.history;
    for (letters, answer) in history {
        print_colored_func_string(letters, game, game.level().num_args() + 1);
        print!(" = {}", answer);
        println!()
    }
}

fn print_info(game: &Game, prev_guess: Option<bool>) {
    clear();
    game.level().config().show_info();
    for (index, (&known, &value)) in zip(game.known(), game.values()).enumerate() {
        let lhs = ASCII_LOWER[index].to_string();
        let rhs = if known {
            value.to_string()
        } else {
            '?'.to_string()
        };
        println!("{lhs} = {rhs}")
    }
    println!("---------------------------------------------------------------------------------------------");
    println!("{}", "HISTORY".underline());
    // println!("---------------------------------------------------------------------------------------------");
    println!();
    print_history(game);
    println!("---------------------------------------------------------------------------------------------");
    if let Some(guess) = prev_guess {
        match guess {
            true => println!("{}", "That's correct!".green()),
            false => println!("{}", "sorry that's wrong :(".red()),
        }
    }
}

#[cfg(not(debug_assertions))]
fn clear() {
    use std::process::Command;

    // Clear the terminal screen:
    if cfg!(target_os = "windows") {
        Command::new("cls").status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    };
}
#[cfg(debug_assertions)]
fn clear() {
    println!();
    println!("cleared screen");
    println!();
}
