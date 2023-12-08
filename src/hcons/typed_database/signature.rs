use frunk::{HCons, HNil};

use super::{CollectionContainer, ComponentContainer};

/// Type mapping from `Hlist![T]` -> `Hlist![&CollectionContainer<T>]`
pub trait CollectionContainerSignature<'a, K> {
    type Signature;
}

impl<'a, K, V, Tail> CollectionContainerSignature<'a, K> for HCons<V, Tail>
where
    K: 'a,
    V: 'a,
    Tail: CollectionContainerSignature<'a, K>,
{
    type Signature =
        HCons<&'a CollectionContainer<K, V>, <Tail as CollectionContainerSignature<'a, K>>::Signature>;
}

impl<'a, K> CollectionContainerSignature<'a, K> for HNil {
    type Signature = HNil;
}

/// Type mapping from `Hlist![T]` -> `Hlist![&ComponentContainer<T>]`
pub trait ComponentContainerSignature<'a> {
    type Signature;
}

impl<'a, V, Tail> ComponentContainerSignature<'a> for HCons<V, Tail>
where
    V: 'a,
    Tail: ComponentContainerSignature<'a>,
{
    type Signature = HCons<&'a ComponentContainer<V>, <Tail as ComponentContainerSignature<'a>>::Signature>;
}

impl<'a> ComponentContainerSignature<'a> for HNil {
    type Signature = HNil;
}
