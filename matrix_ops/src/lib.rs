use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Eq)]

pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Add<Output = T> + Copy> Add for Matrix<T> {
    type Output = Option<Matrix<T>>;
    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() || self.0.is_empty() {
            return None;
        }
        let mut res = Vec::new();

        for (v1, v2) in self.0.iter().zip(other.0.iter()) {
            if v1.len() != v2.len() {
                return None;
            }
            let mut temp = Vec::new();
            for (vv1, vv2) in v1.iter().zip(v2.iter()) {
                temp.push(*vv1 + *vv2);
            }
            res.push(temp);
        }
        Some(Matrix(res))
    }
}

impl<T: Sub<Output = T> + Copy> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() || self.0.is_empty() {
            return None;
        }
        let mut res = Vec::new();

        for (v1, v2) in self.0.iter().zip(rhs.0.iter()) {
            if v1.len() != v2.len() {
                return None;
            }
            let mut temp = Vec::new();
            for (vv1, vv2) in v1.iter().zip(v2.iter()) {
                temp.push(*vv1 - *vv2);
            }
            res.push(temp);
        }
        Some(Matrix(res))
    }
}
