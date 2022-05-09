use std::{fs, thread};

use crate::lib::counter::{Counter, CounterMap};

pub fn execute(folder_path: &str) -> CounterMap {
    let paths = fs::read_dir(folder_path).unwrap();

    let mut counters = vec![];

    for path in paths {
        counters.push(thread::spawn(move || -> Counter {
            let mut counter = Counter::new();
            counter.read_from_file(&path.unwrap().path());
            counter
        }));
    }

    counters
        .into_iter()
        .fold(Counter::new(), |mut acc, curr| {
            acc.merge_with(&curr.join().unwrap().data);
            acc
        })
        .data
}

#[cfg(test)]
mod test;
