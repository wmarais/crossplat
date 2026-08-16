// use super::{NumOps, Point, Vector};

// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
// pub struct Camera<T: NumOps> {
//     up: T,
//     left: T,
//     forward: T,
//     yaw: T,
//     pitch: T,
//     roll: T,
//     near: T,
//     far: T,
//     fov: T
// }


// impl<T: NumOps> Camera<T> {
//     // The position of the camera in space.
//     const IDX_UP: usize = 0;
//     const IDX_LEFT: usize = 1;
//     const IDX_FORWARD: usize = 2;

//     // The rotation of the camera in space.
//     const IDX_YAW: usize = 4;
//     const IDX_PITCH: usize = 5;
//     const IDX_ROLL: usize = 6;

//     // The Near and Far Plane.
//     const IDX_NEAR: usize = 7;
//     const IDX_FAR: usize = 8;
    
//     // The horizontal field-of-view.
//     const IDX_FOV: usize = 9;




//     pub fn move_up(step: T) {

//     }

//     pub fn move_left(step: T) {

//     }

//     pub fn move_forward(step: T) {

//     }


// }