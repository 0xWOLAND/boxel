use winit::event::{WindowEvent, VirtualKeyCode, KeyboardInput, ElementState};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Camera {
    pub eye: [f64; 3],
    pub targ: [f64; 3],
    pub up: [f64; 3],
    pub aspect: f64,
    pub fovy: f64,
    pub znear: f64,
    pub zfar: f64
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f64; 4]; 4],
}

pub struct CameraController {
    speed: f64,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

pub struct CameraAutoRotate {
    pub camera: Camera,
    pub rad: f64,
    pub cur: cgmath::Deg<f64>,
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f64> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

impl Camera {
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f64> {
        let view = cgmath::Matrix4::look_at_rh(
            self.eye.into(),
            self.targ.into(),
            self.up.into()
        );
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }

}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        return Self {
            view_proj: cgmath::Matrix4::identity().into(),
        };
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

impl CameraController {
    pub fn new(speed: f64) -> Self {
        Self {
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput { 
                input: KeyboardInput {
                    state,
                    virtual_keycode: Some(keycode),
                    ..
                },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::W | VirtualKeyCode::Up  => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
             _ => false
        }
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        use cgmath::InnerSpace;
        
        let mut _targ: cgmath::Point3<f64> = camera.targ.into();
        let mut _eye: cgmath::Point3<f64> = camera.eye.into();
        let mut _up: cgmath::Vector3<f64> = camera.up.into();

        let forward = _targ - _eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        if self.is_forward_pressed && forward_mag > self.speed {
            _eye += forward_norm * self.speed;
        }
        if self.is_backward_pressed {
            _eye -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(_up);

        let forward = _targ - _eye;
        let forward_mag = forward.magnitude();

        if self.is_right_pressed {
            _eye = _targ - (forward + right * self.speed).normalize() * forward_mag;
        }
        if self.is_left_pressed {
            _eye = _targ - (forward - right * self.speed).normalize() * forward_mag;
        }

        camera.targ = [_targ.x, _targ.y, _targ.z];
        camera.eye = [_eye.x, _eye.y, _eye.z];
        camera.up = [_up.x, _up.y, _up.z];
    }
}

impl CameraAutoRotate {
    pub fn new(camera: Camera, rad: f64) -> Self {
        let cur: cgmath::Deg<f64> = cgmath::Deg(0.0);
        return Self {
            camera,
            rad,
            cur,
        }
    }

    pub fn rotate(&mut self, camera_uniform: &mut CameraUniform) {
        self.cur += cgmath::Deg(self.rad);
        camera_uniform.view_proj = (OPENGL_TO_WGPU_MATRIX * self.camera.build_view_projection_matrix() * (cgmath::Matrix4::<f64>::from_angle_z(cgmath::Deg(0.0)))).into();

    }
}