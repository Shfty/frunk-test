use std::{
    collections::{BTreeSet, HashSet},
    hash::Hash,
};

use frunk::{HCons, HNil};

pub trait SetInsert<K> {
    type Output;

    fn insert(self, key: K) -> Self::Output;
}

impl<K> SetInsert<K> for HNil {
    type Output = HNil;
    fn insert(self, _key: K) -> Self::Output {
        HNil
    }
}

impl<K, T1> SetInsert<K> for HCons<&mut &mut BTreeSet<K>, T1>
where
    K: Copy + Ord,
    T1: SetInsert<K>,
{
    type Output = HCons<bool, T1::Output>;
    fn insert(self, key: K) -> Self::Output {
        HCons {
            head: self.head.insert(key),
            tail: self.tail.insert(key),
        }
    }
}

impl<K, T1> SetInsert<K> for HCons<&mut &mut HashSet<K>, T1>
where
    K: Copy + Eq + Hash,
    T1: SetInsert<K>,
{
    type Output = HCons<bool, T1::Output>;
    fn insert(self, key: K) -> Self::Output {
        HCons {
            head: self.head.insert(key),
            tail: self.tail.insert(key),
        }
    }
}
