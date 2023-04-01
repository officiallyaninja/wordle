use crate::levels::Level;
use rand::{self, seq::SliceRandom, Rng};
// let mut num_levels = 0;
pub struct Game<'a> {
    level: &'a Level,
    values: Vec<i32>, // has to be i32 but really they only vary from 1-9
    known: Vec<bool>,
    used_letters: Option<Vec<char>>,
    pub history: Vec<(Vec<char>, i32)>,
}
impl<'a> Game<'a> {
    pub fn new(level: &'a Level) -> Self {
        let mut rng = rand::thread_rng();
        let mut values: Vec<i32> = vec![];
        let num_values = level.num_values();
        let mut range: Vec<i32> = match level.config().range {
            Some((lower, higher)) => (lower..=higher).collect(),
            None => (1..10).collect(),
        };

        if level.config().unique_values {
            range.shuffle(&mut rng);

            values = range[0..num_values].to_vec()
        } else {
            for _ in 0..num_values {
                values.push(rng.gen_range(1..10)) //generates values from 1-10
            }
        }

        let used_letters = if level.config().unique_arguments {
            Some(vec![])
        } else {
            None
        };

        Self {
            level,
            values,
            known: vec![false; num_values],
            used_letters,
            history: vec![],
        }
    }

    pub fn values(&self) -> &[i32] {
        self.values.as_ref()
    }

    pub(crate) fn value_at(&self, index: usize) -> Option<&i32> {
        self.values.get(index)
    }

    pub fn known_mut(&mut self) -> &mut Vec<bool> {
        &mut self.known
    }

    pub fn level(&self) -> &Level {
        self.level
    }

    pub fn known(&self) -> &[bool] {
        self.known.as_ref()
    }

    pub fn used_letters_mut(&mut self) -> &mut Option<Vec<char>> {
        &mut self.used_letters
    }
}
