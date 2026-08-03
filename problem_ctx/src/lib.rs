//! This library contains the datatypes and macros for defining rust benchmark
//! implementations. This includes conditional functions and analysis.
//!
//! The only reason this is a separate crate is because eventually this crate should
//! be converted to use proc_macros which would save some user headache. The main advantage
//! is that the macro could look at the types and number of args at compile time, which
//! reduces boilerplate and improves UX.

pub mod macros;

use std::any::Any;

/// ErasedFn is a type erased dynamic function pointer.
/// Returns Option because analysis are defined on partial order lattices.
pub type ErasedFn = fn(&[&dyn Any]) -> Option<Box<dyn Any>>;

/// Generic representation of an analysis for a given term.
/// Fields have static lifetimes because we are using the
/// inventory crate for linker time integration.
/// This can and should be changed at a later time.
pub struct Analysis {
    /// The name of the benchmark this term is in.
    /// Important for properly scoping analysis when running multiple benchmarks.
    pub benchmark_name: &'static str,

    /// The name of the term this analysis is for.
    pub term_name: &'static str,

    /// A slice of type erased function pointer references.
    /// Any given term can have multiple analysis which have each
    /// can different arguments and/or return types. This requires dynamic dispatch.
    pub analysis: &'static [ErasedFn],
}

/// Generic representation of a primitive (conditional function typically)
/// Fields have static lifetimes because we are using the
/// inventory crate for linker time integration.
/// This can and should be changed at a later time.
pub struct Primitive {
    /// Benchmark name containing this function. Used for scoping.
    pub benchmark_name: &'static str,

    /// The function name as a string.
    pub func_name: &'static str,

    /// A single type erased function pointer for dynamic evaluation.
    /// Unlike in the analysis case we are not storing different
    /// function signatures in a vector, so there may be a way
    /// around dynamic dispatch here. However for now this makes it easier
    /// for the solver to not worry about the types they pass in.
    pub primitive: &'static ErasedFn,
}

// Register these two with the linker inventory.
inventory::collect!(Analysis);
inventory::collect!(Primitive);
