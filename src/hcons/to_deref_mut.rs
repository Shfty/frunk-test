use std::ops::{Deref, DerefMut};

use frunk::{HCons, HNil};

pub trait ToDerefMut<'a> {
    type Output;

    fn deref_mut(&'a mut self) -> Self::Output;
}

impl<'a> ToDerefMut<'a> for HNil {
    type Output = HNil;

    #[inline(always)]
    fn deref_mut(&'a mut self) -> Self::Output {
        HNil
    }
}

impl<'a, H, Tail> ToDerefMut<'a> for HCons<&mut H, Tail>
where
    H: 'a + DerefMut,
    Tail: ToDerefMut<'a>,
{
    type Output = HCons<&'a mut <H as Deref>::Target, <Tail as ToDerefMut<'a>>::Output>;

    #[inline(always)]
    fn deref_mut(&'a mut self) -> Self::Output {
        HCons {
            head: DerefMut::deref_mut(&mut self.head),
            tail: (&mut self.tail).deref_mut(),
        }
    }
}
