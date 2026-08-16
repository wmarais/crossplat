use crate::mathematics::traits::Number;
use crate::mathematics::computer_graphics::{Plane, Vector};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Camera<T: Number>([T; 9]);


impl<T: Number> Camera<T> {
    // The position of the camera in space.
    const IDX_LEFT: usize = 0;
    const IDX_UP: usize = 1;
    const IDX_FORWARD: usize = 2;

    // The rotation of the camera in space.
    const IDX_YAW: usize = 4;
    const IDX_PITCH: usize = 5;
    const IDX_ROLL: usize = 6;

    // The Near and Far Plane.
    const IDX_NEAR: usize = 7;
    const IDX_FAR: usize = 8;
    
    // The horizontal field-of-view.
    const IDX_FOV: usize = 9;

    // pub fn near_plane(&self) -> Plane<T> {
    //     Plane::new(Vertex::new([self.0]), distance)
    // }

    // pub fn far_plane(&self) -> Plane<T> {

    // }

    // pub fn forward(&self) -> Vector<T> {
    //     Vector::new([
    //         yaw.sin() * pitch.cos(),
    //         T::zero()-pitch.sin(),
    //         yaw.cos() * pitch.cos(),
    //         T::zero()
    //     ]
    // } 




    pub fn move_up(step: T) {

    }

    pub fn move_left(step: T) {

    }

    pub fn move_forward(step: T) {

    }


}