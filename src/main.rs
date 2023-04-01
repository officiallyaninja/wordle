mod game;
mod input;
mod levels;

use game::Game;
use std::{
    env, iter::zip,
};
use colored::*;
use crate::{input::get_string, levels::Level};


static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
];
fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    let starting_level = 
                    get_string("what level would you like to start at (just press enter to start at beginning): ")
                    .parse::<u32>()
                    .unwrap_or(1);
    

    for level_index in starting_level..{
        // let mut level_index = 1;  // change later
        let Some(level) = &Level::get_level(level_index)
        else {
            break;
        };
        let mut game = game::Game::new(level);

        get_string("press enter to start next level: ");
        
        let arg_colors = level.arg_colors(); 
        let mut prev_guess = None;
                // while any falses are in the game.known vec
        'gameloop: while game.known().iter().any(|&b| !b) {
            let mut letters : Vec<char> = vec![];
            // let mut values: Vec<i32> = vec![];

            for selected_arg_index in 0..arg_colors.len() {
                print_info(&game, prev_guess);
                // now the funcstring should be done being printed
                print_colored_func_string(&letters, &game, selected_arg_index);
                println!();

                let maybe_letter: UserInput = get_input(&mut game);
                let letter = match maybe_letter {
                    UserInput::Letter(letter) => letter,
                    UserInput::Guess(accuracy) => {
                        prev_guess = Some(accuracy);
                        continue 'gameloop;
                    },
                };
                // let UserInput::Letter(letter) = maybe_letter
                // else {
                //     prev_guess = 
                //     continue 'gameloop
                // };
                letters.push(letter);
            }
            let values = letters_to_values(&letters, game.values());
            let answer = (level.func())(&values);
            game.history.push((letters, answer))
        }
        println!("you did it! you win!");
    }
    println!("Congratulations! you beat the game!");
    get_string("press enter to close game:");
}

fn letters_to_values(letters: &[char], true_values: &[i32]) -> Vec<i32> {
    letters.
    iter()
    .map(|&c| char_to_index(c).expect("these should be preverified values"))
    .map(|idx| true_values[idx])
    .collect()
}


enum UserInput {
    Letter(char),
    Guess(bool),
}

fn get_input(game: &mut Game) -> UserInput {
    loop {
        let input = input::get_string("enter guess or number: ").
                                        split_whitespace().
                                        collect::<String>();

        match input.split_once("=") {
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
                    return UserInput::Guess(true)
                }
                else {
                    return UserInput::Guess(false)
                }
                

            },
            None => {
                if let Ok(letter) = input.parse::<char>(){

                        match char_to_index(letter) {
                            Some(index) if index < game.level().num_values() 
                                                    => return UserInput::Letter(letter),
                            Some(_) => println!("error: this letter is not in this level"),
                            None => println!("error: this is not a letter")
                        }
                } else {
                    println!("error: that's not valid letter");
                }
            },
        }
    }
}


fn char_to_index(c: char) -> Option<usize>{
    ASCII_LOWER.iter().position(|&e| e == c)
}



fn print_colored_func_string(
    letters: &[char],
    game: &Game,
    selected_arg_index: usize,
) {
    let level = game.level();
    let arg_color = level.arg_colors();
    for token in level.func_string().split(" ") {
        let Some(hash_index) = token.find("#")
        else {
            print!("{token} ");
            continue; 
        };


        
        let arg_index: usize = (&token[hash_index + 1..])
            .trim_end_matches(")")
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
            },
            false => "?".to_owned(),
        };

        

        let colored_arg = match arg_index == selected_arg_index {
            true => arg_string.color(color).underline(),
            false => arg_string.color(color),
        };

        match (token.starts_with("("), token.ends_with(")")) {
            (true, true) => print!("({}) ", colored_arg),
            (true, false) => print!("({} ", colored_arg),
            (false, true) => print!("{}) ", colored_arg),
            (false, false) => print!("{} ", colored_arg),
        }
    }
}

fn print_history(game: &Game) {
    let history = &game.history;
    for (letters, answer) in history{
        
        print_colored_func_string(letters, game, game.level().num_args()+1);
        print!(" = {}", answer);
        println!()
    } 

}


fn print_info(game: &Game, prev_guess: Option<bool>) {
    clear();
    for (index, (&known, &value)) in zip(game.known(), game.values()).enumerate() {
        let lhs = ASCII_LOWER[index].to_string();
        let rhs = if known {value.to_string()} else {"?".to_string()};
        println!("{lhs} = {rhs}")
    }
    println!("{}", "HISTORY".underline());
    println!("----------------------------------------");
    print_history(&game);
    println!("----------------------------------------");
    if let Some(guess) = prev_guess {
        match guess {
            true => println!("{}", "That's correct!".green()),
            false => println!("{}", "sorry that's wrong :(".red())
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

//foo
