use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign, Div, DivAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Dims(pub i32, pub i32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Dims3D(pub i32, pub i32, pub i32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DimsU(pub usize, pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GameMode {
    pub size: Dims3D,
    pub is_tower: bool,
}

impl Add for Dims {
    type Output = Dims;

    fn add(self, other: Dims) -> Dims {
        Dims(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Dims {
    type Output = Dims;

    fn sub(self, other: Dims) -> Dims {
        Dims(self.0 - other.0, self.1 - other.1)
    }
}

impl AddAssign for Dims {
    fn add_assign(&mut self, other: Dims) {
        self.0 += other.0;
        self.1 += other.1;
    }
}

impl SubAssign for Dims {
    fn sub_assign(&mut self, other: Dims) {
        self.0 -= other.0;
        self.1 -= other.1;
    }
}

impl Mul<i32> for Dims {
    type Output = Dims;

    fn mul(self, other: i32) -> Dims {
        Dims(self.0 * other, self.1 * other)
    }
}

impl MulAssign<i32> for Dims {
    fn mul_assign(&mut self, other: i32) {
        self.0 *= other;
        self.1 *= other;
    }
}

impl Div<i32> for Dims {
    type Output = Dims;

    fn div(self, other: i32) -> Dims {
        Dims(self.0 / other, self.1 / other)
    }
}

impl DivAssign<i32> for Dims {
    fn div_assign(&mut self, other: i32) {
        self.0 /= other;
        self.1 /= other;
    }
}

impl From<(i32, i32)> for Dims {
    fn from(tuple: (i32, i32)) -> Self {
        Dims(tuple.0, tuple.1)
    }
}

impl From<Dims> for (i32, i32) {
    fn from(val: Dims) -> Self {
        (val.0, val.1)
    }
}

impl From<Dims3D> for Dims {
    fn from(dims: Dims3D) -> Self {
        Dims(dims.0, dims.1)
    }
}

impl Add for Dims3D {
    type Output = Dims3D;

    fn add(self, other: Dims3D) -> Dims3D {
        Dims3D(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Dims3D {
    type Output = Dims3D;

    fn sub(self, other: Dims3D) -> Dims3D {
        Dims3D(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl AddAssign for Dims3D {
    fn add_assign(&mut self, other: Dims3D) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl SubAssign for Dims3D {
    fn sub_assign(&mut self, other: Dims3D) {
        self.0 -= other.0;
        self.1 -= other.1;
        self.2 -= other.2;
    }
}

impl Mul<i32> for Dims3D {
    type Output = Dims3D;

    fn mul(self, other: i32) -> Dims3D {
        Dims3D(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl MulAssign<i32> for Dims3D {
    fn mul_assign(&mut self, other: i32) {
        self.0 *= other;
        self.1 *= other;
        self.2 *= other;
    }
}

impl Div<i32> for Dims3D {
    type Output = Dims3D;

    fn div(self, other: i32) -> Dims3D {
        Dims3D(self.0 / other, self.1 / other, self.2 / other)
    }
}

impl DivAssign<i32> for Dims3D {
    fn div_assign(&mut self, other: i32) {
        self.0 /= other;
        self.1 /= other;
        self.2 /= other;
    }
}

impl From<(i32, i32, i32)> for Dims3D {
    fn from(tuple: (i32, i32, i32)) -> Self {
        Dims3D(tuple.0, tuple.1, tuple.2)
    }
}

impl From<Dims3D> for (i32, i32, i32) {
    fn from(val: Dims3D) -> Self {
        (val.0, val.1, val.2)
    }
}

impl From<Dims> for Dims3D {
    fn from(dims: Dims) -> Self {
        Dims3D(dims.0, dims.1, 0)
    }
}

impl Add for DimsU {
    type Output = DimsU;

    fn add(self, other: DimsU) -> DimsU {
        DimsU(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for DimsU {
    type Output = DimsU;

    fn sub(self, other: DimsU) -> DimsU {
        DimsU(self.0 - other.0, self.1 - other.1)
    }
}

impl AddAssign for DimsU {
    fn add_assign(&mut self, other: DimsU) {
        self.0 += other.0;
        self.1 += other.1;
    }
}

impl SubAssign for DimsU {
    fn sub_assign(&mut self, other: DimsU) {
        self.0 -= other.0;
        self.1 -= other.1;
    }
}

impl Mul<usize> for DimsU {
    type Output = DimsU;

    fn mul(self, other: usize) -> DimsU {
        DimsU(self.0 * other, self.1 * other)
    }
}

impl MulAssign<usize> for DimsU {
    fn mul_assign(&mut self, other: usize) {
        self.0 *= other;
        self.1 *= other;
    }
}

impl Div<usize> for DimsU {
    type Output = DimsU;

    fn div(self, other: usize) -> DimsU {
        DimsU(self.0 / other, self.1 / other)
    }
}

impl DivAssign<usize> for DimsU {
    fn div_assign(&mut self, other: usize) {
        self.0 /= other;
        self.1 /= other;
    }
}

impl From<(usize, usize)> for DimsU {
    fn from(tuple: (usize, usize)) -> Self {
        DimsU(tuple.0, tuple.1)
    }
}

impl From<DimsU> for (usize, usize) {
    fn from(val: DimsU) -> Self {
        (val.0, val.1)
    }
}
