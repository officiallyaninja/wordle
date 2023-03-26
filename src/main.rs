mod game;
mod input;
mod levels;

use levels::Level;
use std::{
    collections::HashMap,
    env,
    hash::{self, Hash},
    num::ParseIntError,
};

use colored::*;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let _: String = input::get_string("press enter");
    let levels = Level::get_levels();
    let mut level_index = 2;
    let level = &levels[level_index];
    let mut game = game::Game::new(level);
    let func_string = game.level.func_string;
    let mut arg_color: Vec<Color> = vec![Color::BrightBlack; level.num_args];
    let mut colors_used: u32 = 0;

    // assigning colors
    for token in func_string.split(" ") {
        let Some(hash_index) = token.find("#")
        else {
            continue;
        };

        let arg_index: usize = (&token[hash_index + 1..])
            .trim_end_matches(")")
            .parse()
            .expect("something that isn't a number follows a hash in func_string");
        match func_string.matches(token).count() {
            0 => panic!("pattern taken from func_string doesn't exist in func_string...this feels impossible"),
            1 => (),
            _ => {
                arg_color[arg_index] = get_color(colors_used);
                colors_used += 1
            },
        }
    }
    let arg_color = arg_color; // now arg_color should be immutable
    loop {
        let mut values: Vec<i32> = vec![];

        for selected_arg_index in 0..arg_color.len() {
            clear();
            println!("{}", "HISTORY".underline());
            println!("----------------------------------------");
            print_history(&game.history);
            println!("----------------------------------------");
            // now the funcstring should be done being printed
            print_colored_func_string(&values, level, &arg_color, selected_arg_index);
            println!();

            let num: i32 = input::parse_input(
                "Enter number to insert into underlined char: ",
                "Error, please enter an integer",
                None,
            );
            values.push(num);
        }
        game.history.push((values.clone(), (level.func)(values)))
    }
}

//

//
//
//
//
//   HELPER FUNCTIONS
//
//
//
//

//

fn get_color(colors_used: u32) -> Color {
    match colors_used {
        0 => Color::Blue,
        1 => Color::Red,
        2 => Color::Yellow,
        3 => Color::Green,
        4 => Color::Cyan,
        5 => Color::Magenta,
        6 => Color::BrightBlue,
        7 => Color::BrightGreen,
        8 => Color::BrightRed,
        _ => panic!("too many colors (too many repeated args (>8))"),
    }
}

fn print_colored_func_string(
    values: &Vec<i32>,
    level: &Level,
    arg_color: &Vec<Color>,
    selected_arg_index: usize,
) {
    for token in level.func_string.split(" ") {
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

fn print_history(history: &Vec<(Vec<i32>, i32)>) {
    for _ in history{
        println!("todo")
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
