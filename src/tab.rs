use crate::storage::IndexableRow;
use crate::storage::Storage;

use derive_more::{Deref, DerefMut};
use ordered_multimap::list_ordered_multimap::ListOrderedMultimap;
use std::collections::{BTreeMap, HashMap, VecDeque};

#[derive(Deref, DerefMut)]
pub struct Tab<D>
where
    D: Storage,
    D::Item: std::cmp::Eq + std::hash::Hash,
{
    #[deref]
    #[deref_mut]
    data: D,
    pub headers: HashMap<String, usize>,
    pub indexes: BTreeMap<usize, ListOrderedMultimap<D::Item, usize>>,
    initialized: bool,
}

impl<D: Storage> std::fmt::Debug for Tab<D>
where
    D: std::fmt::Debug,
    D::Item: std::fmt::Debug + std::cmp::Eq + std::hash::Hash,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tab")
            .field("data", &self.data)
            .field("headers", &self.headers)
            .field("indexes", &self.indexes)
            .field("initialized", &self.initialized)
            .finish()
    }
}

impl<D> Tab<D>
where
    D: Storage,
    D::Item: std::cmp::Eq + std::cmp::PartialOrd + std::hash::Hash + Clone,
    D::Row: Clone + std::fmt::Debug,
    <<D as Storage>::Row as IndexableRow>::Item: std::fmt::Debug,
{
    pub fn new() -> Tab<D> {
        Tab {
            data: D::new(),
            headers: HashMap::new(),
            indexes: BTreeMap::new(),
            initialized: false,
        }
    }

    pub fn add_headers<S: Into<String>, H: IntoIterator<Item = S>>(mut self, headers: H) -> Self {
        self.headers = headers
            .into_iter()
            .enumerate()
            .map(|(i, val)| (val.into(), i))
            .collect();
        self.initialized = true;
        self
    }

    pub fn add_index<S: AsRef<str>>(&mut self, header: S) -> Result<(), String> {
        if let Some(key) = self.headers.get(header.as_ref()) {
            self.indexes.insert(*key, ListOrderedMultimap::new());
            self.recalculate_indexes();
            Ok(())
        } else {
            Err(String::new())
        }
    }

    pub fn rows_equal<S: AsRef<str>>(
        &mut self,
        header: S,
        item: &D::Item,
    ) -> Option<RowsIter<'_, D>> {
        if let Some(column) = self.headers.get(header.as_ref()) {
            let map = &self.indexes[column];
            let rows_indexes: VecDeque<usize> = map.get_all(item).map(|x| *x).collect();
            Some(RowsIter {
                data: &self.data,
                indexes: rows_indexes,
            })
        } else {
            None
        }
    }

    pub fn is_index<S: AsRef<str>>(&mut self, header: S) -> bool {
        if let Some(column) = self.headers.get(header.as_ref()) {
            if self.indexes.get(column).is_some() {
                return true;
            }
        }

        false
    }

    pub fn recalculate_indexes(&mut self) -> () {
        self.flush_indexes();
        for (index, map) in self.indexes.iter_mut() {
            let mut tmp_list = Vec::new();
            for (i, field) in self.data.columns().enumerate() {
                if let Some(x) = field.get(*index) {
                    tmp_list.push((x.clone(), i));
                }
            }
            tmp_list.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
            map.extend(tmp_list);
        }
    }

    pub fn flush_indexes(&mut self) -> () {
        for index_item in self.indexes.values_mut() {
            index_item.clear();
        }
    }

    pub fn sort<S: AsRef<str>>(&mut self, header: S) -> () {
        if self.is_index(&header) {
            return self.index_sort(header);
        } else {
            return self.unindex_sort(header);
        }
    }

    /// sorts the data based on index number
    /// Should technically faster then unindex_sort because the sorting on values is done while creating the indexes
    pub fn index_sort<S: AsRef<str>>(&mut self, header: S) -> () {
        let mut new_order = None;
        if let Some(column) = self.headers.get(header.as_ref()) {
            if let Some(index) = self.indexes.get(column) {
                let tmp: Vec<(_, _)> = index.iter().collect();

                let mut tmp = tmp
                    .into_iter()
                    .map(|(_, row)| row)
                    .enumerate()
                    .collect::<Vec<_>>();

                // invert the row indexes
                tmp.sort_unstable_by(|(_, a), (_, b)| a.cmp(b));
                new_order = Some(tmp.into_iter().map(|(a, _)| a).collect::<Vec<_>>());
            }
        }

        if let Some(new_order) = new_order {
            let mut tmp: Vec<_> = new_order
                .into_iter()
                .zip(self.data.to_owned_vec())
                .collect();
            tmp.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
            self.data = D::from_iter(tmp.into_iter().map(|x| x.1));
            self.recalculate_indexes();
        }
    }

    pub fn unindex_sort<S: AsRef<str>>(&mut self, header: S) -> () {
        if let Some(column) = self.headers.get(header.as_ref()) {
            let mut tmp = self.data.to_owned_vec();
            tmp.sort_by(|a, b| a.get(*column).partial_cmp(&b.get(*column)).unwrap());
            self.data = D::from_iter(tmp.into_iter())
        }
    }
}


