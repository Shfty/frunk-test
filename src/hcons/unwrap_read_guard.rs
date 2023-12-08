use frunk::{HCons, HNil};
use parking_lot::MappedRwLockReadGuard;

use std::borrow::Borrow;

use crate::RwLockReadGuard;

pub trait UnwrapReadGuard {
    type Output;

    fn unwrap_read_guard(self) -> Self::Output;
}

impl UnwrapReadGuard for HNil {
    type Output = HNil;

    #[inline(always)]
    fn unwrap_read_guard<'b>(self) -> Self::Output {
        HNil
    }
}

impl<'a, H, Tail> UnwrapReadGuard for HCons<&'a RwLockReadGuard<'a, H>, Tail>
where
    H: 'a,
    Tail: UnwrapReadGuard,
{
    type Output = HCons<&'a H, <Tail as UnwrapReadGuard>::Output>;

    #[inline(always)]
    fn unwrap_read_guard(self) -> Self::Output  {
        HCons {
            head: self.head.borrow(),
            tail: self.tail.unwrap_read_guard(),
        }
    }
}


impl<'a, H, Tail> UnwrapReadGuard for HCons<&'a MappedRwLockReadGuard<'a, H>, Tail>
where
    H: 'a,
    Tail: UnwrapReadGuard,
{
    type Output = HCons<&'a H, <Tail as UnwrapReadGuard>::Output>;

    #[inline(always)]
    fn unwrap_read_guard(self) -> Self::Output {
        HCons {
            head: self.head.borrow(),
            tail: self.tail.unwrap_read_guard(),
        }
    }
}
