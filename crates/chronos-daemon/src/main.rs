fn main() {
    println!("chronos v0.1");
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
