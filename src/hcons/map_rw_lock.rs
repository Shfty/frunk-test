use crate::hcons::RwLockRead;
use crate::hcons::RwLockWrite;

pub trait MapRwLockRead<'a>: Sized + RwLockRead<'a> {
    fn map_read<F, R>(&'a self, f: F) -> R
    where
        F: Fn(<Self as RwLockRead>::Output) -> R,
    {
        f(self.read())
    }
}

impl<'a, T> MapRwLockRead<'a> for T where T: RwLockRead<'a> {}

pub trait MapRwLockWrite: Sized + RwLockWrite {
    fn map_write<F, R>(self, f: F) -> R
    where
        F: Fn(<Self as RwLockWrite>::Output) -> R,
    {
        f(self.write())
    }
}

impl<T> MapRwLockWrite for T where T: RwLockWrite {}
