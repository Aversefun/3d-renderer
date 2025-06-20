//! software rasterizer
#![warn(missing_docs, clippy::missing_docs_in_private_items)]

use rand::prelude::*;
use std::ops::{Mul, Sub};
use tracing::{event, span, Level};

/// A position, rotation, or something else.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    /// The X component.
    x: f64,
    /// The Y component.
    y: f64,
    /// The Z component.
    z: f64,
}

impl Vec3 {
    /// Get the X component.
    pub fn x(self) -> f64 {
        self.x
    }
    /// Set the X component.
    pub fn set_x(&mut self, value: f64) {
        self.x = value;
    }

    /// Get the Y component.
    pub fn y(self) -> f64 {
        self.y
    }
    /// Set the Y component.
    pub fn set_y(&mut self, value: f64) {
        self.y = value;
    }

    /// Get the Z component.
    pub fn z(self) -> f64 {
        self.z
    }
    /// Set the Z component.
    pub fn set_z(&mut self, value: f64) {
        self.z = value;
    }
}

impl Vec3 {
    /// Get the red component (maps to X).
    pub fn r(self) -> f64 {
        self.x
    }
    /// Set the red component (maps to X).
    pub fn set_r(&mut self, value: f64) {
        self.x = value;
    }

    /// Get the green component (maps to Y).
    pub fn g(self) -> f64 {
        self.y
    }
    /// Set the green component (maps to Y).
    pub fn set_g(&mut self, value: f64) {
        self.y = value;
    }

    /// Get the blue component (maps to Z).
    pub fn b(self) -> f64 {
        self.z
    }
    /// Set the blue component (maps to Z).
    pub fn set_b(&mut self, value: f64) {
        self.z = value;
    }
}

/// A position, rotation, or something else.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    /// The X component.
    pub x: f64,
    /// The Y component.
    pub y: f64,
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul for Vec2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Vec2 {
    /// Take the dot product of two Vec2s.
    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }
    /// Turn the vector by 90 degrees clockwise.
    pub fn clockwise90(self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }

    /// Is the provided point p on the right side of the line?
    pub fn point_on_right_line(self, b: Vec2, p: Vec2) -> bool {
        let ap = p - self;
        let ab_perp = (b - self).clockwise90();

        ap.dot(ab_perp) >= 0.0
    }
}

impl rand::distr::Distribution<Vec2> for rand::distr::StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec2 {
        Vec2 {
            x: rng.random(),
            y: rng.random(),
        }
    }
}

impl rand::distr::Distribution<Vec3> for rand::distr::StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        Vec3 {
            x: rng.random(),
            y: rng.random(),
            z: rng.random(),
        }
    }
}

#[cfg(feature = "image_types")]
impl From<Vec3> for image::Rgb<u8> {
    fn from(val: Vec3) -> Self {
        image::Rgb([
            (val.r() * 256.0) as u8,
            (val.g() * 256.0) as u8,
            (val.b() * 256.0) as u8,
        ])
    }
}

/// A 2D triangle.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tri2 {
    /// The points of the triangle.
    pub points: [Vec2; 3],
    /// The COLOR
    pub color: Vec3,
}

impl Mul<f64> for Tri2 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            points: [
                self.points[0] * rhs,
                self.points[1] * rhs,
                self.points[2] * rhs,
            ],
            color: self.color,
        }
    }
}

impl Tri2 {
    /// Is the provided point inside the triangle?
    pub fn inside(self, point: Vec2) -> bool {
        let side_ab = self.points[0].point_on_right_line(self.points[1], point);
        let side_bc = self.points[1].point_on_right_line(self.points[2], point);
        let side_ca = self.points[2].point_on_right_line(self.points[0], point);

        side_ab == side_bc && side_bc == side_ca
    }
    /// Returns the bounding box of the triangle in a pair of coordinates (top-left, and
    /// bottom-right).
    pub fn bounding_box(self) -> (Vec2, Vec2) {
        let top_left = Vec2 {
            x: self.points.iter().map(|v| v.x).reduce(f64::min).unwrap(),
            y: self.points.iter().map(|v| v.y).reduce(f64::min).unwrap(),
        };
        let bottom_right = Vec2 {
            x: self.points.iter().map(|v| v.x).reduce(f64::max).unwrap(),
            y: self.points.iter().map(|v| v.y).reduce(f64::max).unwrap(),
        };
        (top_left, bottom_right)
    }
}

