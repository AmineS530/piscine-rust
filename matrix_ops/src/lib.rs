mod matrix;
mod scalar;

use matrix::*;

impl<T>std::ops::Add for Matrix<T> {
    type Output = T;
    fn add(self, other:Self) -> Self::Output {
       
    }
}

impl<T> std::ops::Sub for Matrix<T> {
    type Output = T ;
    fn sub(self, rhs: Self) -> Self::Output {
        
    }
}