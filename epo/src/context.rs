use std::collections::HashMap;
use std::fmt::Debug;

pub type PrimitiveFn<C> =
    fn(&mut C, &[<C as ProblemContext>::ArgType]) -> <C as ProblemContext>::ArgType;

pub type CondFn<C> =
    fn(&mut C, &[<C as ProblemContext>::ArgType]) -> bool;

pub type NodeName = String;

#[derive(Clone, Debug)]
pub struct Primitive<C: ProblemContext> {
    pub f: PrimitiveFn<C>,
}

#[derive(Clone, Debug)]
pub struct Cond<C: ProblemContext> {
    pub f: CondFn<C>,
}

pub trait ProblemContext: Sized + Default + Clone + Debug {
    type ArgType;
    type AnalysisTupleReturnType: Debug;

    fn set_analysis_map(&mut self) {
        let new_analysis = self.build_analysis_map();
        self.get_analysis_map_mut().extend(new_analysis);
    }

    /// Return a mutable reference to the internal analysis map.
    /// Enforces that the trait user defines a hashmap field satisfying this return type.
    fn get_analysis_map_mut(&mut self) -> &mut HashMap<NodeName, Vec<Primitive<Self>>>;

    /// Define the analysis as a hashmap from [`NodeName`] to a vector of [`PrimitiveFn`]
    fn build_analysis_map(&self) -> HashMap<NodeName, Vec<Primitive<Self>>>;

    fn set_condition_map(&mut self) {
        let new_condition = self.build_condition_map();
        self.get_condition_map_mut().extend(new_condition);
    }

    /// Return a mutable reference to the internal condition map.
    /// Enforces that the trait user defines a hashmap field satisfying this return type.
    fn get_condition_map_mut(&mut self) -> &mut HashMap<NodeName, Cond<Self>>;

    /// Define the condition as a hashmap from [`NodeName`] to a vector of [`Cond`]
    fn build_condition_map(&self) -> HashMap<NodeName, Cond<Self>>;

    /// Manually call all analysis functions for a given [`NodeName`] and return the results in a tuple.
    /// This is only used when the solver requires multiple analysis to be combined into one.
    fn get_all_analysis_results_as_tuple(
        &mut self,
        node_name: &str,
        args: &[Self::ArgType],
    ) -> Option<Self::AnalysisTupleReturnType>;

    fn get_condition_result(
        &mut self, 
        node_name: &str, 
        args: &[Self::ArgType]
    ) -> Option<bool>;

}
