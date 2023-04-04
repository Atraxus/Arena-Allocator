use std::marker::PhantomData;

struct InvariantLifetime<'id>(PhantomData<*mut &'id ()>);

struct ArenaIndex<'id> { 
    idx: usize,
    _marker: InvariantLifetime<'id> 
}

pub struct Arena<'id, T> {
    data: Vec<T>,
    _marker: InvariantLifetime<'id>,
}

// impl Arena<'id, T> {
//     pub fn new<R>(inner:)
// }