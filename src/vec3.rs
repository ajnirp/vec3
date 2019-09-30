use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Div, Sub};

#[derive(Debug, PartialEq)]
pub struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x: x, y: y, z: z }
    }
}

impl<T: Display> Display for Vec3<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// TODO: make this Div<T: num::Float> (num = "0.2.0")
impl Div<f32> for Vec3<f32> {
    type Output = Self;

    fn div(self, f: f32) -> Self {
        if f == 0f32 {
            panic!("Cannot divide {} by zero", self);
        }

        Self {
            x: self.x / f,
            y: self.y / f,
            z: self.z / f,
        }
    }
}