use colored::{Color, Colorize};
use gcd::Gcd;
use std::{cmp, collections::HashSet};
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]

enum Constraint {
    // note, the order it's defined matters here, don't fuck with it
    UniqueArguments,
    InRange(i32, i32),
    // Brittle { index: usize, brittleness: u32 },
}
#[derive(Debug)]
pub struct Config {
    // pub unique_values: bool,
    pub unique_arguments: bool,
    pub range: Option<(i32, i32)>,
}
impl Config {
    fn new() -> Self {
        Self {
            unique_arguments: false,
            range: None,
        }
    }

    pub fn show_info(&self) {
        let mut text: Vec<(String, String)> = vec![];

        text.push((
            "Unique Variables".to_string(),
            "all variables have unique values that are distinct from each other\n".to_string(),
        ));

        if self.unique_arguments {
            text.push((
                "Unique Arguments".to_string(),
                "all function arguments must be distinct(you cannot use the same variable twice in the same function)\n"
                    .to_string(),
            ))
        }
        let (lower, higher) = self.range.unwrap_or((1, 9));
        text.push((
            "In Range".to_string(),
            format!(
                "all values are between {} and {} inclusive\n",
                lower.to_string().green(),
                higher.to_string().green()
            ),
        ));

        if !text.is_empty() {
            println!("{}", "SPECIAL RULES".underline());
            println!();
            for (heading, body) in text {
                print!("{}: {}", heading.cyan(), body);
            }
            println!(
                "---------------------------------------------------------------------------------------------"
            );
        }
    }
}

pub struct Level {
    // id: u32,
    num_values: usize,
    num_args: usize,
    func: fn(&[i32]) -> i32,
    func_string: &'static str,
    config: Config,
    arg_colors: Vec<Color>,
}

impl Level {
    fn new(
        // id: u32,
        num_values: usize,
        func: fn(&[i32]) -> i32,
        func_string: &'static str,
        constraints: Vec<Constraint>,
    ) -> Self {
        // finding number of args
        let mut max_num = 0;
        let mut split = func_string.split('#');
        split.next(); // first match will be the empty string, which we want to ignore
        for slice in split {
            let num: usize = slice.split(' ').
                            next(). // gets the number after the #
                            expect("there is a hash without anything after it").
                            trim_end_matches(')').
                            trim_end_matches(',').
                            parse().expect("something that isn't a number follows the hash");
            max_num = cmp::max(num, max_num);
        }
        let num_args = max_num + 1; // this is the number of args
        func(&vec![0; num_args]); // panics if num args is smaller than the number of args actually used

        //finding constrints
        let constraints: HashSet<Constraint> = HashSet::from_iter(constraints.into_iter());

        let mut config = Config::new();
        for constraint in &constraints {
            match constraint {
                // Constraint::UniqueValues => config.unique_values = true,
                Constraint::UniqueArguments => config.unique_arguments = true,
                Constraint::InRange(lower, higher) => {
                    assert!(lower < higher);
                    if config.range.is_some() {
                        panic!("error there are 2 different ranges on same level");
                    }
                    config.range = Some((*lower, *higher))
                }
            }
        }

        //finding colors
        let mut arg_colors: Vec<Color> = vec![Color::BrightWhite; num_args];
        let mut colors_used: u32 = 0;
        for token in func_string.split(' ') {
            let Some(hash_index) = token.find('#')
            else {
                continue;
            };

            let arg_index: usize = (token[hash_index + 1..])
                .trim_end_matches(')')
                .trim_end_matches(',')
                .parse()
                .expect("something that isn't a number follows a hash in func_string");
            match func_string.matches(token).count() {
                0 => panic!("pattern taken from func_string doesn't exist in func_string...this feels impossible"),
                1 => (),
                _ => {
                    arg_colors[arg_index] = get_color(colors_used);
                    colors_used += 1
                },
            }
        }

        Self {
            // id,
            num_args,
            num_values,
            func,
            func_string,
            config,
            arg_colors,
        }
    }

