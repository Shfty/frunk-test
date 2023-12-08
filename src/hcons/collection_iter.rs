use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    hash::Hash,
};

use frunk::{HCons, HNil};

pub trait SetIter<'a> {
    type Output;

    fn iter(self) -> Self::Output;
}

impl<'a> SetIter<'a> for HNil {
    type Output = HNil;
    fn iter(self) -> Self::Output {
        HNil
    }
}

impl<'a, T, T1> SetIter<'a> for HCons<&'a Vec<T>, T1>
where
    T: 'a,
    T1: SetIter<'a>,
{
    type Output = HCons<std::slice::Iter<'a, T>, T1::Output>;
    fn iter(self) -> Self::Output {
        HCons {
            head: self.head.iter(),
            tail: self.tail.iter(),
        }
    }
}

impl<'a, T, T1> SetIter<'a> for HCons<&'a VecDeque<T>, T1>
where
    T: 'a,
    T1: SetIter<'a>,
{
    type Output = HCons<std::collections::vec_deque::Iter<'a, T>, T1::Output>;
    fn iter(self) -> Self::Output {
        HCons {
            head: self.head.iter(),
            tail: self.tail.iter(),
        }
    }
}

impl<'a, K, V, T1> SetIter<'a> for HCons<&'a BTreeMap<K, V>, T1>
where
    K: 'a + Ord,
    V: 'a,
    T1: SetIter<'a>,
{
    type Output = HCons<std::collections::btree_map::Iter<'a, K, V>, T1::Output>;
    fn iter(self) -> Self::Output {
        HCons {
            head: self.head.iter(),
            tail: self.tail.iter(),
        }
    }
}

impl<'a, K, V, T1> SetIter<'a> for HCons<&'a HashMap<K, V>, T1>
where
    K: 'a + Eq + Hash,
    V: 'a,
    T1: SetIter<'a>,
{
    type Output = HCons<std::collections::hash_map::Iter<'a, K, V>, T1::Output>;
    fn iter(self) -> Self::Output {
        HCons {
            head: self.head.iter(),
            tail: self.tail.iter(),
        }
    }
}

impl<'a, K, T1> SetIter<'a> for HCons<&'a BTreeSet<K>, T1>
where
    K: 'a + Copy + Ord,
    T1: SetIter<'a>,
{
    type Output = HCons<std::collections::btree_set::Iter<'a, K>, T1::Output>;
    fn iter(self) -> Self::Output {
        HCons {
            head: self.head.iter(),
            tail: self.tail.iter(),
        }
    }
}

impl<'a, K, T1> SetIter<'a> for HCons<&'a HashSet<K>, T1>
where
    K: 'a + Ord,
    T1: SetIter<'a>,
{
    type Output = HCons<std::collections::hash_set::Iter<'a, K>, T1::Output>;
    fn iter(self) -> Self::Output {
        HCons {
            head: self.head.iter(),
            tail: self.tail.iter(),
        }
    }
}
