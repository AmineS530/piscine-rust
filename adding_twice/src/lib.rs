pub fn twice<T>(f: impl Fn(T) -> T) -> impl Fn(T) -> T {
    move |x| f(f(x))
}

pub fn add_curry(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}
