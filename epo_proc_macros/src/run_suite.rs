use include_dir::{Dir, include_dir};
use proc_macro2::TokenStream;
use syn::Type;
use quote::quote;

static FILES: Dir = include_dir!("$CARGO_MANIFEST_DIR/benchmarks");

pub(crate) fn run_suite_impl(solver_type: Type) -> TokenStream {
    
    let mut names = Vec::new();
    for file in FILES.files() {
        let name = file.path().display().to_string();
        names.push(quote!(#name));
    }
    quote!(println!("{:?}", (#(#names),*));)
}