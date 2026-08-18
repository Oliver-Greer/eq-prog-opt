use std::collections::HashMap;

pub type PrimitiveFn<C: Context> = fn(&mut C, &[C::ArgType]) -> C::ArgType;

pub type NodeName = String;

pub trait Context: Sized {
    type ArgType;
    type AnalysisTupleReturnType;
    
    fn set_analysis_map(&mut self) {
        let new_analysis = self.build_analysis_map();
        self.get_analysis_map_mut().extend(new_analysis);
    }

    /// Return a mutable reference to the internal analysis map.
    /// Enforces that the trait user defines a hashmap field satisfying this return type.
    fn get_analysis_map_mut(&mut self) -> &mut HashMap<NodeName, Vec<PrimitiveFn<Self>>>;


    /// Define the analysis as a hashmap from [`NodeName`] to a vector of [`PrimitiveFn`] 
    fn build_analysis_map(&self) -> HashMap<NodeName, Vec<PrimitiveFn<Self>>>;

    
    /// Manually call all analysis functions for a given [`NodeName`] and return the results in a tuple.
    /// This is only used when the solver requires multiple analysis to be combined into one.
    fn get_all_analysis_results_as_tuple(&self, node_name: &str) -> Self::AnalysisTupleReturnType;

    
    /// Return a vector of analysis function pointers for a given node name.
    fn get_all_analysis_results(&self, node_name: &str) -> Vec<PrimitiveFn<Self>>;
}