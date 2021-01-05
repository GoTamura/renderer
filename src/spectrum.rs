use std::ops::*;

#[macro_export]
macro_rules! spe {
    ( $x:expr ) => {
        Spectrum {
            r: $x,
            g: $x,
            b: $x,
        }
    };
    ( $x:expr, $y:expr, $z:expr ) => {
        Spectrum {
            r: $x,
            g: $y,
            b: $z,
        }
    };
}

pub const BLACK: Spectrum = Spectrum {
    r: 0.,
    g: 0.,
    b: 0.,
};
pub struct Spectrum {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Mul<Spectrum> for Spectrum {
    type Output = Spectrum;

    fn mul(self, other: Spectrum) -> Spectrum {
        Spectrum {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl Mul<f64> for Spectrum {
    type Output = Spectrum;

    fn mul(self, rhs: f64) -> Spectrum {
        Spectrum {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Add for Spectrum {
    type Output = Spectrum;

    fn add(self, other: Spectrum) -> Spectrum {
        Spectrum {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Spectrum {
    pub fn min(self, other: Spectrum) -> Spectrum {
        Spectrum {
            r: f64::min(self.r, other.r),
            g: f64::min(self.g, other.g),
            b: f64::min(self.b, other.b),
        }
    }

    pub fn max(self, other: Spectrum) -> Spectrum {
        Spectrum {
            r: f64::max(self.r, other.r),
            g: f64::max(self.g, other.g),
            b: f64::max(self.b, other.b),
        }
    }
}
