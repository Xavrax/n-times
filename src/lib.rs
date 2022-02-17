use crate::repeater::Repeater;

mod repeater;

pub trait Countable {
    fn times(&self) -> Repeater;
}
