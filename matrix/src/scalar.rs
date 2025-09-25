use std::ops::{Add, Div, Mul, Sub};

pub trait Scalar:
    Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Sized
{
    type Item;
    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}

#[macro_export]
macro_rules! Scalar {
    ($t:ty) => {
        impl Scalar for $t {
            type Item = $t;

            fn zero() -> Self::Item {
                0 as $t
            }

            fn one() -> Self::Item {
                1 as $t
            }
        }
    };
}
Scalar!(u32);
Scalar!(u64);
Scalar!(i32);
Scalar!(i64);
Scalar!(f32);
Scalar!(f64);
