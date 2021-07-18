pub mod smallvec;
pub mod vec;

pub use self::smallvec::SmallVecBackend;
pub use vec::VecBackend;

pub trait Storage {
    type Row: IndexableRow<Item = Self::Item>;
    type Item;

    fn new() -> Self;
    fn from_iter<I: Iterator<Item = Self::Row>>(iterator: I) -> Self;
    fn columns<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self::Row> + 'a>;
    fn to_vec(self) -> Vec<Self::Row>;
    fn to_owned_vec(&self) -> Vec<Self::Row>;
    fn swap(&mut self, a: usize, b: usize);

    fn get(&self, index: usize) -> Option<&Self::Row>;
}

pub trait IndexableRow {
    type Item;

    fn get(&self, index: usize) -> Option<&Self::Item>;
}
