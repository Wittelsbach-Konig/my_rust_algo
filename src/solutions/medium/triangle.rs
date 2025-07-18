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
        self.a == self.c
    }

    pub fn is_scalene(&self) -> bool {
        self.a != self.b && self.b != self.c
    }

    pub fn is_isosceles(&self) -> bool {
        self.a == self.b || self.b == self.c
    }
}

#[cfg(test)]
mod equilateral {
    use super::Triangle;
    #[test]
    fn all_sides_are_equal() {
        let input = [2, 2, 2];
        let output = Triangle::build(input).unwrap();
        assert!(output.is_equilateral());
    }
    #[test]
    fn any_side_is_unequal() {
        let input = [2, 3, 2];
        let output = Triangle::build(input).unwrap();
        assert!(!output.is_equilateral());
    }
    #[test]
    fn no_sides_are_equal() {
        let input = [5, 4, 6];
        let output = Triangle::build(input).unwrap();
        assert!(!output.is_equilateral());
    }
    #[test]
    fn sides_may_be_floats() {
        let input = [0.5, 0.5, 0.5];
        let output = Triangle::build(input).unwrap();
        assert!(output.is_equilateral());
    }
}

#[cfg(test)]
mod isosceles {
    use super::Triangle;

    #[test]
    fn last_two_sides_are_equal() {
        let input = [3, 4, 4];
        let output = Triangle::build(input).unwrap();
        assert!(output.is_isosceles());
    }
    #[test]
    fn first_two_sides_are_equal() {
        let input = [4, 4, 3];
        let output = Triangle::build(input).unwrap();
        assert!(output.is_isosceles());
    }
    #[test]
    fn first_and_last_sides_are_equal() {
        let input = [4, 3, 4];
        let output = Triangle::build(input).unwrap();
        assert!(output.is_isosceles());
    }
    #[test]
    fn equilateral_triangles_are_also_isosceles() {
        let input = [4, 4, 4];
        let output = Triangle::build(input).unwrap();
        assert!(output.is_isosceles());
    }
    #[test]
    fn no_sides_are_equal() {
        let input = [2, 3, 4];
        let output = Triangle::build(input).unwrap();
        assert!(!output.is_isosceles());
    }
    #[test]
    fn sides_may_be_floats() {
        let input = [0.5, 0.4, 0.5];
        let output = Triangle::build(input).unwrap();
        assert!(output.is_isosceles());
    }
}

#[cfg(test)]
mod scalene {
    use super::Triangle;

    #[test]
    fn no_sides_are_equal() {
        let input = [5, 4, 6];
        let output = Triangle::build(input).unwrap();
        assert!(output.is_scalene());
    }
    #[test]
    fn all_sides_are_equal() {
        let input = [4, 4, 4];
        let output = Triangle::build(input).unwrap();
        assert!(!output.is_scalene());
    }
    #[test]
    fn first_and_second_sides_are_equal() {
        let input = [4, 4, 3];
        let output = Triangle::build(input).unwrap();
        assert!(!output.is_scalene());
    }
    #[test]
    fn first_and_third_sides_are_equal() {
        let input = [3, 4, 3];
        let output = Triangle::build(input).unwrap();
        assert!(!output.is_scalene());
    }
    #[test]
    fn second_and_third_sides_are_equal() {
        let input = [4, 3, 3];
        let output = Triangle::build(input).unwrap();
        assert!(!output.is_scalene());
    }
    #[test]
    fn sides_may_be_floats() {
        let input = [0.5, 0.4, 0.6];
        let output = Triangle::build(input).unwrap();
        assert!(output.is_scalene());
    }
}

#[cfg(test)]
mod invalid {
    use super::Triangle;

    #[test]
    fn all_zero_sides_is_not_a_triangle() {
        let input = [0, 0, 0];
        let output = Triangle::build(input);
        assert!(output.is_none());
    }
    #[test]
    fn first_triangle_inequality_violation() {
        let input = [1, 1, 3];
        let output = Triangle::build(input);
        assert!(output.is_none());
    }
    #[test]
    fn second_triangle_inequality_violation() {
        let input = [1, 3, 1];
        let output = Triangle::build(input);
        assert!(output.is_none());
    }

    #[test]
    fn zero_triangle_inequality_violation() {
        let input = [0, 3, 3];
        let output = Triangle::build(input);
        assert!(output.is_none());
    }

    #[test]
    fn third_triangle_inequality_violation() {
        let input = [3, 1, 1];
        let output = Triangle::build(input);
        assert!(output.is_none());
    }
    #[test]
    fn may_not_violate_triangle_inequality() {
        let input = [7, 3, 2];
        let output = Triangle::build(input);
        assert!(output.is_none());
    }
}
