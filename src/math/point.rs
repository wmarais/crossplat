
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Point<T, const K: usize>([T; K]);
