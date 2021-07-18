pub use serde_value;
pub use smallvec;

pub mod storage;
pub mod tab;
pub mod value;

pub use self::tab::Tab;
pub use storage::SmallVecBackend;
pub use storage::VecBackend;
pub use value::Value;

#[cfg(test)]
mod tests {
    use crate::smallvec::smallvec;
    use crate::value::value;
    use crate::Value;
    use crate::{SmallVecBackend, Tab, VecBackend};

    #[test]
    fn vec_backend_works() {
        let mut example =
            Tab::<VecBackend<u8>>::new().add_headers(vec!["one", "two", "three", "four"]);

        example.push(vec![1, 2, 3, 4]);
        example.push(vec![1, 2, 3, 4]);
        example.push(vec![1, 2, 3, 4]);
        example.push(vec![1, 2, 3, 4]);
        example.push(vec![1, 2, 3, 4]);
        example.push(vec![1, 2, 3, 4]);

        assert_eq!(example.len(), 6);
    }

    #[test]
    fn small_vec_backend_works() {
        let mut example =
            Tab::<SmallVecBackend<[u8; 4]>>::new().add_headers(vec!["one", "two", "three", "four"]);

        example.push(smallvec![1, 2, 3, 4]);
        example.push(smallvec![1, 2, 3, 4]);
        example.push(smallvec![1, 2, 3, 4]);
        example.push(smallvec![1, 2, 3, 4]);
        example.push(smallvec![1, 2, 3, 4]);
        example.push(smallvec![1, 2, 3, 4]);

        assert_eq!(example.len(), 6);
    }

    #[test]
    fn indexing() {
        let mut example = Tab::<SmallVecBackend<[Value; 6]>>::new().add_headers(vec![
            "my index", "title", "rating", "flag1", "flag2", "flag3",
        ]);

        example.push(smallvec![
            value(1u8),
            value("testing"),
            value(3.14),
            value(true),
            value(false),
            value(false),
        ]);

        example.push(smallvec![
            value(4u8),
            value("else"),
            value(0.123),
            value(true),
            value(false),
            value(false),
        ]);

        example.push(smallvec![
            value(3u8),
            value("other"),
            value(5.12),
            value(true),
            value(true),
            value(false),
        ]);

        example.add_index("my index").unwrap();
        // original order
        assert_eq!(Some(&value("other")), example.get(2).unwrap().get(1));

        example.sort("my index");
        assert_ne!(Some(&value("other")), example.get(2).unwrap().get(1));
        assert_eq!(Some(&value("else")), example.get(2).unwrap().get(1));
    }
}
