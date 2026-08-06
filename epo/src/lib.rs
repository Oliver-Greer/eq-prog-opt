//! Public interface for the benchmarking library.
//! 
//! We expose ast terms, bridges for analysis and primitives, 
//! and the main solver trait.

pub mod ast;
pub(crate) mod parse;

use std::any::Any;
use std::collections::HashMap;

use ast::*;

use problem_ctx::ErasedFn;

pub type Result<T> = std::result::Result<T, String>;

// Hashmaps for problem context (analysis and primitives)
pub type AnalysisMap = HashMap<String, &'static [ErasedFn]>;
pub type PrimitiveMap = HashMap<String, &'static ErasedFn>;


/// AnalysisBridge is the struct that let's solvers interface with 
/// the analysis context. The context is a hashmap from terms to Fn pointers.
/// Clone is required because of the static lifetimes and shared ownership, 
/// which can and should be changed.
#[derive(Default, Clone)]
pub struct AnalysisBridge {
    /// Map from term name to slice of Fn pointers.
    map: AnalysisMap,
}

impl AnalysisBridge {
    /// Evaluates a function on a given term.
    /// Eventually this needs to loop through all functions in args and return all results.
    pub fn evaluate_term(&self, name: &str, args: &[&dyn Any]) -> Option<Box<dyn Any>> {
        if let Some(functions) = self.map.get(name) {
            functions[0](&args)
        } else {
            None
        }
    }
}


/// PrimitiveBridge is the struct that let's solvers interface with 
/// the primitive context. The context is a hashmap from terms to a Fn pointer.
/// Clone is required because of the static lifetimes and shared ownership, 
/// which can and should be changed.
#[derive(Default, Clone)]
pub struct PrimitiveBridge {
    /// Map from function name to Fn pointer.
    map: PrimitiveMap,
}

impl PrimitiveBridge {
    /// Evaluates a function with a given name.
    pub fn evaluate_primitive(&self, name: &str, args: &[&dyn Any]) -> Option<Box<dyn Any>> {
        if let Some(function) = self.map.get(name) {
            function(&args)
        } else {
            None
        }
    }
}


/// Solver is the main trait any program optimizer 
/// trying to run the benchmarks must implement.
/// It requires function definitions for translating the benchmark 
/// ast into whatever the solvers internal representation may be.
pub trait Solver: Sized {
    /// Creates a new solver.
    fn new() -> Self;

    /// Declares an analysis for the solver. The AnalysisBridge is already 
    /// filled in. The job of the solver is to translate it into whatever 
    /// internal representation is needed.
    fn declare_analysis(&mut self, analysis_map: AnalysisBridge) -> Result<()>;

    /// Declares primitives for the solver. The PrimitiveBridge is already 
    /// filled in. The job of the solver is to translate it into whatever 
    /// internal representation is needed.
    fn declare_primitives(&mut self, primitive_map: PrimitiveBridge) -> Result<()>;

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

        solver.declare_analysis(prog.analysis_bridge)?;
        solver.declare_primitives(prog.primitive_bridge)?;

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
