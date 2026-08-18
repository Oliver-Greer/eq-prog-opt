//! Public interface for the benchmarking library.
//! 
//! We expose ast terms, bridges for analysis and primitives, 
//! and the main solver trait.

pub mod ast;
pub mod context;
pub(crate) mod parse;

use ast::*;

use crate::context::Context;

// Standardized result type for this lib.
pub type Result<T> = std::result::Result<T, String>;

// We restrict integers to i64, otherwise users could use
// i32 and downcasts would fail. Better to make it universal.
pub type IntType = i64;

// We restrict floats to f64, otherwise users could use
// f32 and downcasts would fail. Better to make it universal.
pub type FloatType = f64;

// Strings are renamed to StringType to avoid confusion with str.
pub type StringType = String;


/// Solver is the main trait any program optimizer 
/// trying to run the benchmarks must implement.
/// It requires function definitions for translating the benchmark 
/// ast into whatever the solvers internal representation may be.
pub trait Solver<C: Context>: Sized {
    /// Creates a new solver.
    fn new() -> Self;

    /// Declares an analysis for the solver. The AnalysisBridge is already 
    /// filled in. The job of the solver is to translate it into whatever 
    /// internal representation is needed.
    fn declare_analysis(&mut self) -> Result<()>;

    /// Declares primitives for the solver. The PrimitiveBridge is already 
    /// filled in. The job of the solver is to translate it into whatever 
    /// internal representation is needed.
    fn declare_primitives(&mut self) -> Result<()>;

    /// Declares a sort declaration.
    fn declare_sort(&mut self, sort: Sort) -> Result<()>;

    /// Declares a constructor for nodes in the ast.
    fn declare_constructor(&mut self, func: Constructor) -> Result<()>;

    /// Declares both rewrites and birewrites for the solver.
    /// Rewrites can contain conditions, which must be linked with 
    /// the functions in the PrimitiveBridge.
    fn declare_rewrite(&mut self, rewrite: Rewrite) -> Result<()>;

    /// Declare a cost function for the solver.
    fn declare_cost(&mut self, costs: CostFunc) -> Result<()>;

    /// Declare an optimize declaration for the solver.
    fn optimize(&mut self, optimize: Optimize) -> Result<Term>;

    /// Declares all declarations for the solver, initializing it's internal state.
    /// Then benchmark the solver on the given optimize declarations.
    fn benchmark(prog: Program) -> Result<Vec<Term>> {
        let mut solver = Self::new();

        for sort in prog.sorts {
            solver.declare_sort(sort)?;
        }

        for cons in prog.constructors {
            solver.declare_constructor(cons)?;
        }
        
        for rewrite in prog.rewrites {
            solver.declare_rewrite(rewrite)?;
        }
        for cost in prog.costfuncs {
            solver.declare_cost(cost)?;
        }

        let mut results: Vec<Term> = Vec::new();
        for optimize in prog.optimize {
            // TODO: Wrap optimize call to collect metrics
            results.push(solver.optimize(optimize)?);
        }
        Ok(results)
    }

    /// Parse a benchmark file given as a string and benchmark it.
    fn parse_str_and_run(src: &str) -> Result<Vec<Term>> {
        let prog: Program = Program::from_str(src)?;
        Self::benchmark(prog)
    }

    /// Parse a benchmark file from the filepath, and benchmark it.
    fn parse_file_and_run(path: &str) -> Result<Vec<Term>> {
        let prog: Program = Program::from_file(path)?;
        Self::benchmark(prog)
    }
}
