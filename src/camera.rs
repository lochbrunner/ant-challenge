use nalgebra::{Isometry3, Perspective3, Point3, Vector3};
use std::f32::consts::PI;

pub struct Camera {
    projection: Perspective3<f32>,
    left_right_radians: f32,
    up_down_radians: f32,
    orbit_radius: f32,
    target: Point3<f32>,
}

impl Camera {
    pub fn new() -> Camera {
        let fovy = PI / 3.0;

        Camera {
            projection: Perspective3::new(2.0, fovy, 0.1, 50.0),
            left_right_radians: 45.0f32.to_radians(),
            up_down_radians: 80.0f32.to_radians(),
            orbit_radius: 6.,
            target: Point3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn view(&self) -> [f32; 16] {
        let eye = self.get_eye_pos();

        let view = Isometry3::look_at_rh(&eye, &self.target, &Vector3::y());

        let view = view.to_homogeneous();

        let mut view_array = [0.; 16];
        view_array.copy_from_slice(view.as_slice());

        view_array
    }

    pub fn view_flipped_y(&self) -> [f32; 16] {
        let mut eye = self.get_eye_pos();
        eye.y = -1.0 * eye.y;

        let view = Isometry3::look_at_rh(&eye, &self.target, &Vector3::y());

        let view = view.to_homogeneous();

        let mut view_array = [0.; 16];
        view_array.copy_from_slice(view.as_slice());

        view_array
    }

    pub fn get_eye_pos(&self) -> Point3<f32> {
        let yaw = self.left_right_radians;
        let pitch = self.up_down_radians;

        let eye_x = self.orbit_radius * yaw.sin() * pitch.cos() + self.target.x;
        let eye_y = self.orbit_radius * pitch.sin() + self.target.y;
        let eye_z = self.orbit_radius * yaw.cos() * pitch.cos() + self.target.z;

        Point3::new(eye_x, eye_y, eye_z)
    }
    pub fn projection(&self) -> [f32; 16] {
        let mut perspective_array = [0.; 16];
        perspective_array.copy_from_slice(self.projection.as_matrix().as_slice());

        perspective_array
    }

    pub fn orbit_left_right(&mut self, delta: f32) {
        self.left_right_radians += delta;
    }

    pub fn orbit_up_down(&mut self, delta: f32) {
        self.up_down_radians += delta;

        // Make sure:
        // 0.1 <= radians <= PI / 2.1
        // in order to restrict the camera's up/down orbit motion

        if self.up_down_radians - (PI / 2.1) > 0. {
            self.up_down_radians = PI / 2.1;
        }

        if self.up_down_radians - 0.1 < 0. {
            self.up_down_radians = 0.1;
        }
    }

    pub fn move_left_right(&mut self, delta: f32) {
        let yaw = self.left_right_radians;
        self.target.x += yaw.cos() * delta;
        self.target.z += -yaw.sin() * delta;
    }
    pub fn move_up_down(&mut self, delta: f32) {
        let pitch = self.up_down_radians;
        let yaw = self.left_right_radians;

        self.target.x += yaw.sin() * -pitch.sin() * delta;
        self.target.y += pitch.cos() * delta;
        self.target.z += yaw.cos() * -pitch.sin() * delta;
    }

    pub fn zoom(&mut self, delta: f32) {
        self.orbit_radius *= (1.05 as f32).powf(delta);

        if self.orbit_radius > 50. {
            self.orbit_radius = 50.;
        } else if self.orbit_radius < 1. {
            self.orbit_radius = 1.;
        }
    }
}
