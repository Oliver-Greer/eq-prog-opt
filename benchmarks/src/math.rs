use macros::{register_analysis, register_primitive, link};

fn fold_add(a: &i64, b: &i64) -> i64 {
    a + b
}

fn fold_num(a: &i64) -> i64 {
    *a
}

fn is_non_zero(a: &i64) -> bool {
    *a != 0
}

register_analysis!["Add", 2, fold_add];
register_analysis!["Num", 1, fold_num];

register_primitive![1, is_non_zero];
link!();