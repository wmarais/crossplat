use crate::mathematics::computer_graphics::{Plane, Vector, Vertex};
use crate::mathematics::geometry::Angle;
use crate::mathematics::traits::Number;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Camera<T: Number> {
    position: Vertex<T>,
    direction: Vector<T>,
    yaw: Angle<T>,
    pitch: Angle<T>,
    roll: Angle<T>,
    fov: Angle<T>,
    near: T,
    far: T,
}

impl<T: Number> Camera<T> {
    pub fn yaw(&self) -> Angle<T> {
        self.yaw
    }

    pub fn pitch(&self) -> Angle<T> {
        self.pitch
    }

    pub fn roll(&self) -> Angle<T> {
        self.roll
    }

    pub fn position(&self) -> &Vertex<T> {
        &self.position
    }

    pub fn direction(&self) -> &Vector<T> {
        &self.direction
    }

    pub fn move_to(&mut self, position: &Vertex<T>) {
        self.position = *position;
    }

    pub fn move_by(&mut self, step: &Vector<T>) {
        self.position = self.position + *step;
    }

    pub fn rotate_to(&mut self, yaw: Angle<T>, pitch: Angle<T>, roll: Angle<T>) {
        self.yaw = yaw;
        self.pitch = pitch;
        self.roll = roll;
    }

    pub fn rotate_by(&mut self, yaw: Angle<T>, pitch: Angle<T>, roll: Angle<T>) {
        self.yaw = self.yaw + yaw;
        self.pitch = self.pitch + pitch;
        self.roll = self.roll + roll;
    }

    // pub fn calc_direction(&self) -> Vector<T> {

    // }

    // // The position of the camera in space.
    // const IDX_X: usize = 0;
    // const IDX_Y: usize = 1;
    // const IDX_Z: usize = 2;

    // // The rotation of the camera in space.
    // const IDX_YAW: usize = 4;
    // const IDX_PITCH: usize = 5;
    // const IDX_ROLL: usize = 6;

    // // The Near and Far Plane.
    // const IDX_NEAR: usize = 7;
    // const IDX_FAR: usize = 8;

    // // The horizontal field-of-view.
    // const IDX_FOV: usize = 9;

    // pub fn yaw(&self) -> Angle<T> {
    //     Angle::from_radians(self.0[Self::IDX_YAW])
    // }

    // pub fn pitch(&self) -> Angle<T> {
    //     Angle::from_radians(self.0[Self::IDX_PITCH])
    // }

    // pub fn roll(&self) -> Angle<T> {
    //     Angle::from_radians(self.0[Self::IDX_ROLL])
    // }

    // pub fn fov(&self) -> Angle<T> {
    //     Angle::from_radians(self.0[Self::IDX_FOV])
    // }

    // pub fn position(&self) -> Vertex<T> {
    //     Vertex::new([
    //         self.0[Self::IDX_X],
    //         self.0[Self::IDX_Y],
    //         self.0[Self::IDX_Z],
    //         T::one()
    //     ])
    // }

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

    pub fn move_up(step: T) {}

    pub fn move_left(step: T) {}

    pub fn move_forward(step: T) {}
}
