use super::NumOps;

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Quaternion<T: NumOps>([T; 4]);