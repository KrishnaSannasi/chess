use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Vector {
    pub fn zero() -> Self {
        Vector { x: 0, y: 0 }
    }
}

impl Default for Vector {
    fn default() -> Self {
        Self::zero()
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(mut self, Vector { x, y }: Self) -> Self {
        self.x += x;
        self.y += y;
        self
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(mut self, Vector { x, y }: Self) -> Self {
        self.x -= x;
        self.y -= y;
        self
    }
}

macro_rules! impl_op {
    ($type:ident $op:ident $op_fun:ident $tok:tt) => {
        impl $op<$type> for Vector {
            type Output = Vector;

            fn $op_fun(mut self, other: $type) -> Vector {
                self.x $tok other;
                self.y $tok other;
                self
            }
        }
        impl $op<Vector> for $type {
            type Output = Vector;

            fn $op_fun(self, other: Vector) -> Vector {
                other.$op_fun(self)
            }
        }
    };
}

impl_op! { i32 Mul mul *= }
impl_op! { i32 Div div /= }