pub struct RowsIter<'a, D> {
    data: &'a D,
    indexes: VecDeque<usize>,
}

impl<'a, D: Storage> Iterator for RowsIter<'a, D> {
    type Item = &'a D::Row;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self.indexes.pop_front() {
            self.data.get(index)
        } else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{Tab, VecBackend};
    use rand::seq::SliceRandom;

    #[test]
    fn add_index() {
        let mut example =
            Tab::<VecBackend<u8>>::new().add_headers(vec!["one", "two", "three", "four"]);

        example.push(vec![1, 2, 3, 4]);
        example.push(vec![9, 10, 11, 12]);
        example.push(vec![5, 6, 7, 8]);
        example.push(vec![1, 2, 3, 4]);

        example.add_index("two").unwrap();
        example.add_index(String::from("three")).unwrap();

        // example.sort("three");
        example.sort("one");
        // println!("{:?}", example);
        // assert!(example.len() == 123);
    }

    #[test]
    fn sorting_indexed() {
        use rand::SeedableRng;

        let mut rng = rand::rngs::StdRng::seed_from_u64(123435);
        let range: Vec<_> = (0..1000).into_iter().collect();

        let mut tmp = Vec::new();

        for line in range.chunks(25) {
            tmp.push(line.to_vec())
        }

        tmp.shuffle(&mut rng);

        let headers: Vec<_> = (0..25).map(|x| x.to_string()).collect();
        let mut example = Tab::<VecBackend<u32>>::new().add_headers(headers);

        for line in tmp {
            example.push(line);
        }

        example.add_index("3").unwrap();
        example.sort("3");

        // check if it is sorted
        assert_eq!(
            example.get(0),
            Some(&vec![
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24
            ])
        );
        assert_eq!(
            example.get(1),
            Some(&vec![
                25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
                46, 47, 48, 49
            ])
        );
        assert_eq!(
            example.get(39),
            Some(&vec![
                975, 976, 977, 978, 979, 980, 981, 982, 983, 984, 985, 986, 987, 988, 989, 990,
                991, 992, 993, 994, 995, 996, 997, 998, 999
            ])
        );
    }

    #[test]
    fn sorting_unindexed() {
        use rand::SeedableRng;

        let mut rng = rand::rngs::StdRng::seed_from_u64(123435);
        let range: Vec<_> = (0..1000).into_iter().collect();

        let mut tmp = Vec::new();

        for line in range.chunks(25) {
            tmp.push(line.to_vec())
        }

        tmp.shuffle(&mut rng);

        let headers: Vec<_> = (0..25).map(|x| x.to_string()).collect();
        let mut example = Tab::<VecBackend<u32>>::new().add_headers(headers);

        for line in tmp {
            example.push(line);
        }

        example.sort("3");

        // check if it is sorted
        assert_eq!(
            example.get(0),
            Some(&vec![
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24
            ])
        );
        assert_eq!(
            example.get(1),
            Some(&vec![
                25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
                46, 47, 48, 49
            ])
        );
        assert_eq!(
            example.get(39),
            Some(&vec![
                975, 976, 977, 978, 979, 980, 981, 982, 983, 984, 985, 986, 987, 988, 989, 990,
                991, 992, 993, 994, 995, 996, 997, 998, 999
            ])
        );
    }

    #[test]
    fn rows_equal() {
        let mut example =
            Tab::<VecBackend<u8>>::new().add_headers(vec!["one", "two", "three", "four"]);

        example.push(vec![1, 5, 3, 4]);
        example.push(vec![3, 5, 3, 4]);
        example.push(vec![5, 6, 7, 8]);
        example.push(vec![1, 4, 3, 4]);

        example.add_index("two").unwrap();

        let rows: Vec<_> = example.rows_equal("two", &5).unwrap().collect();

        assert_eq!(rows.len(), 2);
    }
}
