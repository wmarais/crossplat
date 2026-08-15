
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Matrix<T, const M: usize, const N: usize>([[T; N]; M]);
