use std::fs;

use crate::lib::counter::{Counter, CounterMap};

pub fn execute(folder_path: &str) -> CounterMap {
    let paths = fs::read_dir(folder_path).unwrap();

    let counters = paths.map(|path| {
        let mut counter = Counter::new();
        counter.read_from_file(&path.unwrap().path());
        counter
    });

    counters
        .fold(Counter::new(), |mut acc, curr| {
            acc.merge_with(&curr.data);
            acc
        })
        .data
}

#[cfg(test)]
mod test;
