extern crate test;

#[cfg(test)]
mod tests {
    use crate::{single_thread, lib::constants::MOCK_FOLDER};

    use super::*;
    use test::Bencher;

    #[bench]
    fn main_bench(b: &mut Bencher) {
        b.iter(|| single_thread::execute(MOCK_FOLDER));
    }
}
