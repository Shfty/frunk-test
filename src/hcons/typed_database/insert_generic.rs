use frunk::{into_generic, Generic};

use super::InsertHList;

pub trait InsertGeneric<'a, Key, Values> {
    fn insert_generic(
        self,
        key: Key,
        values: Values,
    ) -> <Self as InsertHList<Key, <Values as Generic>::Repr>>::Output
    where
        Self: Sized + InsertHList<Key, <Values as Generic>::Repr>,
        Values: Generic,
    {
        self.insert_hlist(key, into_generic(values))
    }
}

impl<'a, Key, Values, T> InsertGeneric<'a, Key, Values> for T {}
