// include! literally copies the file content here
include!("../../bench.rs");

use capitalizing::*;

fn main() {
    let input1 = "joe is missing";
    let input2 = "heLLo THere";
    // Call the benchmark
    bench("capitalize_first x1000", 1000, || capitalize_first(input1));
    bench("change_case x1000", 1000, || change_case(input2));
    bench("title_case x1000", 1000, || title_case("hello my\t\tname is carl"));
}