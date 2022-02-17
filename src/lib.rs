use crate::repeater::Repeater;

mod repeater;

pub trait Countable {
    fn times(&self) -> Repeater;
}

// todo: is this cloning needed?
impl<I> Countable for I where I: Into<usize> + Clone {
    fn times(&self) -> Repeater {
        Repeater::new(self.clone().into())
    }
}