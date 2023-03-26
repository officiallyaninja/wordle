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

const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let _: String = input::get_string("press enter");
    let levels = Level::get_levels();
    let mut level_index = 2;
    let level = &levels[level_index];
    let game = game::Game::new(level);
    let func_string = game.level.func_string;
    let mut arg_color: Vec<Color> = vec![Color::BrightBlack; func_string.split(" ").count()];
    let mut colors_used: u32 = 0;

    // assigning colors
    for token in func_string.split(" ") {
        let Some(hash_index) = token.find("#")
        else {
            print!("{token} ");
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
    loop {
        clear();
        println!("{}", "HISTORY".underline());
        // println!("----------------------------------------");
        print_history(&game.history);
        println!("----------------------------------------");
        for token in func_string.split(" ") {
            let Some(hash_index) = token.find("#")
            else {
                print!("{token} ");
                continue;
            };

            let arg_index: usize = (&token[hash_index + 1..])
                .trim_end_matches(")")
                .parse()
                .expect("something that isn't a number follows a hash in func_string");

            let colored_qn_mark = "?".color(arg_color[arg_index]);

            match (token.starts_with("("), token.ends_with(")")) {
                (true, true) => print!("({}) ", colored_qn_mark),
                (true, false) => print!("({} ", colored_qn_mark),
                (false, true) => print!("{}) ", colored_qn_mark),
                (false, false) => print!("{} ", colored_qn_mark),
            }
        } // now the funcstring should be done being printed
        println!();

        let _: String = input::get_string("Enter number to insert into underlined char: ");
    }
}

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

fn print_history(history: &Vec<(Vec<i32>, i32)>) {
    println!("todo")
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
