use frunk::ToRef;

use crate::hcons::{CollectionGet, RwLockRead, Unwrap};

pub trait GetCollectionEntity<Key>: Sized {
    fn get_collection_entity<'a>(
        &'a self,
        key: Key,
    ) -> <<<Self as RwLockRead>::Output as CollectionGet<Key>>::Output as Unwrap>::Output
    where
        Key: 'a + Copy,
        Self: RwLockRead<'a>,
        <Self as RwLockRead<'a>>::Output: CollectionGet<'a, Key>,
        <<Self as RwLockRead<'a>>::Output as CollectionGet<'a, Key>>::Output: Unwrap,
        <<<Self as RwLockRead<'a>>::Output as CollectionGet<'a, Key>>::Output as Unwrap>::Output: ToRef<'a>,
    {
        let collection = self.read();
        let entity = collection.collection_get(key);
        let entity = entity.unwrap();
        let entity_ref = entity.to_ref();
        entity
    }
}

impl<Key, T> GetCollectionEntity<Key> for T {}
