use proc_macro::TokenStream;
use syn::{Type, parse_macro_input};

mod run_suite;

#[proc_macro]
pub fn run_suite(input: TokenStream) -> TokenStream {
    // input is the solver the user implements
    let solver_type = parse_macro_input!(input as Type);
    run_suite::run_suite_impl(solver_type).into()
}
