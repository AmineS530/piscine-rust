use std::time::Instant;
/// Quick benchmark helper with adjustable iterations
pub fn bench<F, R>(name: &str, iterations: usize, func: F) -> R
where
    F: Fn() -> R,
    R: std::fmt::Debug + Clone,
{
    let mut last_result = None;
    let start = Instant::now();

    for _ in 0..iterations {
        last_result = Some(func());
    }

    let duration = start.elapsed();
    if let Some(result) = &last_result {
        println!(
            "{} -> last result: {:?} ({} iterations took {:?})",
            name, result, iterations, duration
        );
        result.clone()
    } else {
        panic!("No result returned from function");
    }
}
/*
    !on top
        include!("../../bench.rs"); // to run 
    !to run:
    ?example:
        bench("capitalize_first x1000", 1000, || capitalize_first(input1));

*/