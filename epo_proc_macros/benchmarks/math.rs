use epo::context::{ProblemContext, Primitive, Cond};
use epo::{IntType, primitive, condition};
use std::collections::HashMap;

#[derive(Default, Clone, Debug)]
pub struct MyContext {
	analysis_map: HashMap<String, Vec<Primitive<Self>>>,
    condition_map: HashMap<String, Cond<Self>>,
}

impl MyContext {

    pub fn fold_num(&mut self, a: IntType) -> IntType {
        a
    }

    pub fn fold_add(&mut self, a: IntType, b: IntType) -> IntType {
        a + b
    }

    pub fn fold_sub(&mut self, a: IntType, b: IntType) -> IntType {
        a - b
    }

    pub fn fold_mul(&mut self, a: IntType, b: IntType) -> IntType {
        a * b
    }

    pub fn is_not_zero(&mut self, a: IntType) -> bool {
        a != 0
    }
}

impl ProblemContext for MyContext {
	type ArgType = IntType;
	type AnalysisTupleReturnType = IntType;

	fn get_analysis_map_mut(
		&mut self,
	) -> &mut HashMap<String, Vec<Primitive<Self>>> {
		&mut self.analysis_map
	}

	fn build_analysis_map(
		&self,
	) -> HashMap<String, Vec<Primitive<Self>>> {
		HashMap::from([
            ("Num".to_string(), vec![primitive![1, Self::fold_num]]),
            ("Add".to_string(), vec![primitive![2, Self::fold_add]]),
            ("Mul".to_string(), vec![primitive![2, Self::fold_mul]]),
            ("Sub".to_string(), vec![primitive![2, Self::fold_sub]]),
        ])
	}

	fn get_all_analysis_results_as_tuple(&mut self, node_name: &str, args: &[Self::ArgType]) 
        -> Option<Self::AnalysisTupleReturnType> 
    {
		if let Some(func) = self.analysis_map.get(node_name) {
			let prim_fn = func[0].f;
			Some(prim_fn(self, args))
		} else {
			None
		}
	}

    fn get_condition_map_mut(&mut self) -> &mut HashMap<String, Cond<Self>> {
        &mut self.condition_map
    }

    fn build_condition_map(&self) -> HashMap<String, Cond<Self>> {
        HashMap::from([
            ("IsNotZero".to_string(), condition![1, Self::is_not_zero])
        ])
    }
    
    fn get_condition_result(
        &mut self, 
        node_name: &str, 
        args: &[Self::ArgType]
    ) -> Option<bool>
    {
        if let Some(func) = self.condition_map.get(node_name) {
			let cond_fn = func.f;
			Some(cond_fn(self, args))
		} else {
			None
		}
    }
}

pub type Context = MyContext;