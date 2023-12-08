use frunk::{into_labelled_generic, LabelledGeneric};

use super::InsertHList;

pub trait InsertLabelledGeneric<'a, Key, Values> {
    fn insert_labelled_generic(
        self,
        key: Key,
        other: Values,
    ) -> <Self as InsertHList<Key, <Values as LabelledGeneric>::Repr>>::Output
    where
        Self: Sized + InsertHList<Key, <Values as LabelledGeneric>::Repr>,
        Values: LabelledGeneric,
    {
        let repr = into_labelled_generic(other);
        self.insert_hlist(key, repr)
    }
}

impl<'a, Key, Values, T> InsertLabelledGeneric<'a, Key, Values> for T {}
