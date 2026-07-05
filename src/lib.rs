mod ast;
mod egg;
mod stochastic;
pub mod parse;

use ast::*;
pub use egg::EggSolver;
pub use stochastic::SLS;

pub type Result<T> = std::result::Result<T, String>;

pub trait Solver: Sized {
    fn new() -> Self;
    fn declare_sort(&mut self, sort: Sort) -> Result<()>;
    fn declare_function(&mut self, func: Function) -> Result<()>;
    fn declare_rewrite(&mut self, rewrite: Rewrite) -> Result<()>;
    fn optimize(&mut self, optimize: Optimize) -> Result<Term>;
    fn run_program(prog: Program) -> Result<Vec<Term>> {
        let mut solver = Self::new();

        for sort in prog.sorts {
            solver.declare_sort(sort)?;
        }
        for func in prog.funcs {
            solver.declare_function(func)?;
        }
        for rewrite in prog.rewrites {
            solver.declare_rewrite(rewrite)?;
        }

        let mut results: Vec<Term> = Vec::new();
        for optimize in prog.optimize {
            results.push(solver.optimize(optimize)?);
        }
        Ok(results)
    }

    fn parse_str_and_run(src: &str) -> Result<Vec<Term>> {
        let prog: Program = Program::from_str(src)?;
        Self::run_program(prog)
    }

    fn parse_file_and_run(path: &str) -> Result<Vec<Term>> {
        let prog: Program = Program::from_file(path)?;
        Self::run_program(prog)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math() {
        let results = crate::egg::EggSolver::parse_file_and_run("benchmarks/math.lisp").unwrap();
        for result in results {
            println!("Result: {}", result);
        }
    }
}
