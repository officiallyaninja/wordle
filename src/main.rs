mod game;
mod input;
mod levels;

use game::Game;
use input::parse_input;
use levels::Level;
use std::{
    env,
};

use colored::*;


static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
];
fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let mut level_index = parse_input("what level do you want to enter: ", "dumbass", None);  // change later
    let level = &Level::get_level(level_index);
    let mut game = game::Game::new(level);


    // assigning colors
    
    let arg_colors = level.arg_colors(); 
    loop {
        let mut values: Vec<i32> = vec![];

        for selected_arg_index in 0..arg_colors.len() {
            clear();
            println!("{}", "HISTORY".underline());
            println!("----------------------------------------");
            print_history(&game);
            println!("----------------------------------------");
            // now the funcstring should be done being printed
            print_colored_func_string(&values, level, &arg_colors, selected_arg_index);
            println!();

            let num: i32 = get_input(&mut game);
            values.push(num);
        }
        let answer = (level.func())(&values);
        game.history.push((values, answer))
    }
}



//   HELPER FUNCTIONS



/// also does checks if user enters a check input
fn get_input(game: &mut Game) -> i32 {
    loop {
        let input = input::get_string("enter guess or number: ");
        if let Ok(num) = input.parse() {
            if 0 < num && num < 10 { 
                return num;
            } else {
            println!("error number should be between 1 and 0");
            continue;
            }
        } 
    }
}


fn print_colored_func_string(
    values: &[i32],
    level: &Level,
    arg_color: &[Color],
    selected_arg_index: usize,
) {
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

        let arg_string: String = match values.len() > arg_index {
            true => values[arg_index].to_string(),
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
    for (values, answer) in history{
        
        print_colored_func_string(values, game.level, game.level.arg_colors(), game.level.num_args()+1);
        print!(" = {}", answer);
        println!()
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
