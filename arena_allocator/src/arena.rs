use std::marker::PhantomData;

struct InvariantLifetime<'id>(PhantomData<*mut &'id ()>);

pub struct ArenaToken<'id> { 
    idx: usize,
    _marker: InvariantLifetime<'id> 
}

pub struct ArenaCell<'id, T> {
    value: T,
    _marker: InvariantLifetime<'id>,
}

impl<'id, T> ArenaCell<'id, T> {
    /// Wraps some data T into a ArenaCell with brand `'id`.
    fn new(value: T) -> Self { 
        ArenaCell{value, _marker: InvariantLifetime(PhantomData)}
    }
    /// Turns an owned ArenaCell back into owned data.
    fn into_inner(self) -> T { 
        self.value
    }
    /// Turns a mutably borrowed ArenaCell to and from mutably borrowed data.
    fn get_mut(&mut self) -> &mut T { 
        &mut self.value
    }
    fn from_mut(t: &mut T) -> &mut Self { 
        unsafe { &mut *(t as *mut T as *mut Self) } // transmute since we know the layout
    }

    // Immutably borrows the ArenaCell with the same-branded token.
    #[allow(unused_variables)]
    fn borrow<'a>(&'a self, token: &'a ArenaToken<'id>) -> &'a T {
        &self.value
    }
    /// Mutably borrows the ArenaCell with the same-branded token.
    #[allow(unused_variables)]
    fn borrow_mut<'a>(&'a self, token: &'a mut ArenaToken<'id>) -> &'a mut T {
        unsafe { &mut *(self as *const Self as *mut Self) }.get_mut()
    }
}

pub struct Arena<'arena, T> {
    cells: Vec<ArenaCell<'arena, T>>,
}

impl<'arena, T> Arena<'arena, T> {
    pub fn new<F, R>(f: F) -> R
    where
        F: FnOnce(&mut Arena<'arena, T>) -> R,
    {
        let mut arena = Arena { cells: Vec::new() };
        let result = f(&mut arena);
        arena.cells.clear();
        result
    }
    
    pub fn alloc(&mut self, value: T) -> ArenaToken<'arena> {
        let idx = self.cells.len();
        self.cells.push(ArenaCell::new(value));
        ArenaToken { idx: idx, _marker: InvariantLifetime(PhantomData) }
    }

    pub fn get<'a>(&'a self, token: &'a ArenaToken<'arena>) -> &'a T {
        self.cells[token.idx].borrow(token)
    }

    pub fn get_mut<'a>(&'a self, token: &'a mut ArenaToken<'arena>) -> &'a mut T {
        self.cells[token.idx].borrow_mut(token)
    }
}