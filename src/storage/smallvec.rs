use crate::smallvec::{Array, SmallVec};
use crate::storage::{IndexableRow, Storage};
use std::ops::Deref;

pub type SmallVecBackend<Row> = Vec<SmallVec<Row>>;
// pub type Row<Field> = SmallVec<Field>;

impl<Row: Array> Storage for SmallVecBackend<Row>
where
    Row::Item: Clone,
{
    type Row = SmallVec<Row>;
    type Item = Row::Item;

    fn new() -> Self {
        Vec::new()
    }

    fn from_iter<I: Iterator<Item = Self::Row>>(iterator: I) -> Self {
        std::iter::FromIterator::from_iter(iterator)
    }

    fn columns<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self::Row> + 'a> {
        Box::new(self.into_iter())
    }
    fn to_vec(self) -> Vec<Self::Row> {
        self
    }

    fn to_owned_vec(&self) -> Vec<Self::Row> {
        Clone::clone(self)
    }

    fn swap(&mut self, a: usize, b: usize) {
        self.as_mut_slice().swap(a, b)
    }

    fn get(&self, index: usize) -> Option<&Self::Row> {
        self.as_slice().get(index)
    }
}

impl<Row: Array> IndexableRow for SmallVec<Row> {
    type Item = Row::Item;
    fn get(&self, index: usize) -> Option<&Self::Item> {
        self.deref().get(index)
    }
}

#[cfg(test)]
mod tests {
    use crate::smallvec::smallvec;
    use crate::SmallVecBackend;

    #[test]
    fn it_works() {
        let mut example = SmallVecBackend::<[u8; 4]>::new();

        example.push(smallvec![1, 2, 3, 4]);
        example.push(smallvec![1, 2, 3, 4]);
        example.push(smallvec![1, 2, 3, 4]);
        example.push(smallvec![1, 2, 3, 4]);
        example.push(smallvec![1, 2, 3, 4]);
        example.push(smallvec![1, 2, 3, 4]);

        assert_eq!(example.len(), 6);
    }
}
