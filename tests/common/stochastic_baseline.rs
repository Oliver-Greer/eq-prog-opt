use crate::*;
use rand::RngExt;

/// TODO: Create pattern matching implementation for vars

#[derive(Default)]
pub struct SLS {
    rules: Vec<Rewrite>
}

impl Solver for SLS {
    fn new() -> Self {
        Default::default()
    }
    
    fn declare_sort(&mut self, sort: Sort) -> Result<()> {
        Ok(())
    }

    fn declare_function(&mut self, func: Function) -> Result<()> {
        Ok(())
    }

    fn declare_rewrite(&mut self, rewrite: Rewrite) -> Result<()> {
        self.rules.push(rewrite.clone());
        Ok(())
    }

    fn optimize(&mut self, optimize: Optimize) -> Result<Term> {
        todo!()
    }
}