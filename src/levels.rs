use std::cmp;
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Constraint {
    // note, the order it's defined matters here, don't fuck with it
    UniqueValues,
    UniqueArguments,
    Brittle { index: usize, brittleness: u32 },
}

pub struct Level {
    id: u32,
    pub num_values: usize,
    pub num_args: usize,
    pub func: fn(Vec<i32>) -> i32,
    pub func_string: &'static str,
    pub constraints: Vec<Constraint>,
}

impl Level {
    fn new(
        id: u32,
        num_values: usize,
        func: fn(Vec<i32>) -> i32,
        func_string: &'static str,
        mut constraints: Vec<Constraint>,
    ) -> Self {
        let mut max_num = 0;
        let mut split = func_string.split('#');
        split.next();
        // println!("{:?}", split.clone().collect::<Vec<&str>>());
        for slice in split {
            let num: usize = slice.split(' ').
                            next(). // should get each of the numbers after the #
                            expect("there is a hash without anything after it").
                            trim_end_matches(')').
                            parse().
                            expect("something that isn't a number follows the hash");
            max_num = cmp::max(num, max_num);
        }
        let num_args = max_num + 1; // this is the number of args
        func(vec![0; num_args]); // panics if num args is smaller than the number of args actually used
        constraints.sort();

        for constraint in &constraints {
            match constraint {
                Constraint::UniqueValues => {
                    if num_values > 9 {
                        panic!("you can't have more than 9 values if you want unique values")
                    }
                }
                Constraint::UniqueArguments => {
                    if num_args > 9 {
                        panic!("you can't have more than 9 args if you want unique args")
                    }
                }
                Constraint::Brittle { .. } => todo!(),
            }
        }

        Self {
            id,
            num_args,
            num_values,
            func,
            func_string,
            constraints,
        }
    }

    pub fn get_levels() -> Vec<Level> {
        let mut num_levels = 0;
        let mut get_id = || {
            num_levels += 1;
            return num_levels;
        };

        use Constraint::*;
        vec![
            Level::new(get_id(), 4, |v| v[0] + v[1], "#0 + #1", vec![UniqueValues]),
            Level::new(get_id(), 4, |v| v[0] - v[1], "#0 - #1", vec![UniqueValues]),
            Level::new(
                get_id(),
                5,
                |v| (v[0] * v[1] + v[2] - v[3]) * (v[1] - v[2]),
                "(#0 * #1 + #2 - #3) * (#1 - #2)",
                vec![UniqueValues, UniqueArguments],
            ),
        ]
    }
}

/*
Level::new(
    get_id(),
    num,
    |v| //stuff,
    "// stuff as str",
    vec![constrains],
),
*/
