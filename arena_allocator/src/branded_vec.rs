use std::marker::PhantomData;

#[derive(Copy, Clone, Debug)]
struct InvariantLifetime<'id>(PhantomData<*mut &'id ()>);

// A vector branded with a phantom lifetime `'id`, holding an underlying vector `Vec<T>`.
#[derive(Debug)]
pub struct BrandedVec<'id, T> {
    inner: Vec<T>,
    _marker: InvariantLifetime<'id>,
}
// A branded index into a vector with the same brand `'id`.
#[derive(Copy, Clone, Debug)]
pub struct BrandedIndex<'id> {
    idx: usize,
    _marker: InvariantLifetime<'id>,
}


impl<'id, T> BrandedVec<'id, T> {
    // Turns a regular `inner: Vec<T>` into a branded vector of type `BrandedVec<'id, T>`,
    // then passes it to a closure `f`.
    pub fn new<R>(inner: Vec<T>, f: impl for<'a> FnOnce(BrandedVec<'a, T>) -> R) -> R { 
        let bv = BrandedVec { inner, _marker: InvariantLifetime(PhantomData) };
        f(bv)
    }
    // Pushes to the vector and returns the index of the pushed element.
    pub fn push(&mut self, val: T) -> BrandedIndex<'id> {
        let idx = self.inner.len();
        self.inner.push(val);
        BrandedIndex { idx, _marker: InvariantLifetime(PhantomData) }
    }
    // Bounds-checks an index; inbounds indices are returned as `BrandedIndex`.
    pub fn get_index(&self, index: usize) -> Option<BrandedIndex<'id>> {
        if index < self.inner.len() {
            Some(BrandedIndex { idx: index, _marker: InvariantLifetime(PhantomData) })
        } else {
            None
        }
    }
    // Given an index with the same brand, returns a reference to that element in
    // the vector without performing any bounds checks.
    pub fn get(&self, index: BrandedIndex<'id>) -> &T {
        &self.inner[index.idx]
    }
    pub fn get_mut(&mut self, index: BrandedIndex<'id>) -> &mut T {
        &mut self.inner[index.idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_branded_vec() {
        let vec1: Vec<u8> = vec![10, 11];
        let vec2: Vec<u8> = vec![20, 21];
        BrandedVec::new(vec1, move |mut bvec1: BrandedVec<u8>| {        // bvec1: BrandedVec<'id1, u8>
            bvec1.push(12); let i1 = bvec1.push(13);                    // i1: BrandedIndex<'id1> is an index into bvec1
            BrandedVec::new(vec2, move |mut bvec2: BrandedVec<u8>| {    // bvec2: BrandedVec<'id2, u8>
                let i2 = bvec2.push(22);                                // i2: BrandedIndex<'id2> is an index into bvec2
                *bvec2.get_mut(i2) -= 1;                                // No bounds check! Updates to 21
                println!("{:?}", bvec2.get(i2));                        // No bounds check! Prints 21
                println!("{:?}", bvec1.get(i1));                        // No bounds check! Prints 13
                // println!("{:?}", bvec2.get(i1));                     // Rejected: i1 is not an index of bvec2
            });                                                         // end of `bvec2`'s closure
        });                                                             // end of `bvec1`'s closure
    }
}