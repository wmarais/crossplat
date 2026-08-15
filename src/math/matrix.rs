use super::NumOps;

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Matrix<T: NumOps, const M: usize, const N: usize>([[T; N]; M]);
