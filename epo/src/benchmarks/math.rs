use problem_ctx::{IntType, link, register_analysis, register_primitive};

fn fold_add(a: &IntType, b: &IntType) -> IntType {
    a + b
}

fn fold_num(a: &IntType) -> IntType {
    *a
}

register_analysis!["Add", 2, fold_add];
register_analysis!["Num", 1, fold_num];

fn is_non_zero(a: &IntType) -> bool {
    *a != 0
}

register_primitive![1, is_non_zero];
link!();
