pub struct Repeater {
    n: usize,
}

impl Repeater {
    pub(crate) fn new(n: usize) -> Self {
        Self {
            n,
        }
    }

    pub fn run<F>(self, mut f: F) where F: FnMut() {
        for _ in 0..self.n {
            f();
        }
    }
}

#[cfg(test)]
mod should {
    use super::*;
    use test_case::test_case;

    #[test_case(0)]
    #[test_case(1)]
    #[test_case(2)]
    #[test_case(3)]
    fn loop_for_n_times(n: usize) {
        let mut counter = 0;

        let sut = Repeater::new(n);
        sut.run(|| counter += 1);

        assert_eq!(counter, n)
    }
}