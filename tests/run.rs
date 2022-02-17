use n_times::Countable;
use std::num::NonZeroUsize;
use test_case::test_case;

#[test_case(false => 0)]
#[test_case(true => 1)]
fn should_work_with_bool(boolean: bool) -> i32 {
    let mut counter = 0;

    boolean.times().run(|| counter += 1);

    counter
}

#[test_case(0)]
#[test_case(1)]
#[test_case(2)]
#[test_case(5)]
#[test_case(10)]
#[test_case(u8::MAX)]
fn should_work_with_u8(n: u8) {
    let mut counter = 0;

    n.times().run(|| counter += 1);

    assert_eq!(counter, n);
}

#[test_case(0)]
#[test_case(1)]
#[test_case(2)]
#[test_case(5)]
#[test_case(10)]
#[test_case(u16::MAX)]
fn should_work_with_u16(n: u16) {
    let mut counter = 0;

    n.times().run(|| counter += 1);

    assert_eq!(counter, n);
}

#[test_case(0)]
#[test_case(1)]
#[test_case(2)]
#[test_case(5)]
#[test_case(10)]
#[test_case(usize::MAX; "inconclusive - expensive test")]
fn should_work_with_usize(n: usize) {
    let mut counter = 0;

    n.times().run(|| counter += 1);

    assert_eq!(counter, n);
}

#[test_case(1)]
#[test_case(2)]
#[test_case(5)]
#[test_case(10)]
#[test_case(usize::MAX; "inconclusive - expensive test")]
fn should_work_with_non_zero_usize(n: usize) {
    let n = NonZeroUsize::new(n).unwrap();
    let mut counter = 0;
    some_condition.then(|| amount_of_something.times().run(|| println!("Hello World!")));

    n.times().run(|| {
        println!("{}", counter);
        counter += 1
    });

    assert_eq!(NonZeroUsize::new(counter).unwrap(), n);
}
