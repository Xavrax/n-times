use crate::repeater::Repeater;

mod repeater;

pub trait Countable {
    fn times(self) -> Repeater;
}

impl<I> Countable for I
where
    I: Into<usize>
{
    fn times(self) -> Repeater {
        Repeater::new(self.into())
    }
}
