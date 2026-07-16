mod common;
use common::egg_baseline::EggSolver;

#[cfg(test)]
mod tests {
    use epo::{self, Solver};

    use super::*;

    #[test]
    fn test_matmul() {
        let results = EggSolver::parse_file_and_run("../benchmarks/matmul.lisp").unwrap();
        for result in results {
            println!("Result: {}", result);
        }
    }
}
