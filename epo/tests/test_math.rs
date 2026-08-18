mod common;
use common::egg_baseline::EggSolver;
use epo::run_suite;

#[cfg(test)]
mod tests {
    use epo::{self, Solver};

    use super::*;

    #[test]
    fn test_math() {
        run_suite![EggSolver];
        //let results = EggSolver::parse_file_and_run("../epo/src/benchmarks/test.lisp").unwrap();
        // for result in results {
        //     println!("Result: {}", result);
        // }
    }
}
