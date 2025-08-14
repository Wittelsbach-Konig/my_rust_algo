use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct Triangle<T> {
    a: T,
    b: T,
    c: T,
}

impl<T> Triangle<T>
where
    T: PartialEq + PartialOrd + Copy + Add<Output = T>,
{
    pub fn build(mut sides: [T; 3]) -> Option<Triangle<T>> {
        sides.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let [a, b, c] = sides;

        (a + b > c).then_some(Triangle { a, b, c })
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.b == self.c
    }

    pub fn is_scalene(&self) -> bool {
        self.a != self.b && self.b != self.c && self.a != self.c
    }

    pub fn is_isosceles(&self) -> bool {
        self.a == self.b || self.b == self.c || self.a == self.c
    }
}
