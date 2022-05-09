#![feature(test)]

use std::{collections::HashMap, env};

pub mod lib;

mod multithreaded;
mod multithreaded_rayon;
mod single_thread;
mod single_thread_reduce;

fn main() {
    let args: Vec<String> = env::args().collect();

    let folder_path = &args[1];
    let mode = args[2].parse::<u32>().unwrap();

    let counter = match mode {
        1 => single_thread::execute(&folder_path),
        2 => single_thread_reduce::execute(&folder_path),
        3 => multithreaded::execute(&folder_path),
        4 => multithreaded_rayon::execute(&folder_path),
        _ => HashMap::new(),
    };

    for (key, value) in counter {
        println!("{} entries {} times", key, value);
    }
}
