#![deny(clippy::pedantic)]

struct Fib(u128, u128);

impl Fib {
    fn new() -> Self {
        Fib(0, 1)
    }
}

impl Iterator for Fib {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        *self = Fib(self.1, self.0 + self.1);
        Some(self.0)
    }
}

fn main() {
    let last = 20;
    println!(
        "fib({}) result: {:?}",
        last,
        Fib::new().take(last).collect::<Vec<u128>>()
    );
}
