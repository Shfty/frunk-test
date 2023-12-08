use frunk::{HCons, HNil};

use super::{CollectionContainer, ComponentContainer};

pub trait RemoveKey<Key, Values> {
    type Output;

    fn remove_key(&self, key: &Key) -> Self::Output;
}

impl<Key> RemoveKey<Key, HNil> for HNil {
    type Output = HNil;
    fn remove_key(&self, _key: &Key) -> Self::Output {
        HNil
    }
}

impl<'a, Key, Tail, ValuesHead, ValuesTail> RemoveKey<Key, HCons<ValuesHead, ValuesTail>>
    for HCons<CollectionContainer<Key, ValuesHead>, Tail>
where
    Key: Ord,
    Tail: RemoveKey<Key, ValuesTail>,
{
    type Output = HCons<Option<ComponentContainer<ValuesHead>>, Tail::Output>;
    fn remove_key(&self, key: &Key) -> Self::Output {
        HCons {
            head: self.head.write().remove(key),
            tail: self.tail.remove_key(key),
        }
    }
}

impl<'a, Key, Tail, ValuesHead, ValuesTail> RemoveKey<Key, HCons<ValuesHead, ValuesTail>>
    for HCons<&CollectionContainer<Key, ValuesHead>, Tail>
where
    Key: Ord,
    Tail: RemoveKey<Key, ValuesTail>,
{
    type Output = HCons<Option<ComponentContainer<ValuesHead>>, Tail::Output>;
    fn remove_key(&self, key: &Key) -> Self::Output {
        HCons {
            head: self.head.write().remove(key),
            tail: self.tail.remove_key(key),
        }
    }
}
