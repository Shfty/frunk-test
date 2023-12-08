use frunk::{from_generic, Generic};

use super::GetEntityHList;

pub trait GetEntityGeneric<'a, Key, Values> {
    fn get_entity_generic<Struct>(&'a self, key: &Key) -> Struct
    where
        Self: GetEntityHList<'a, Key, Values>,
        Struct: Generic<Repr = <Self as GetEntityHList<'a, Key, Values>>::Output>,
    {
        from_generic(self.get_entity_hlist(key))
    }
}

impl<'a, Key, Values, T> GetEntityGeneric<'a, Key, Values> for T {}
