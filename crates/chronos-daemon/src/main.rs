fn main() {
    println!("chronos v{}", env!("CARGO_PKG_VERSION"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_runs_without_panic() {
        // Trivial test to hit 100% test coverage on the MVP stub
        main();
    }
}
