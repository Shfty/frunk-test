use frunk::{HCons, ToRef};

pub trait CollectionGet<'a, K>: ToRef<'a> {
    type Output;

    fn collection_get(&'a self, key: K) -> <Self as CollectionGet<'a, K>>::Output;
}

impl<'a, Head, Tail> CollectionGet<'a, usize> for HCons<Vec<Head>, Tail>
where
    Head: 'a,
    Tail: CollectionGet<'a, usize>,
{
    type Output = HCons<Option<&'a Head>, <Tail as CollectionGet<'a, usize>>::Output>;

    fn collection_get(&'a self, key: usize) -> <Self as CollectionGet<'a, usize>>::Output {
        HCons {
            head: self.head.get(key),
            tail: self.tail.collection_get(key)
        }
    }
}
