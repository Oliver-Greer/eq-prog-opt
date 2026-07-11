mod common;
use common::egg_baseline::EggSolver;

#[cfg(test)]
mod tests {
    use epo::{self, Solver};

    use super::*;

    #[test]
    fn test_math() {
        let results = EggSolver::parse_file_and_run("../benchmarks/math.lisp").unwrap();
        for result in results {
            println!("Result: {}", result);
        }
    }
}
