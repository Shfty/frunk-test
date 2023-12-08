use frunk::{HCons, HNil};

use crate::RwLockReadGuard;

use super::{CollectionContainer, ComponentCollection};

pub trait ReadCollection<'a, Key>: Sized {
    type Output;

    fn read_collection(&self) -> Self::Output;
}

impl<'a, Key, T, Tail> ReadCollection<'a, Key> for HCons<&'a CollectionContainer<Key, T>, Tail>
where
    Tail: ReadCollection<'a, Key>,
{
    type Output = HCons<RwLockReadGuard<'a, ComponentCollection<Key, T>>, Tail::Output>;

    fn read_collection(&self) -> Self::Output {
        let head = self.head;
        let guard: RwLockReadGuard<'a, ComponentCollection<Key, T>> = head.read();
        HCons {
            head: guard,
            tail: self.tail.read_collection(),
        }
    }
}

impl<'a, Key> ReadCollection<'a, Key> for HNil {
    type Output = HNil;
    fn read_collection(&self) -> Self::Output {
        HNil
    }
}
