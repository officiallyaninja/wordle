use crate::levels::{Constraint, Level};
use rand::{self, seq::SliceRandom, Rng};
// let mut num_levels = 0;
pub struct Game<'a> {
    pub level: &'a Level,
    values: Vec<i32>, // has to be i32 but really they only vary from 1-9
    known: Vec<bool>,
    pub history: Vec<(Vec<i32>, i32)>,
}
impl<'a> Game<'a> {
    pub fn new(level: &'a Level) -> Self {
        let mut rng = rand::thread_rng();
        let mut values: Vec<i32> = vec![];
        let num_values = level.num_values();

        if level.constraints().contains(&Constraint::UniqueValues) {
            let mut range: Vec<i32> = (0..10).collect();
            range.shuffle(&mut rng);

            values = range[0..num_values].to_vec()
        } else {
            for _ in 0..num_values {
                values.push(rng.gen_range(1..10)) //generates values from 1-10
            }
        }

        Self {
            level,
            values,
            known: vec![false; num_values],
            history: vec![],
        }
    }
}
