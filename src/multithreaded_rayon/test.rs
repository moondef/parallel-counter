extern crate test;

#[cfg(test)]
mod tests {
    use crate::{lib::constants::MOCK_FOLDER, multithreaded_rayon};

    use super::*;
    use test::Bencher;

    #[bench]
    fn main_bench(b: &mut Bencher) {
        b.iter(|| multithreaded_rayon::execute(MOCK_FOLDER));
    }
}
