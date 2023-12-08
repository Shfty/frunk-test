use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

use frunk::{HCons, HNil};

pub trait MapKeys<'a> {
    type Output;

    fn keys(self) -> Self::Output;
}

impl<'a> MapKeys<'a> for HNil {
    type Output = HNil;
    fn keys(self) -> Self::Output {
        HNil
    }
}

impl<'a, K, T1, H2> MapKeys<'a> for HCons<&'a BTreeMap<K, H2>, T1>
where
    K: 'a + Copy + Ord,
    H2: 'a,
    T1: MapKeys<'a>,
{
    type Output = HCons<std::collections::btree_map::Keys<'a, K, H2>, T1::Output>;
    fn keys(self) -> Self::Output {
        HCons {
            head: self.head.keys(),
            tail: self.tail.keys(),
        }
    }
}

impl<'a, K, T1, H2> MapKeys<'a> for HCons<&'a HashMap<K, H2>, T1>
where
    K: 'a + Eq + Hash,
    H2: 'a,
    T1: MapKeys<'a>,
{
    type Output = HCons<std::collections::hash_map::Keys<'a, K, H2>, T1::Output>;
    fn keys(self) -> Self::Output {
        HCons {
            head: self.head.keys(),
            tail: self.tail.keys(),
        }
    }
}
