use std::fs;

use crate::lib::counter::{Counter, CounterMap};

pub fn execute(folder_path: &str) -> CounterMap {
    let paths = fs::read_dir(folder_path).unwrap();

    let mut words_counter = Counter::new();

    for path in paths {
        let mut next_counter = Counter::new();
        next_counter.read_from_file(&path.unwrap().path());
        words_counter.merge_with(&next_counter.data);
    }

    words_counter.data
}

#[cfg(test)]
mod test;
