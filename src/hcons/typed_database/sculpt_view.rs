use frunk::{hlist::Sculptor, Generic};

use super::CollectionContainerSignature;

pub trait SculptView<Key, Index>: Sized {
    fn sculpt_view<'a, Values>(
        self,
    ) -> <<Values as Generic>::Repr as CollectionContainerSignature<'a, Key>>::Signature
    where
        Self: Sculptor<
            <<Values as Generic>::Repr as CollectionContainerSignature<'a, Key>>::Signature,
            Index,
        >,
        Values: Generic,
        <Values as Generic>::Repr: CollectionContainerSignature<'a, Key>,
    {
        self.sculpt().0
    }
}

impl<'a, Key, Index, T> SculptView<Key, Index> for T {}
