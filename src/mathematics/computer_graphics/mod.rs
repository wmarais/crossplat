mod projection_camera;
pub use projection_camera::*;

mod pyramid_frustum;
pub use pyramid_frustum::*;

use crate::mathematics::geometry;
use crate::mathematics::linear_algebra;

/// A point in 3D space (vertex).
pub type Vertex<T> = geometry::Point<T, 4>;

/// A vector in 3D space.
pub type Vector<T> = linear_algebra::Vector<T, 4>;

/// A plane in 3D space.
pub type Plane<T> = geometry::Plane<T, 4>;

/// A ray in 3D space.
pub type Ray<T> = geometry::Ray<T, 4>;

/// A circle in 3D space (sphere).
pub type Sphere<T> = geometry::Circle<T, 3>;

/// A rectangle in 3D space (box).
pub type Box<T> = geometry::Rectangle<T, 4>;
