pub struct Matrix<T> {
    pub n: usize,
    pub m: usize,
    pub data: Box<[T]>,
}