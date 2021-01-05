use crate::geometry::normal::*;
use crate::geometry::point::*;
use std::convert::From;
use std::ops::*;

pub type Vector3f = Vector3<f64>;

#[macro_export]
macro_rules! vec3f {
    ( $x:expr ) => {
        Vector3f {
            x: $x,
            y: $x,
            z: $x,
        }
    };
    ( $x:expr, $y:expr, $z:expr ) => {
        Vector3f {
            x: $x,
            y: $y,
            z: $z,
        }
    };
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Add<Output = T>> Add for Vector3<T> {
    type Output = Vector3<T>;

    fn add(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Add<Output = T> + Copy> AddAssign for Vector3<T> {
    fn add_assign(&mut self, other: Vector3<T>) {
        *self = *self + other;
    }
}

impl<T: Sub<Output = T>> Sub for Vector3<T> {
    type Output = Vector3<T>;

    fn sub(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Sub<Output = T> + Copy> SubAssign for Vector3<T> {
    fn sub_assign(&mut self, other: Vector3<T>) {
        *self = *self - other;
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn div(self, rhs: T) -> Vector3<T> {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T: Div<Output = T> + Copy> DivAssign<T> for Vector3<T> {
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn mul(self, rhs: T) -> Vector3<T> {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy> Mul<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;
    fn mul(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T: Mul<Output = T> + Copy> MulAssign<T> for Vector3<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

impl<T> Vector3<T>
where
    T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Copy,
{
    pub fn dot(self, other: Vector3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Vector3f {
    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn noramlize(self) -> Vector3f {
        let length = self.length();
        Vector3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    pub fn min(self, other: Vector3f) -> Vector3f {
        Vector3f {
            x: f64::min(self.x, other.x),
            y: f64::min(self.y, other.y),
            z: f64::min(self.z, other.z),
        }
    }

    pub fn max(self, other: Vector3f) -> Vector3f {
        Vector3f {
            x: f64::max(self.x, other.x),
            y: f64::max(self.y, other.y),
            z: f64::max(self.z, other.z),
        }
    }
}

impl<T> Index<u32> for Vector3<T> {
    type Output = T;

    fn index(&self, i: u32) -> &T {
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => &self.z,
        }
    }
}

impl<T> From<Point3<T>> for Vector3<T> {
    fn from(n: Point3<T>) -> Self {
        Vector3::<T> {
            x: n.x,
            y: n.y,
            z: n.z,
        }
    }
}

impl<T> From<Normal3<T>> for Vector3<T> {
    fn from(n: Normal3<T>) -> Self {
        Vector3::<T> {
            x: n.x,
            y: n.y,
            z: n.z,
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_add_assign() {
        println!("hello");
    }
}
