use derive_more::{Add, AddAssign, Div, DivAssign, MulAssign, Product, Sub};

pub type Point3 = Vec3;

#[derive(Default, Debug, Add, AddAssign, MulAssign, DivAssign, Sub, Div, Product, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    const MAX_COLOR: f64 = 255.999;

    pub fn new(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Self {
        Self { x: x.into(), y: y.into(), z: z.into() }
    }

    #[inline]
    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    #[inline]
    pub fn dot(&self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[inline]
    pub fn cross(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    #[inline]
    pub fn scale_vec(&self, t: f64) -> Self {
        Self { x: self.x * t, y: self.y * t, z: self.z * t }
    }

    #[inline]
    pub fn unit_vec(v: &Self) -> Self {
        v.scale_vec(1.0 / &v.length())
    }

    #[inline]
    pub fn write_color(&self) {
        let (x, y, z) = (
            (self.x * Self::MAX_COLOR) as i32,
            (self.y * Self::MAX_COLOR) as i32,
            (self.y * Self::MAX_COLOR) as i32,
        );
        println!("{} {} {}", x, y, z);
    }
}

impl std::fmt::Display for Vec3 {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x, y, z) = (
            (self.x * Self::MAX_COLOR) as i32,
            (self.y * Self::MAX_COLOR) as i32,
            (self.z * Self::MAX_COLOR) as i32,
        );
        writeln!(f, "{} {} {}", x, y, z)
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}