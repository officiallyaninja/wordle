use std::{
    io::{self, Write},
    str::FromStr,
};

pub fn get_string(text: &str) -> String {
    let mut input = String::new();
    print!("{text}");
    io::stdout().flush().expect("CONSOLE FLUSH ERROR");

    io::stdin().read_line(&mut input).expect("READ ERROR");
    input
}

pub fn parse_input<T>(text: &str, err_msg: &str, cond: Option<fn(T) -> bool>) -> T
where
    T: FromStr + Copy,
{
    let mut input;
    loop {
        input = String::new();
        print!("{text}");
        io::stdout().flush().expect("CONSOLE FLUSH ERROR");

        io::stdin().read_line(&mut input).expect("READ ERROR");

        if let Ok(parsed) = input.trim().parse() {
            match cond {
                Some(func) => {
                    if func(parsed) {
                        return parsed;
                    }
                }
                None => return parsed,
            }
        }

        if err_msg != "" {
            println!("{err_msg}");
        }
    }
}

fn input_with_quit<T>(
    text: &str,
    quit_msg: &str,
    err_msg: &str,
    cond: Option<fn(T) -> bool>,
) -> Option<T>
where
    T: FromStr + Copy,
{
    let mut input;
    loop {
        input = String::new();
        print!("{text}");
        io::stdout().flush().expect("CONSOLE FLUSH ERROR");

        io::stdin().read_line(&mut input).expect("READ ERROR");

        if input.trim() == quit_msg.trim() {
            return None;
        }

        if let Ok(parsed) = input.trim().parse() {
            match cond {
                Some(func) => {
                    if func(parsed) {
                        return Some(parsed);
                    }
                }
                None => return Some(parsed),
            }
        }
        if err_msg != "" {
            println!("{err_msg}");
        }
    }
}

pub fn vec_input<T>(
    text: &str,
    quit_msg: &str,
    err_msg: &str,
    cond: Option<fn(T) -> bool>,
) -> Vec<T>
where
    T: FromStr + Copy,
{
    let mut list: Vec<T> = vec![];
    loop {
        match input_with_quit(text, err_msg, quit_msg, cond) {
            Some(value) => list.push(value),
            None => return list,
        }
    }
}

pub fn input_one_of(text: &str, err_msg: &str, valid_inputs: Vec<&str>) -> String {
    let mut input;
    loop {
        input = String::new();
        print!("{text}");
        io::stdout().flush().expect("CONSOLE FLUSH ERROR");

        io::stdin().read_line(&mut input).expect("READ ERROR");
        let input = input.trim();
        if valid_inputs.contains(&input) {
            return input.to_owned();
        }
        println!("{err_msg}");
    }
}
