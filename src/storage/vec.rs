use crate::storage::{IndexableRow, Storage};
use std::ops::Deref;
pub type VecBackend<Field> = Vec<Row<Field>>;
pub type Row<Field> = Vec<Field>;

impl<Field: Clone> Storage for VecBackend<Field> {
    type Row = Row<Field>;
    type Item = Field;

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

impl<Field> IndexableRow for Row<Field> {
    type Item = Field;
    fn get(&self, index: usize) -> Option<&Self::Item> {
        self.deref().get(index)
    }
}

#[cfg(test)]
mod tests {
    use crate::VecBackend;

    #[test]
    fn it_works() {
        let mut example = VecBackend::new();

        example.push(vec![1, 2, 3, 4]);
        example.push(vec![1, 2, 3, 4]);
        example.push(vec![1, 2, 3, 4]);
        example.push(vec![1, 2, 3, 4]);
        example.push(vec![1, 2, 3, 4]);
        example.push(vec![1, 2, 3, 4]);

        assert_eq!(example.len(), 6);
    }
}
