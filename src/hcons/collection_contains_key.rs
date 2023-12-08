use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

use frunk::{HCons, HNil};

pub trait CollectionContainsKey<'a, K> {
    type Output;

    fn contains_key(&'a self, index: K) -> Self::Output;
}

impl<'a, K> CollectionContainsKey<'a, K> for HNil {
    type Output = HNil;

    #[inline(always)]
    fn contains_key(&'a self, _: K) -> Self::Output {
        HNil
    }
}

impl<'a, Tail, K, V> CollectionContainsKey<'a, K> for HCons<&BTreeMap<K, V>, Tail>
where
    Tail: CollectionContainsKey<'a, K>,
    K: Ord,
    V: 'a,
{
    type Output = HCons<bool, <Tail as CollectionContainsKey<'a, K>>::Output>;

    #[inline(always)]
    fn contains_key(&'a self, index: K) -> Self::Output {
        HCons {
            head: self.head.contains_key(&index),
            tail: (&self.tail).contains_key(index),
        }
    }
}

impl<'a, Tail, K, V> CollectionContainsKey<'a, K> for HCons<&HashMap<K, V>, Tail>
where
    Tail: CollectionContainsKey<'a, K>,
    K: Eq + Hash,
    V: 'a,
{
    type Output = HCons<bool, <Tail as CollectionContainsKey<'a, K>>::Output>;

    #[inline(always)]
    fn contains_key(&'a self, index: K) -> Self::Output {
        HCons {
            head: self.head.contains_key(&index),
            tail: (&self.tail).contains_key(index),
        }
    }
}