    pub fn get_level(index: u32) -> Option<Level> {
        // let mut num_levels = 0;
        // let mut get_id = || {
        //     num_levels += 1;
        //     return num_levels;
        // };

        use Constraint::*;
        let level = match index {
            ..=0 => panic!("error, level index less than 1"),
            1 => Level::new(4, |v| v[0] + v[1], "#0 + #1", vec![InRange(1, 4)]),
            2 => Level::new(4, |v| v[0] - v[1], "#0 - #1", vec![InRange(1, 4)]),
            3 => Level::new(
                4,
                |v| v[0] + v[1] - v[2],
                "#0 + #1 - #2",
                vec![UniqueArguments, InRange(1, 4)],
            ),
            4 => Level::new(
                4,
                |v| v[0] * v[0] + v[1] * v[1] + v[2],
                "#0 ^ 2 + #1 ^ 2 + #2",
                vec![InRange(1, 4)],
            ),
            5 => Level::new(
                4,
                |v| v[0] * v[1] - v[2] * v[3],
                "#0 * #1 - #2 * #3",
                vec![InRange(1, 4)],
            ),
            6 => Level::new(
                5,
                |v| {
                    (v[0] * v[1] * v[2])
                        .checked_sub_unsigned(v[3].unsigned_abs().gcd(v[4].unsigned_abs()))
                        .expect("how the fuck did you overflow")
                },
                "#0 * #1 * #2 - gcd(#3, #4)",
                vec![InRange(1, 5)],
            ),
            7 => Level::new(
                2,
                |v| 2 * v[0] * v[0] - 7 * v[0] + 4,
                "2 * #0 ^ 2 - 7 * #0 + 4",
                vec![InRange(1, 2)],
            ),
            8 => Level::new(
                4,
                |v| 2 * v[0] + 3 * v[1] - 4 * v[2],
                "2 * #0 + 3 * #1 - 4 * #2",
                vec![UniqueArguments, InRange(1, 4)],
            ),
            9 => Level::new(
                6,
                |v| (v[0] * v[1] * v[2]) % 10,
                "(#0 * #1 * #2) % 10",
                vec![UniqueArguments, InRange(2, 7)],
            ),
            10 => Level::new(
                5,
                |v| ((v[0] * v[1] * v[2]) % 10) + v[3],
                "((#0 * #1 * #2) % 10) + #3",
                vec![UniqueArguments, InRange(1, 5)],
            ),
            11 => Level::new(
                5,
                |v| 2 * v[0] * v[0] - 7 * v[0] + 4 + v[1],
                "2 * #0 ^ 2 - 7 * #0 + 4 + #1",
                vec![UniqueArguments, InRange(1, 5)],
            ),
            12 => Level::new(
                5,
                |v| v[0] * v[1].max(v[2]) + v[3] * (v[4] % 2),
                "#0 * max(#1, #2) + #3 * (#4 % 2)",
                vec![UniqueArguments, InRange(1, 5)],
            ),

            _ => return None,
        };
        Some(level)
    }

    pub fn arg_colors(&self) -> &[Color] {
        self.arg_colors.as_ref()
    }

    pub fn func_string(&self) -> &str {
        self.func_string
    }

    pub fn num_values(&self) -> usize {
        self.num_values
    }

    pub fn num_args(&self) -> usize {
        self.num_args
    }

    pub fn func(&self) -> fn(&[i32]) -> i32 {
        self.func
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

#[cfg(test)]
impl Level {
    fn verify_config(&self) {
        let (lower, higher) = self.config().range.unwrap_or((1, 9));

        assert!(lower < higher);

        let max_num_unique = higher - lower + 1;
        assert!(self.num_values <= max_num_unique.try_into().unwrap());

        if self.config().unique_arguments {
            assert!(self.num_args() <= self.num_values())
        }
    }

    fn all_levels() -> impl Iterator<Item = Self> {
        (1..).map_while(|idx| Level::get_level(idx))
    }
}

/*
=> Level::new(
                ,
                |v| ,
                "",
                vec![],
            ),
*/

//helper funcs

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

#[cfg(test)]
mod tests {
    use super::Level;

    #[test]
    fn verify_levels() {
        let mut num = 0;
        for level in Level::all_levels() {
            println!("{}", num);
            level.verify_config();
            num += 1;
        }
    }

    #[test]
    fn verify_num_levels() {
        assert!(Level::all_levels().count() == 6)
    }
}
