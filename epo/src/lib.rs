pub mod ast;
pub mod parse;

use std::collections::HashMap;
use std::any::Any;

use ast::*;

use macros::problem_context::ErasedFn;

pub type Result<T> = std::result::Result<T, String>;

pub trait AnalysisApi {
    fn get_analysis(&self) -> &HashMap<String, &'static [ErasedFn]>;
}

#[derive(Default, Clone)]
pub struct AnalysisMap {
    map: HashMap<String, &'static [ErasedFn]>,
}

impl AnalysisMap {
    pub fn evaluate_node(&self, name: &str, args: &[& dyn Any]) -> Option<Box<dyn Any>> {
        if let Some(funcs) = self.map.get(name) {
            funcs[0](&args)
        } else {
            None
        }
    }
}

pub struct PrimitiveMap {
    pub map: HashMap<String, &'static ErasedFn>,
}

pub trait Solver: Sized {
    fn new() -> Self;
    fn declare_sort(&mut self, sort: Sort) -> Result<()>;
    fn declare_constructor(&mut self, func: Constructor) -> Result<()>;
    fn declare_analysis(&mut self, analysis_map: AnalysisMap) -> Result<()>;
    fn declare_primitive(&mut self, primitive_map: PrimitiveMap) -> Result<()>;
    fn declare_rewrite(&mut self, rewrite: Rewrite) -> Result<()>;
    fn declare_cost(&mut self, costs: CostFunc) -> Result<()>;
    fn optimize(&mut self, optimize: Optimize) -> Result<Term>;
    fn benchmark(prog: Program) -> Result<Vec<Term>> {
        let mut solver = Self::new();

        for sort in prog.sorts {
            solver.declare_sort(sort)?;
        }
        for cons in prog.constructors {
            solver.declare_constructor(cons)?;
        }
        solver.declare_analysis(prog.analysis_impl)?;
        solver.declare_primitive(prog.primitive_impl)?;
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

    fn parse_str_and_run(src: &str) -> Result<Vec<Term>> {
        let prog: Program = Program::from_str(src)?;
        Self::benchmark(prog)
    }

    fn parse_file_and_run(path: &str) -> Result<Vec<Term>> {
        let prog: Program = Program::from_file(path)?;
        Self::benchmark(prog)
    }
}
