use std::ops::{Add, Mul};

#[derive(Debug, Clone, PartialEq, Eq)]

pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Copy> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        self.0.len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0[0].len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        let mut res = vec![];
        for i in self.0.iter() {
            res.push(i[n]);
        }
        res
    }
}

impl<T> Mul for Matrix<T>
where
    T: Mul<Output = T> + Add<Output = T> + Copy,
{
    type Output = Option<Matrix<T>>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.number_of_cols() != rhs.number_of_rows() {
            return None;
        }
        let mut res = Vec::new();
        for i in 0..self.number_of_rows() {
            let mut new_row = Vec::new();
            for j in 0..rhs.number_of_cols() {
                let mut sum = self.0[i][0] * rhs.0[0][j];

                for k in 1..self.number_of_cols() {
                    sum = sum + self.0[i][k] * rhs.0[k][j];
                }
                new_row.push(sum);
            }
            res.push(new_row);
        }
        Some(Matrix(res))
    }
}