impl rand::distr::Distribution<Tri2> for rand::distr::StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Tri2 {
        Tri2 {
            points: [self.sample(rng), self.sample(rng), self.sample(rng)],
            color: self.sample(rng),
        }
    }
}

/// A scene.
#[derive(Debug, Clone, PartialEq)]
pub struct Scene {
    /// The output.
    output: Box<[[Vec3; 600]; 600]>,
    /// The triangle.
    triangles: Vec<Tri2>,
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

/// default output (all black)
static DEFAULT_OUTPUT: [[Vec3; 600]; 600] = [[Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                }; 600]; 600];

impl Scene {
    /// Create a new Scene.
    pub fn new() -> Self {
        let span = span!(Level::TRACE, "initalize_scene");
        let _enter = span.enter();
        Self {
            output: Box::new(
                DEFAULT_OUTPUT
            ),
            triangles: vec![
                Tri2 {
                    points: [
                        Vec2 { x: 0.0, y: 0.0 },
                        Vec2 { x: 0.0, y: 0.0 },
                        Vec2 { x: 0.0, y: 0.0 },
                    ],
                    color: Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0
                    }
                };
                20
            ]
            .iter()
            .map(|_| rand::rng().random::<Tri2>() * 512.0)
            .collect::<Vec<Tri2>>(),
        }
    }
    /// Render this Scene.
    pub fn render(&mut self) {
        let span = span!(Level::TRACE, "render_scene");
        let _enter = span.enter();
        
        for triangle in &self.triangles {
            let (top_left, bottom_right) = triangle.bounding_box();
            event!(Level::TRACE, "calculated triangle bounding box: {top_left:#?}, {bottom_right:#?}");

            for (y, row) in self.output[top_left.x as usize..bottom_right.x as usize]
                .iter_mut()
                .enumerate()
            {
                for (x, color) in row[top_left.y as usize..bottom_right.y as usize]
                    .iter_mut()
                    .enumerate()
                {
                    if triangle.inside(Vec2 {
                        x: x as f64,
                        y: y as f64,
                    }) {
                        (*color) = triangle.color
                    }
                }
            }
        }
    }
    /// Display the texture on the window.
    fn display_tex_sdl3(&self, texture: &mut sdl3::render::Texture) {
        texture
            .update(
                None,
                &self
                    .output
                    .iter()
                    .flatten()
                    .flat_map(|val| {
                        [
                            (val.r() * 256.0) as u8,
                            (val.g() * 256.0) as u8,
                            (val.b() * 256.0) as u8,
                        ]
                    })
                    .collect::<Vec<u8>>(),
                self.output[0].len() * 3,
            )
            .unwrap();
    }
    /// Display this rendered Scene.
    #[cfg(feature = "sdl3")]
    pub fn display_sdl3(&mut self) -> Result<(), impl std::error::Error> {
        let sdl = sdl3::init()?;

        let mut canvas = sdl
            .video()?
            .window(
                "ThreeD Window",
                self.output[0].len() as u32,
                self.output.len() as u32,
            )
            .build()
            .unwrap()
            .into_canvas();

        let creator = canvas.texture_creator();

        let mut texture = creator
            .create_texture_static(
                sdl3::pixels::PixelFormat::try_from(sdl3::sys::pixels::SDL_PIXELFORMAT_RGB24)
                    .unwrap(),
                self.output[0].len() as u32,
                self.output.len() as u32,
            )
            .unwrap();

        self.display_tex_sdl3(&mut texture);

        canvas.copy(&texture, None, None)?;

        canvas.present();

        let mut pump = sdl.event_pump()?;
        for event in pump.wait_iter() {
            match event {
                sdl3::event::Event::Quit { timestamp: _ } => {
                    break;
                }
                sdl3::event::Event::KeyDown {
                    timestamp: _,
                    window_id: _,
                    keycode,
                    scancode: _,
                    keymod: _,
                    repeat: _,
                    which: _,
                    raw: _,
                } => match keycode.unwrap() {
                    sdl3::keyboard::Keycode::Q => {
                        break;
                    }
                    sdl3::keyboard::Keycode::R => {
                        self.render();
                        self.display_tex_sdl3(&mut texture);

                        canvas.copy(&texture, None, None)?;
                        canvas.present();
                    }
                    sdl3::keyboard::Keycode::T => {
                        (*self) = Self::new();
                        self.render();
                        self.display_tex_sdl3(&mut texture);

                        canvas.copy(&texture, None, None)?;
                        canvas.present();
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        Ok::<_, sdl3::Error>(())
    }
}
