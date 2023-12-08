use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

use frunk::{HCons, HNil};

pub trait MapValues<'a> {
    type Output;

    fn values(self) -> Self::Output;
}

impl<'a> MapValues<'a> for HNil {
    type Output = HNil;
    fn values(self) -> Self::Output {
        HNil
    }
}

impl<'a, K, T1, H2> MapValues<'a> for HCons<&'a BTreeMap<K, H2>, T1>
where
    K: 'a + Copy + Ord,
    H2: 'a,
    T1: MapValues<'a>,
{
    type Output = HCons<std::collections::btree_map::Values<'a, K, H2>, T1::Output>;
    fn values(self) -> Self::Output {
        HCons {
            head: self.head.values(),
            tail: self.tail.values(),
        }
    }
}

impl<'a, K, T1, H2> MapValues<'a> for HCons<&'a HashMap<K, H2>, T1>
where
    K: 'a + Eq + Hash,
    H2: 'a,
    T1: MapValues<'a>,
{
    type Output = HCons<std::collections::hash_map::Values<'a, K, H2>, T1::Output>;
    fn values(self) -> Self::Output {
        HCons {
            head: self.head.values(),
            tail: self.tail.values(),
        }
    }
}
