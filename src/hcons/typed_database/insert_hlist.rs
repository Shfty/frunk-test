use frunk::{HCons, HNil};

use super::{CollectionContainer, ComponentContainer};

pub trait InsertHList<Key, Values> {
    type Output;

    fn insert_hlist(&self, key: Key, other: Values) -> Self::Output;
}

impl<Key> InsertHList<Key, HNil> for HNil {
    type Output = HNil;
    fn insert_hlist(&self, _key: Key, _other: HNil) -> Self::Output {
        HNil
    }
}

impl<'a, Key, Tail, ValuesHead, ValuesTail> InsertHList<Key, HCons<ValuesHead, ValuesTail>>
    for HCons<CollectionContainer<Key, ValuesHead>, Tail>
where
    Key: Copy + Ord,
    Tail: InsertHList<Key, ValuesTail>,
{
    type Output = HCons<Option<ComponentContainer<ValuesHead>>, Tail::Output>;
    fn insert_hlist(&self, key: Key, other: HCons<ValuesHead, ValuesTail>) -> Self::Output {
        HCons {
            head: self
                .head
                .write()
                .insert(key, ComponentContainer::new(other.head)),
            tail: self.tail.insert_hlist(key, other.tail),
        }
    }
}

impl<'a, Key, Tail, ValuesHead, ValuesTail> InsertHList<Key, HCons<ValuesHead, ValuesTail>>
    for HCons<&CollectionContainer<Key, ValuesHead>, Tail>
where
    Key: Copy + Ord,
    Tail: InsertHList<Key, ValuesTail>,
{
    type Output = HCons<Option<ComponentContainer<ValuesHead>>, Tail::Output>;
    fn insert_hlist(&self, key: Key, other: HCons<ValuesHead, ValuesTail>) -> Self::Output {
        HCons {
            head: self
                .head
                .write()
                .insert(key, ComponentContainer::new(other.head)),
            tail: self.tail.insert_hlist(key, other.tail),
        }
    }
}
