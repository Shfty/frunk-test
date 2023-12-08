use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
    marker::PhantomData,
};

use frunk::{Func, Poly};

/// Polymorphic function for map-folding `Hlist![BTreeMap<Key, T>, ...]` into `BTreeSet<Key>`
/// # Examples
/// ```
/// let mut list: Hlist![
///     BTreeMap<usize, bool>,
///     BTreeMap<usize, i32>,
///     BTreeMap<usize, f32>,
///     BTreeMap<usize, &'static str>
/// ] = Default::default();
///
/// let a: &mut BTreeMap<usize, bool> = list.get_mut();
/// a.insert(0, false);
/// a.insert(1, true);
///
/// let b: &mut BTreeMap<usize, i32> = list.get_mut();
/// b.insert(0, 0);
/// b.insert(1, 1);
/// b.insert(2, 2);
/// b.insert(3, 4);
///
/// let c: &mut BTreeMap<usize, f32> = list.get_mut();
/// c.insert(0, 0.0);
/// c.insert(1, 0.2);
/// c.insert(2, 0.4);
/// c.insert(3, 0.6);
/// c.insert(4, 0.8);
/// c.insert(5, 1.0);

/// let d: &mut BTreeMap<usize, &'static str> = list.get_mut();
/// d.insert(0, "foo");
/// d.insert(1, "bar");
/// d.insert(2, "baz");
/// d.insert(3, "decafisbad");
///
/// // Mapper homogenizes the list into Hlist![BTreeSet<Key>, ...]
/// let key_list = list.to_ref().map(FoldBTreeKeys::mapper());
///
/// // Folder performs a union over the BTreeSets
/// let keys = key_list.foldl(FoldBTreeKeys::folder, Default::default());
///
/// assert_eq!(keys, [0, 1, 2, 3, 4, 5].iter().copied().collect::<BTreeSet<Key>>());
/// ```
#[derive(Default)]
pub struct FoldBTreeKeys<Key>(PhantomData<Key>);

#[allow(dead_code)]
impl<Key> FoldBTreeKeys<Key>
where
    Key: Default + Ord,
{
    pub fn mapper() -> Poly<Self> {
        Default::default()
    }

    pub fn folder(mut acc: BTreeSet<Key>, mut f: BTreeSet<Key>) -> BTreeSet<Key> {
        acc.append(&mut f);
        acc
    }
}

impl<'a, Key, Value> Func<&'a BTreeMap<Key, Value>> for FoldBTreeKeys<Key>
where
    Key: Copy + Ord,
    Value: 'a + Debug,
{
    type Output = BTreeSet<Key>;

    fn call(i: &'a BTreeMap<Key, Value>) -> Self::Output {
        i.keys().copied().collect()
    }
}
