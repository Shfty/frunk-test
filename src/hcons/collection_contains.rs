use std::{
    collections::{BTreeSet, HashSet},
    hash::Hash,
};

use frunk::{HCons, HNil};

pub trait CollectionContains<'a, K> {
    type Output;

    fn contains(&'a self, index: K) -> Self::Output;
}

impl<'a, K> CollectionContains<'a, K> for HNil {
    type Output = HNil;

    #[inline(always)]
    fn contains(&'a self, _: K) -> Self::Output {
        HNil
    }
}

impl<'a, Tail, K> CollectionContains<'a, K> for HCons<&BTreeSet<K>, Tail>
where
    Tail: CollectionContains<'a, K>,
    K: Ord,
{
    type Output = HCons<bool, <Tail as CollectionContains<'a, K>>::Output>;

    #[inline(always)]
    fn contains(&'a self, index: K) -> Self::Output {
        HCons {
            head: self.head.contains(&index),
            tail: (&self.tail).contains(index),
        }
    }
}

impl<'a, Tail, K> CollectionContains<'a, K> for HCons<&HashSet<K>, Tail>
where
    Tail: CollectionContains<'a, K>,
    K: Eq + Hash,
{
    type Output = HCons<bool, <Tail as CollectionContains<'a, K>>::Output>;

    #[inline(always)]
    fn contains(&'a self, index: K) -> Self::Output {
        HCons {
            head: self.head.contains(&index),
            tail: (&self.tail).contains(index),
        }
    }
}
