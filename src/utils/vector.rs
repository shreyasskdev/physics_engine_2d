use libm::{cos, sin, atan2, sqrt};
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

pub struct Vector{
    _x: f64,
    _y: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Vector {
        Self { _x: x, _y: y }
    }

    pub fn set_x(&mut self, value: f64){
        self._x = value;
    }
    pub fn get_x(&self) -> f64 {
        self._x
    }


    pub fn set_y(&mut self, value: f64){
        self._y = value;
    }
    pub fn get_y(&self) -> f64 {
        self._y
    }

    pub fn set_angle(&mut self, angle: f64) {
        let length = self.get_length();
        self._x = cos(angle) * length;
        self._y = sin(angle)  * length;
    }
    pub fn get_angle(&self) -> f64 {
        atan2(self._y, self._x)
    }

    pub fn set_length(&mut self, length: f64) {
        let angle = self.get_angle();
        self._x = cos(angle) * length;
        self._y = sin(angle) * length;
    }
    pub fn get_length(&self) -> f64 {
        sqrt(self._x * self._x + self._y * self._y)
    }
}

impl Add for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Self::Output {
        Vector::new(self._x + rhs._x, self._y + rhs._y)
    }
}
impl Sub for Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Self::Output {
        Vector::new(self._x - rhs._x, self._y - rhs._y)
    }
}
impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector::new(self._x * rhs, self._y * rhs)
    }
}
impl Div<f64> for Vector {
    type Output = Vector;
    fn div(self, rhs: f64) -> Self::Output {
        Vector::new(self._x / rhs, self._y / rhs)
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self._x += rhs._x;
        self._y += rhs._y;
    } 
}
impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        self._x -= rhs._x;
        self._y -= rhs._y;
    } 
}
impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, rhs: f64) {
        self._x *= rhs;
        self._y *= rhs;
    } 
}
impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, rhs: f64) {
        self._x /= rhs;
        self._y /= rhs;
    }  
}