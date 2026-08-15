#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Vector<T, const K: usize>([T; K]);
