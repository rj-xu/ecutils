use std::cmp::PartialEq;

struct Point {
    x: i64,
    y: i64,
}

struct Curve {
    p: i64,
    a: i64,
    b: i64,
    g: Point,
    n: i64,
    h: i64,
}

impl PartialEq for &Point {
    fn eq(&self, other: &Self) -> bool {
        return (self.x == other.x) && (self.y == other.y);
    }
}

impl Curve {
    fn is_point_on_curve(&self, p: &Point) -> bool {
        let left_side = p.y.pow(2) % self.p;
        let right_side = (p.x.pow(3) + self.a * p.x + self.b) % self.p;
        return left_side == right_side;
    }
    fn add_points(&self, p1: &Point, p2: &Point) -> Result<Point, bool> {
        if (!self.is_point_on_curve(p1)) || (!self.is_point_on_curve(p2)) {
            return Err(false);
        }

        if p1 == p2 {
            let n = (x * p1.x.pow(2) + self.a) % self.p;
            let d = (2 * p1.y) % self.p;
            let inv = d.pow(-1) % self.p;
        }
    }
}
