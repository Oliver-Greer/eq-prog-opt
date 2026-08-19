use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
use syn::{File, Ident, Item, Type, parse_file};

// Scan for all benchmark files (both .lisp and .rs)
// parse the lisp files into a program
// get the correct .rs file from the implementation file field if it exists
pub(crate) fn run_suite_impl(solver_type: Type) -> TokenStream {
    let crate_dir: &Path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let benchmark_dir: PathBuf = crate_dir.join("benchmarks");

    let mut rs_files: HashMap<String, PathBuf> = HashMap::new();
    let mut lisp_files: HashMap<String, PathBuf> = HashMap::new();

    if let Ok(files) = fs::read_dir(benchmark_dir) {
        for file in files.flatten() {
            let path = file.path();
            let stem = path.file_stem().unwrap().to_str().unwrap();
            match path.extension().and_then(|s| s.to_str()) {
                Some("rs") => {
                    rs_files.insert(stem.to_string(), path.to_path_buf());
                }
                Some("lisp") => {
                    lisp_files.insert(stem.to_string(), path.to_path_buf());
                }
                _ => {}
            }
        }
    };

    let mut benchmark_modules: Vec<TokenStream> = Vec::new();
    let mut benchmark_calls: Vec<TokenStream> = Vec::new();

    for (idx, (stem, lisp_path)) in lisp_files.iter().enumerate() {
        let mod_ident: Ident = format_ident!("__benchmark_mod_{}", idx);

        // if a matching rs file exists declare it as a module
        if let Some(rs_path) = rs_files.get(stem) {
            let canonical_path: PathBuf = rs_path.canonicalize().unwrap();
            let mod_path: &str = canonical_path.to_str().unwrap();

            let file_str: String = std::fs::read_to_string(mod_path).unwrap_or_else(|err| {
                panic!("Failed to read file: {} with error: {}", mod_path, err)
            });
            let mod_file: File = parse_file(&file_str).unwrap_or_else(|err| {
                panic!("Failed to parse file: {} with error: {}", mod_path, err)
            });

            let mod_tokens: Vec<Item> = mod_file.items;

            benchmark_modules.push(quote! {
                mod #mod_ident {
                    #(#mod_tokens)*
                }
            });
        }

        // now include the lisp file as a string, parse it, and benchmark it
        let canonical_lisp_path: PathBuf = lisp_path.canonicalize().unwrap();
        let lisp_path_str: &str = canonical_lisp_path.to_str().unwrap();

        benchmark_calls.push(quote!{
            {
                let raw_lisp: &str = include_str!(#lisp_path_str);
                let parsed: epo::Program = epo::Program::from_str(raw_lisp)
                    .expect("Failed to parse!");
                let ctx = <#mod_ident::Context as Default>::default();
                let results = epo::Solver::<#mod_ident::Context>::benchmark(&mut solver, ctx, parsed);
            }
        });
    }

    let final_expanded = quote! {
        {
            #(#benchmark_modules)*

            let mut solver = <#solver_type as Default>::default();

            #(#benchmark_calls)*
        }
    };

    TokenStream::from(final_expanded)
}
