use epo::context::{ProblemContext, Primitive, Cond};
use epo::{IntType, primitive, condition};
use std::collections::HashMap;

#[derive(Default, Clone, Debug)]
pub struct MyContext {
	analysis_map: HashMap<String, Primitive>,
    condition_map: HashMap<String, Cond>,
}

impl MyContext {

    pub fn fold_num(a: IntType) -> IntType {
        a
    }

    pub fn fold_add(a: IntType, b: IntType) -> IntType {
        a + b
    }

    pub fn fold_sub(a: IntType, b: IntType) -> IntType {
        a - b
    }

    pub fn fold_mul(a: IntType, b: IntType) -> IntType {
        a * b
    }

    pub fn is_not_zero(a: IntType) -> bool {
        a != 0
    }
}

impl ProblemContext for MyContext {

	fn get_analysis_map_mut(
		&mut self,
	) -> &mut HashMap<String, Primitive> {
		&mut self.analysis_map
	}

    fn get_analysis_map(
		self,
	) -> HashMap<String, Primitive> {
		self.analysis_map
	}

	fn build_analysis_map(
		&self,
	) -> HashMap<String, Primitive> {
		HashMap::from([
            ("Num".to_string(), primitive![1, Self::fold_num]),
            ("Add".to_string(), primitive![2, Self::fold_add]),
            ("Mul".to_string(), primitive![2, Self::fold_mul]),
            ("Sub".to_string(), primitive![2, Self::fold_sub]),
        ])
	}

    fn get_condition_map_mut(&mut self) -> &mut HashMap<String, Cond> {
        &mut self.condition_map
    }

    fn get_condition_map(self) -> HashMap<String, Cond> {
        self.condition_map
    }

    fn build_condition_map(&self) -> HashMap<String, Cond> {
        HashMap::from([
            ("IsNotZero".to_string(), condition![1, Self::is_not_zero])
        ])
    }
}

pub type Context = MyContext;