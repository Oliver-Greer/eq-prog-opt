use epo::context::ProblemContext;
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct MyContext {
	analysis_map: HashMap<String, Vec<fn(&mut Self, &[()]) -> ()>>,
}

impl ProblemContext for MyContext {
	type ArgType = ();
	type AnalysisTupleReturnType = ();

	fn get_analysis_map_mut(
		&mut self,
	) -> &mut HashMap<String, Vec<fn(&mut Self, &[Self::ArgType]) -> Self::ArgType>> {
		&mut self.analysis_map
	}

	fn build_analysis_map(
		&self,
	) -> HashMap<String, Vec<fn(&mut Self, &[Self::ArgType]) -> Self::ArgType>> {
		HashMap::new()
	}

	fn get_all_analysis_results_as_tuple(&self, _: &str) -> Self::AnalysisTupleReturnType {
		()
	}

	fn get_all_analysis_results(
		&self,
		_: &str,
	) -> Vec<fn(&mut Self, &[Self::ArgType]) -> Self::ArgType> {
		Vec::new()
	}
}

pub type Context = MyContext;