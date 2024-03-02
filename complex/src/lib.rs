use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Complex { re, im }
    }

    /// Calculates the norm of the complex number
    pub fn norm(&self) -> f64 {
        (self.re.powi(2) + self.im.powi(2)).sqrt()
    }

    /// Calculates the square of the norm of the complex number, without taking the square root
    pub fn norm_sqr(&self) -> f64 {
        self.re.powi(2) + self.im.powi(2)
    }

    /// Calculates the sine of the complex number
    pub fn sin(&self) -> Self {
        Complex {
            re: self.re.sin() * self.im.cosh(),
            im: self.re.cos() * self.im.sinh(),
        }
    }

    /// Calculates the cosine of the complex number
    pub fn cos(&self) -> Self {
        Complex {
            re: self.re.cos() * self.im.cosh(),
            im: -self.re.sin() * self.im.sinh(),
        }
    }

    /// Raises the complex number to an integer power
    pub fn powi(&self, mut exp: i32) -> Self {
        if exp == 0 {
            return Complex { re: 1.0, im: 0.0 };
        }

        let mut base = *self;
        let mut result = Complex { re: 1.0, im: 0.0 };

        while exp > 0 {
            if exp % 2 == 1 {
                result = result * base;
            }
            exp /= 2;
            base = base * base;
        }

        result
    }
}

impl Add for Complex {
    type Output = Self;

    /// Adds two complex numbers
    fn add(self, other: Self) -> Self::Output {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl Sub for Complex {
    type Output = Self;

    /// Subtracts one complex number from another
    fn sub(self, other: Self) -> Self::Output {
        Complex {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }
}

impl Mul for Complex {
    type Output = Self;

    /// Multiplies two complex numbers
    fn mul(self, other: Self) -> Self::Output {
        Complex {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

impl Div for Complex {
    type Output = Self;

    /// Divides one complex number by another
    fn div(self, other: Self) -> Self::Output {
        if other.re == 0.0 && other.im == 0.0 {
            Complex { re: 0.0, im: 0.0 };
        }

        let denominator = other.re * other.re + other.im * other.im;
        Complex {
            re: (self.re * other.re + self.im * other.im) / denominator,
            im: (self.im * other.re - self.re * other.im) / denominator,
        }
    }
}
