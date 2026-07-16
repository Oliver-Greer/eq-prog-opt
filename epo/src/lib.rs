pub mod ast;
pub mod parse;

use ast::*;

pub type Result<T> = std::result::Result<T, String>;

pub trait Solver: Sized {
    fn new() -> Self;
    fn declare_sort(&mut self, sort: Sort) -> Result<()>;
    fn declare_constructor(&mut self, func: Constructor) -> Result<()>;
    fn declare_primitive(&mut self, func: Primitive) -> Result<()>;
    fn declare_rewrite(&mut self, rewrite: Rewrite) -> Result<()>;
    fn optimize(&mut self, optimize: Optimize) -> Result<Term>;
    fn benchmark(prog: Program) -> Result<Vec<Term>> {
        let mut solver = Self::new();

        for sort in prog.sorts {
            solver.declare_sort(sort)?;
        }
        for cons in prog.constructors {
            solver.declare_constructor(cons)?;
        }
        for prim in prog.primitives {
            solver.declare_primitive(prim)?;
        }
        for rewrite in prog.rewrites {
            solver.declare_rewrite(rewrite)?;
        }

        let mut results: Vec<Term> = Vec::new();
        for optimize in prog.optimize {
            // TODO: Wrap optimize call to collect metrics
            results.push(solver.optimize(optimize)?);
        }
        Ok(results)
    }

    fn parse_str_and_run(src: &str) -> Result<Vec<Term>> {
        let prog: Program = Program::from_str(src)?;
        Self::benchmark(prog)
    }

    fn parse_file_and_run(path: &str) -> Result<Vec<Term>> {
        let prog: Program = Program::from_file(path)?;
        Self::benchmark(prog)
    }
}
