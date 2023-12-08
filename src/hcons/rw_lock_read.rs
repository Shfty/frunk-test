use crate::{RwLock, RwLockReadGuard};

use frunk::{HCons, HNil, ToRef};

pub trait RwLockRead<'a>: ToRef<'a> {
    type Output;

    fn read(&'a self) -> <Self as RwLockRead<'a>>::Output;
}

impl<'a, Head, Tail> RwLockRead<'a> for HCons<RwLock<Head>, Tail>
where
    Head: 'a,
    Tail: RwLockRead<'a>,
{
    type Output = HCons<RwLockReadGuard<'a, Head>, <Tail as RwLockRead<'a>>::Output>;

    fn read(&'a self) -> <Self as RwLockRead<'a>>::Output {
        HCons {
            head: self.head.read(),
            tail: self.tail.read(),
        }
    }
}

impl<'a> RwLockRead<'a> for HNil {
    type Output = HNil;

    fn read(&'a self) -> <Self as RwLockRead<'a>>::Output {
        todo!()
    }
}
