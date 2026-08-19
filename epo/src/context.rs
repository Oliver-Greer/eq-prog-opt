use std::collections::HashMap;
use std::fmt::Debug;

pub type PrimitiveFn =
    fn(&[i64]) -> i64;

pub type CondFn =
    fn(&[i64]) -> bool;

pub type NodeName = String;

#[derive(Clone, Debug)]
pub struct Primitive {
    pub f: PrimitiveFn,
}

#[derive(Clone, Debug)]
pub struct Cond {
    pub f: CondFn,
}

pub trait ProblemContext: Sized + Default + Clone + Debug {

    fn set_analysis_map(&mut self) {
        let new_analysis = self.build_analysis_map();
        self.get_analysis_map_mut().extend(new_analysis);
    }

    /// Return a mutable reference to the internal analysis map.
    /// Enforces that the trait user defines a hashmap field satisfying this return type.
    fn get_analysis_map_mut(&mut self) -> &mut HashMap<NodeName, Primitive>;

    /// Return a immutable copy of the internal analysis map.
    /// Enforces that the trait user defines a hashmap field satisfying this return type.
    fn get_analysis_map(self) -> HashMap<NodeName, Primitive>;

    /// Define the analysis as a hashmap from [`NodeName`] to a vector of [`Primitive`]
    fn build_analysis_map(&self) -> HashMap<NodeName, Primitive>;

    fn set_condition_map(&mut self) {
        let new_condition = self.build_condition_map();
        self.get_condition_map_mut().extend(new_condition);
    }

    /// Return a mutable reference to the internal condition map.
    /// Enforces that the trait user defines a hashmap field satisfying this return type.
    fn get_condition_map_mut(&mut self) -> &mut HashMap<String, Cond>;

    /// Return a immutable copy of the internal condition map.
    /// Enforces that the trait user defines a hashmap field satisfying this return type.
    fn get_condition_map(self) -> HashMap<String, Cond>;

    /// Define the condition as a hashmap from [`NodeName`] to a vector of [`Cond`]
    fn build_condition_map(&self) -> HashMap<String, Cond>;
}
