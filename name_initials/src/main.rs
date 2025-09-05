use std::time::Instant;
// use dhat::Profiler; // Profiler struct

use name_initials::*; // your module containing `initials`

fn main() {
    // Start heap profiling
    // let _profiler = Profiler::new_heap(); // <- track heap allocations

    let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];

    // Start timing
    let start = Instant::now();

    let result = initials(names);

    let duration = start.elapsed();

    println!("Initials: {:?}", result);
    println!("Time elapsed: {:?}", duration);

    // When `_profiler` goes out of scope, dhat prints heap allocation statistics
}
