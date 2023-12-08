use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

use frunk::{HCons, HNil};

pub trait MapInsert<K, Other> {
    type Output;

    fn insert(self, key: K, other: Other) -> Self::Output;
}

impl<K> MapInsert<K, HNil> for HNil {
    type Output = HNil;
    fn insert(self, _key: K, _other: HNil) -> Self::Output {
        HNil
    }
}

impl<K, T1, H2, T2> MapInsert<K, HCons<H2, T2>> for HCons<&mut &mut BTreeMap<K, H2>, T1>
where
    K: Copy + Ord,
    T1: MapInsert<K, T2>,
{
    type Output = HCons<Option<H2>, T1::Output>;
    fn insert(self, key: K, other: HCons<H2, T2>) -> Self::Output {
        HCons {
            head: self.head.insert(key, other.head),
            tail: self.tail.insert(key, other.tail),
        }
    }
}

impl<K, T1, H2, T2> MapInsert<K, HCons<H2, T2>> for HCons<&mut &mut HashMap<K, H2>, T1>
where
    K: Copy + Eq + Hash,
    T1: MapInsert<K, T2>,
{
    type Output = HCons<Option<H2>, T1::Output>;
    fn insert(self, key: K, other: HCons<H2, T2>) -> Self::Output {
        HCons {
            head: self.head.insert(key, other.head),
            tail: self.tail.insert(key, other.tail),
        }
    }
}
