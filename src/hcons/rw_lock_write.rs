use crate::{RwLock, RwLockWriteGuard};

use frunk::{hlist::HMappable, Func, Poly};

pub trait RwLockWrite {
    type Output;

    fn write(self) -> Self::Output;
}

impl<T> RwLockWrite for T
where
    T: HMappable<Poly<RwLockWritePoly>>,
{
    type Output = <Self as HMappable<Poly<RwLockWritePoly>>>::Output;

    fn write(self) -> Self::Output {
        self.map(RwLockWritePoly::mapper())
    }
}

#[derive(Default, Copy, Clone)]
pub struct RwLockWritePoly;

impl RwLockWritePoly {
    pub fn mapper() -> Poly<Self> {
        Poly(Default::default())
    }
}

impl<'a, T> Func<&'a RwLock<T>> for RwLockWritePoly
where
    T: 'a,
{
    type Output = RwLockWriteGuard<'a, T>;

    fn call(i: &'a RwLock<T>) -> Self::Output {
        i.write()
    }
}
