use std::marker::PhantomData;

use cranelift_entity::EntityRef;

/// Same as Box<[T]>, also explicitly names the key type. In this sense it's
/// similar to [cranelift_entity::PrimaryMap]
pub struct KArray<K: EntityRef, T>(pub Box<[T]>, PhantomData<K>);

impl<K: EntityRef, T> KArray<K, T> {
    pub const fn new(array: Box<[T]>) -> Self {
        KArray(array, PhantomData)
    }
}

// pub struct KVec<K: EntityRef, T>(pub Vec<T>, PhantomData<K>);
