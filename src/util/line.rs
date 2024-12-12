use super::Vec2;

/// A bounded 2d line
pub struct Line2 {
    pub start: Vec2,
    pub dir: Vec2,
    pub len: usize,
}

impl Line2 {
    pub const fn end(&self) -> Vec2 {
        self.start
            .const_add(self.dir.const_scalar_mul(self.len as i64))
    }

    pub const fn x_range(&self) -> (i64, i64) {
        let (a, b) = (self.start.x, self.end().x);
        if a < b {
            (a, b)
        } else {
            (b, a)
        }
    }

    pub const fn y_range(&self) -> (i64, i64) {
        let (a, b) = (self.start.y, self.end().y);
        if a < b {
            (a, b)
        } else {
            (b, a)
        }
    }

    pub fn intersection(&self, other: &Self) -> Option<Vec2> {
        // Only work on axis-aligned orthogonal lines for now
        debug_assert_eq!(self.dir.x.abs() + self.dir.y.abs(), 1);
        debug_assert_eq!(other.dir.x.abs() + other.dir.y.abs(), 1);
        debug_assert_eq!(self.dir.dot(other.dir), 0);

        fn range_intersect(l: (i64, i64), r: (i64, i64)) -> Option<i64> {
            if l.0 == l.1 && l.0 >= r.0 && l.0 <= r.1 {
                Some(l.0)
            } else if r.0 == r.1 && r.0 >= l.0 && r.0 <= l.1 {
                Some(r.0)
            } else {
                None
            }
        }

        let x = range_intersect(self.x_range(), other.x_range())?;
        let y = range_intersect(self.y_range(), other.y_range())?;

        Some(Vec2::new(x, y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_range() {
        let l = Line2 {
            start: Vec2::new(10, 20),
            dir: Vec2::new(1, 0),
            len: 3,
        };

        assert_eq!(l.x_range(), (10, 13));
        assert_eq!(l.y_range(), (20, 2));
    }
    
    #[test]
    fn test_line_intersection() {
        let l1 = Line2 {
            start: Vec2::new(10, 0),
            dir: Vec2::new(0, 1),
            len: 10,
        };
        
        let mut l2 = Line2 {
            start: Vec2::new(5, 5),
            dir: Vec2::new(1, 0),
            len: 6,
        };
        
        assert_eq!(l1.intersection(&l2), Some(Vec2::new(10, 5)));
        
        l2.len = 4;
        assert_eq!(l1.intersection(&l2), None);
    }
}
