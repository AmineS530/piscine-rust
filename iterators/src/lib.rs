#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {}

impl Collatz {
	pub fn new(n: u64) -> Self {}
}

pub fn collatz(n: u64) -> usize {}