use std::marker::PhantomData;

struct InvariantLifetime<'id>(PhantomData<*mut &'id ()>);

pub struct ArenaIndex<'id> { 
    idx: usize,
    _marker: InvariantLifetime<'id> 
}

pub struct Arena<'id, T> {
    data: Vec<T>,
    _marker: InvariantLifetime<'id>,
}

impl<'id, T>  Arena<'id, T> {
    pub fn new<R>(f: impl for<'a> FnOnce(Arena<'a, T>) -> R) -> R {
        let inner: Vec<T> = Vec::new();
        let arena = Arena { data: inner, _marker: InvariantLifetime(PhantomData) };
        f(arena)
    }

    pub fn alloc(&mut self, val: T) -> usize {
        let idx = self.data.len();
        self.data.push(val);
        idx
    }

    pub fn get_index(&mut self, idx: usize) -> Option<ArenaIndex<'id>> {
        if idx < self.data.len() {
            Some(ArenaIndex { idx, _marker: InvariantLifetime(PhantomData) })
        } 
        else {
            None
        }
    }

    pub fn get(&self, val: usize) -> &T {
        &self.data[val]
    }

    pub fn get_mut(&mut self, val: usize) -> &mut T {
        &mut self.data[val]
    }

    // pub fn free(&mut self, &T) {
    //     &mut self.data[]
    // }
}