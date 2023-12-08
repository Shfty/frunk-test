pub type RwLock<T> = parking_lot::RwLock<T>;
pub type RwLockReadGuard<'a, T> = parking_lot::RwLockReadGuard<'a, T>;
pub type RwLockWriteGuard<'a, T> = parking_lot::RwLockWriteGuard<'a, T>;
