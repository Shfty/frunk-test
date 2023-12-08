use frunk::{from_labelled_generic, LabelledGeneric};

use super::GetEntityHList;

pub trait GetEntityLabelledGeneric<'a, Key, Values> {
    fn get_entity_labelled_generic<Struct>(&'a self, key: &Key) -> Struct
    where
        Self: GetEntityHList<'a, Key, Values>,
        Struct: LabelledGeneric<Repr = <Self as GetEntityHList<'a, Key, Values>>::Output>,
    {
        from_labelled_generic(self.get_entity_hlist(key))
    }
}

impl<'a, Key, Values, T> GetEntityLabelledGeneric<'a, Key, Values> for T {}
