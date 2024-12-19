#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub x: i64,
    pub y: i64,
}

impl std::fmt::Debug for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec2({}, {})", self.x, self.y)
    }
}

impl Vec2 {
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub const fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    pub const fn l1_norm(self) -> i64 {
        self.x.abs() + self.y.abs()
    }
    
    pub const fn dot(self, other: Self) -> i64 {
        self.x * other.x + self.y * other.y
    }
    
    pub const fn const_add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
    
    pub const fn const_scalar_mul(self, mul: i64) -> Self {
        Self {
            x: self.x * mul,
            y: self.y * mul
        }
    }

    /// Is this vec2 inside a map that spans (0, 0) to `map_size`
    pub const fn inside_map(&self, map_size: Self) -> bool {
        self.x >= 0 && self.x < map_size.x && self.y >= 0 && self.y < map_size.y
    }
}

impl std::ops::Mul<i64> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Add<Self> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Add<Self> for &Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        *self + *rhs
    }
}

impl std::ops::AddAssign<Self> for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub<Self> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Sub<Self> for &Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        *self - *rhs
    }
}

impl std::ops::SubAssign<Self> for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
