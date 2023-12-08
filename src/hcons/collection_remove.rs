use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    hash::Hash,
};

use frunk::{HCons, HNil};

pub trait CollectionRemove<K, Other> {
    type Output;

    fn remove(self, key: &K) -> Self::Output;
}

impl<K> CollectionRemove<K, HNil> for HNil {
    type Output = HNil;
    fn remove(self, _key: &K) -> Self::Output {
        HNil
    }
}

impl<K, T1, H2, T2> CollectionRemove<K, HCons<H2, T2>> for HCons<&mut &mut BTreeMap<K, H2>, T1>
where
    K: Copy + Ord,
    T1: CollectionRemove<K, T2>,
{
    type Output = HCons<Option<H2>, T1::Output>;
    fn remove(self, key: &K) -> Self::Output {
        HCons {
            head: self.head.remove(key),
            tail: self.tail.remove(key),
        }
    }
}

impl<K, T1, H2, T2> CollectionRemove<K, HCons<H2, T2>> for HCons<&mut &mut HashMap<K, H2>, T1>
where
    K: Copy + Eq + Hash,
    T1: CollectionRemove<K, T2>,
{
    type Output = HCons<Option<H2>, T1::Output>;
    fn remove(self, key: &K) -> Self::Output {
        HCons {
            head: self.head.remove(key),
            tail: self.tail.remove(key),
        }
    }
}

impl<K, T1, H2, T2> CollectionRemove<K, HCons<H2, T2>> for HCons<&mut &mut BTreeSet<K>, T1>
where
    K: Copy + Ord,
    T1: CollectionRemove<K, T2>,
{
    type Output = HCons<bool, T1::Output>;
    fn remove(self, key: &K) -> Self::Output {
        HCons {
            head: self.head.remove(key),
            tail: self.tail.remove(key),
        }
    }
}

impl<K, T1, H2, T2> CollectionRemove<K, HCons<H2, T2>> for HCons<&mut &mut HashSet<K>, T1>
where
    K: Copy + Eq + Hash,
    T1: CollectionRemove<K, T2>,
{
    type Output = HCons<bool, T1::Output>;
    fn remove(self, key: &K) -> Self::Output {
        HCons {
            head: self.head.remove(key),
            tail: self.tail.remove(key),
        }
    }
}
