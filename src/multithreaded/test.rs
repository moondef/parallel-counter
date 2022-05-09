extern crate test;

#[cfg(test)]
mod tests {
    use crate::{lib::constants::MOCK_FOLDER, multithreaded};

    use super::*;
    use test::Bencher;

    #[bench]
    fn main_bench(b: &mut Bencher) {
        b.iter(|| multithreaded::execute(MOCK_FOLDER));
    }
}
