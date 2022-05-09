use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub type CounterMap = HashMap<String, u128>;

pub struct Counter {
    pub data: CounterMap,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn read_from_file(&mut self, path: &PathBuf) -> &Self {
        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);

            for line_result in reader.lines() {
                if let Ok(line) = line_result {
                    let count = self.data.entry(line.to_owned()).or_insert(0);

                    *count += 1;
                }
            }
        }

        self
    }

    pub fn merge_with(&mut self, ext_map: &CounterMap) -> &Self {
        for (word, count) in ext_map {
            let current_count = self.data.entry(word.to_owned()).or_insert(0);
            *current_count += count;
        }

        self
    }
}
