use std::ops::Deref;

use frunk::{HCons, HNil};

pub trait ToDeref {
    type Output;

    fn to_deref(self) -> Self::Output;
}

impl ToDeref for HNil {
    type Output = HNil;

    #[inline(always)]
    fn to_deref(self) -> Self::Output {
        HNil
    }
}

impl<'a, H, Tail> ToDeref for HCons<&'a H, Tail>
where
    H: Deref,
    Tail: ToDeref,
{
    type Output = HCons<&'a <H as Deref>::Target, <Tail as ToDeref>::Output>;

    #[inline(always)]
    fn to_deref(self) -> Self::Output {
        HCons {
            head: self.head.deref(),
            tail: self.tail.to_deref(),
        }
    }
}
