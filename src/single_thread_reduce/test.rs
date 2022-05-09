extern crate test;

#[cfg(test)]
mod tests {
    use crate::{lib::constants::MOCK_FOLDER, single_thread_reduce};

    use super::*;
    use test::Bencher;

    #[bench]
    fn main_bench(b: &mut Bencher) {
        b.iter(|| single_thread_reduce::execute(MOCK_FOLDER));
    }
}
