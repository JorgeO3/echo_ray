use super::color::Color;
use super::vec3::{Point3, Vec3};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: impl Into<Point3>, dir: impl Into<Vec3>) -> Self {
        Self { orig: orig.into(), dir: dir.into() }
    }

    fn ray(&self, t: f64) -> Point3 {
        self.orig + self.dir.scale_vec(t)
    }

    pub fn ray_color(r: &Self) -> Color {
        let unit_direction = Vec3::unit_vec(&r.dir);
        let a = 0.5 * (unit_direction.y + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}