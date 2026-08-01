use macros::{register_analysis, register_primitive, link};

fn fold_add(a: &i64, b: &i64) -> i64 {
    a + b
}

fn is_non_zero(a: &i64) -> bool {
    *a != 0
}

register_analysis!["Add", 2, fold_add];
register_primitive![1, is_non_zero];
link!();