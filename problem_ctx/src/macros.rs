//! This file provides helper macros that allow
//! benchmarks to automatically define and link problem context to the solver.

/// Hacky macro that creates a dummy function to
/// ensure dead code elim doesn't strip away the inventory submit.
/// This is unsustainable and the main reason to move away from inventory
/// and towards a registry struct. It requires a dummy call in epo for each impl file.
#[macro_export]
macro_rules! link {
    () => {
        pub fn dummy() {}
    };
}

/// Macro to register an analysis implementation with the solver.
/// It wraps the given function with a dynamic wrapper and uses
/// the compiler to infer the generic types.
/// Then it creates a isolated static Fn pointer and submits it to the inventory.
/// Right now this does not support multiple functions per term (it creates a len 1 slice).
/// Keeping everything static and using inventory is also a limiting factor,
/// as we cannot do string/path manip in this block. This should be a proc_macro.
#[macro_export]
macro_rules! register_analysis {
    ($term:expr, 2, $func_name:ident) => {
        const _: () = {
            fn infer<A: 'static, B: 'static, R: 'static>(
                f: fn(&A, &B) -> R,
                args: &[&dyn std::any::Any],
            ) -> Option<Box<dyn std::any::Any>> {
                let a: Option<&A> = if args.len() > 0 {
                    args[0].downcast_ref::<A>()
                } else {
                    None
                };
                let b: Option<&B> = if args.len() > 1 {
                    args[1].downcast_ref::<B>()
                } else {
                    None
                };
                if let Some(first) = a
                    && let Some(second) = b
                {
                    Some(Box::new(f(first, second)) as Box<dyn std::any::Any>)
                } else {
                    None
                }
            }

            fn wrapper(args: &[&dyn std::any::Any]) -> Option<Box<dyn std::any::Any>> {
                infer($func_name as fn(&_, &_) -> _, args)
            }
            static FN_PTR: problem_ctx::ErasedFn = wrapper;

            inventory::submit!(problem_ctx::Analysis {
                benchmark_name: file!(),
                term_name: $term,
                analysis: &[FN_PTR]
            });
        };
    };
    ($term:expr, 1, $func_name:ident) => {
        const _: () = {
            fn infer<A: 'static, R: 'static>(
                f: fn(&A) -> R,
                args: &[&dyn std::any::Any],
            ) -> Option<Box<dyn std::any::Any>> {
                let a: Option<&A> = if args.len() == 1 {
                    args[0].downcast_ref::<A>()
                } else {
                    None
                };
                if let Some(first) = a {
                    Some(Box::new(f(first)) as Box<dyn std::any::Any>)
                } else {
                    None
                }
            }

            fn wrapper(args: &[&dyn std::any::Any]) -> Option<Box<dyn std::any::Any>> {
                infer($func_name as fn(&_) -> _, args)
            }
            static FN_PTR: problem_ctx::ErasedFn = wrapper;

            inventory::submit!(problem_ctx::Analysis {
                benchmark_name: file!(),
                term_name: $term,
                analysis: &[FN_PTR]
            });
        };
    };
}

/// Macro to register a primitive implementation with the solver.
/// It wraps the given function with a dynamic wrapper and uses
/// the compiler to infer the generic types.
/// Then it creates a isolated static Fn pointer and submits it to the inventory.
/// This is a little cleaner than the analysis one, because there is no Fn pointer slice.
/// However, a proc_macro could still infer the number of args and the types, improving UX.
#[macro_export]
macro_rules! register_primitive {
    (1, $func_name:ident) => {
        const _: () = {
            fn infer<A: 'static, R: 'static>(
                f: fn(&A) -> R,
                args: &[&dyn std::any::Any],
            ) -> Option<Box<dyn std::any::Any>> {
                let a = if args.len() == 1 {
                    args[0].downcast_ref::<A>()
                } else {
                    None
                };

                if let Some(first) = a {
                    Some(Box::new(f(first)) as Box<dyn std::any::Any>)
                } else {
                    None
                }
            }

            fn wrapper(args: &[&dyn std::any::Any]) -> Option<Box<dyn std::any::Any>> {
                infer($func_name as fn(&_) -> _, args)
            }
            static FN_PTR: problem_ctx::ErasedFn = wrapper;

            inventory::submit!(problem_ctx::Primitive {
                benchmark_name: std::file!(),
                func_name: stringify!($func_name),
                primitive: &FN_PTR,
            });
        };
    };
    (2, $func_name:ident) => {
        const _: () = {
            fn infer<A: 'static, B: 'static, R: 'static>(
                f: fn(&A, &B) -> R,
                args: &[&dyn std::any::Any],
            ) -> Box<dyn std::any::Any> {
                let a: &A = args[0].downcast_ref::<A>().unwrap();
                let b: &B = args[1].downcast_ref::<B>().unwrap();

                Box::new(f(a, b)) as Box<dyn std::any::Any>
            }

            fn wrapper(args: &[&dyn std::any::Any]) -> Box<dyn std::any::Any> {
                infer($func_name as fn(&_, &_) -> _, args)
            }
            static FN_PTR: problem_ctx::problem_context::ErasedFn = wrapper;

            inventory::submit!(problem_ctx::problem_context::Primitive {
                benchmark_name: std::file!(),
                func_name: stringify!($func_name),
                primitive: &FN_PTR,
            });
        };
    };
}
