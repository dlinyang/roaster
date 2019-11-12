pub struct VectorN<T> {
    pub data: Box<[T]>,
}

impl<T> VectorN<T> {
    pub fn new(data: [T]) -> Self {
        VectorN {
            data,
        }
    }
}