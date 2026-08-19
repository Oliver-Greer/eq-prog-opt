//! Public interface for the benchmarking library.
//!
//! We expose ast terms, bridges for analysis and primitives,
//! and the main solver trait.

pub mod ast;
pub mod context;
pub mod macros;
pub(crate) mod parse;

pub use ast::*;

use crate::context::ProblemContext;

// Standardized result type for this lib.
pub type Result<T> = std::result::Result<T, String>;

// We restrict integers to i64, otherwise users could use
// i32 and functions would fail. Better to make it universal.
pub type IntType = i64;

// We restrict floats to f64, otherwise users could use
// f32 and functions would fail. Better to make it universal.
pub type FloatType = f64;

// Strings are renamed to StringType to avoid confusion with str.
pub type StringType = String;

/// Solver is the main trait any program optimizer
/// trying to run the benchmarks must implement.
/// It requires function definitions for translating the benchmark
/// ast into whatever the solvers internal representation may be.
pub trait Solver<C: ProblemContext>: Sized {
    /// Creates a new solver.
    fn new() -> Self;

    /// Sets up solver fields.
    /// Generally this means analysis and primitive maps.
    fn init_solver(&mut self, context: C) -> Result<()>;

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
    fn benchmark(&mut self, prog: Program, context: C) -> Result<Vec<Term>> {
        // clone is fine here because we don't care about
        // anything other than the optimize calls
        self.init_solver(context.clone())?;

        for sort in prog.sorts {
            self.declare_sort(sort)?;
        }

        for cons in prog.constructors {
            self.declare_constructor(cons)?;
        }

        for rewrite in prog.rewrites {
            self.declare_rewrite(rewrite)?;
        }
        for cost in prog.costfuncs {
            self.declare_cost(cost)?;
        }

        let mut results: Vec<Term> = Vec::new();
        for optimize in prog.optimize {
            // TODO: Wrap optimize call to collect metrics
            results.push(self.optimize(optimize)?);
        }
        Ok(results)
    }
}
