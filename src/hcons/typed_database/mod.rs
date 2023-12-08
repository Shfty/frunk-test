mod get_entity_generic;
mod get_entity_hlist;
mod get_entity_labelled_generic;
mod insert_generic;
mod insert_hlist;
mod insert_labelled_generic;
mod remove_key;
mod sculpt_view;
mod signature;
mod read_collection;
mod get_collection_entity;

pub use get_entity_generic::*;
pub use get_entity_hlist::*;
pub use get_entity_labelled_generic::*;
pub use insert_generic::*;
pub use insert_hlist::*;
pub use insert_labelled_generic::*;
pub use remove_key::*;
pub use sculpt_view::*;
pub use signature::*;
pub use read_collection::*;
pub use get_collection_entity::*;

use std::{collections::BTreeMap, sync::Arc};

use crate::RwLock;

pub type ComponentContainer<T> = RwLock<T>;
pub type ComponentMap<K, V> = BTreeMap<K, ComponentContainer<V>>;
pub type ComponentCollection<K, V> = ComponentMap<K, V>;
pub type CollectionContainer<K, V> = RwLock<ComponentCollection<K, V>>;
pub type ComponentDatabase<K, V> = Arc<CollectionContainer<K, V>>;
