use rayon::prelude::*;

use std::{fs, path::PathBuf};

use crate::lib::counter::{Counter, CounterMap};

pub fn execute(folder_path: &str) -> CounterMap {
    let paths = fs::read_dir(folder_path)
        .unwrap()
        .filter_map(|entry| entry.ok().and_then(|e| Some(e.path())))
        .collect::<Vec<PathBuf>>();

    let counters = paths.par_iter().map(|path| {
        let mut counter = Counter::new();
        counter.read_from_file(&path);
        counter
    });

    counters
        .reduce(
            || Counter::new(),
            |mut acc, curr| {
                acc.merge_with(&curr.data);
                acc
            },
        )
        .data
}

#[cfg(test)]
mod test;
