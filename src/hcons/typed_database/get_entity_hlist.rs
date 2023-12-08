use frunk::{HCons, HNil};

use super::CollectionContainer;

pub trait GetEntityHList<'a, Key, Values> {
    type Output;

    fn get_entity_hlist(&'a self, key: &Key) -> Self::Output;
}

impl<'a, Key> GetEntityHList<'a, Key, HNil> for HNil {
    type Output = HNil;
    fn get_entity_hlist(&'a self, _key: &Key) -> Self::Output {
        HNil
    }
}

impl<'a, Key, Tail, ValuesHead, ValuesTail>
    GetEntityHList<'a, Key, HCons<ValuesHead, ValuesTail>>
    for HCons<CollectionContainer<Key, ValuesHead>, Tail>
where
    Key: Ord,
    Tail: GetEntityHList<'a, Key, ValuesTail>,
    ValuesHead: 'a + Copy,
{
    type Output = HCons<ValuesHead, Tail::Output>;

    fn get_entity_hlist(&'a self, key: &Key) -> Self::Output {
        HCons {
            head: *self.head.read()[key].read(),
            tail: self.tail.get_entity_hlist(key),
        }
    }
}

impl<'a, Key, Tail, ValuesHead, ValuesTail>
    GetEntityHList<'a, Key, HCons<ValuesHead, ValuesTail>>
    for HCons<&CollectionContainer<Key, ValuesHead>, Tail>
where
    Key: Ord,
    Tail: GetEntityHList<'a, Key, ValuesTail>,
    ValuesHead: 'a + Copy,
{
    type Output = HCons<ValuesHead, Tail::Output>;

    fn get_entity_hlist(&'a self, key: &Key) -> Self::Output {
        HCons {
            head: *self.head.read()[key].read(),
            tail: self.tail.get_entity_hlist(key),
        }
    }
}
