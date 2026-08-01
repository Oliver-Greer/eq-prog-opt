//! This file provides helper macros that allow
//! benchmarks to automatically link problem context.

use std::any::Any;

pub type ErasedFn = fn(&[&dyn Any]) -> Option<Box<dyn Any>>;
pub struct Analysis {
    pub benchmark_name: &'static str,
    pub term_name: &'static str,
    pub analysis: &'static [ErasedFn],
}

pub struct Primitive {
    pub benchmark_name: &'static str,
    pub func_name: &'static str,
    pub primitive: &'static ErasedFn,
}

inventory::collect!(Analysis);
inventory::collect!(Primitive);

/// Hacky macro that creates a dummy function to
/// ensure dead code elim doesnt strip away the inventory submit
#[macro_export]
macro_rules! link {
    () => {
        pub fn dummy() {}
    };
}

#[macro_export]
macro_rules! register_analysis {
    ($term:expr, 2, $func_name:ident) => {
        const _: () = {
            fn infer<A: 'static, B: 'static, R: 'static>(
                f: fn(&A, &B) -> R,
                args: &[&dyn std::any::Any],
            ) -> Option<Box<dyn std::any::Any>> {
                let a: Option<&A> = if args.len() > 0 {args[0].downcast_ref::<A>()} else {None};
                let b: Option<&B> = if args.len() > 1 {args[1].downcast_ref::<B>()} else {None};
                if let Some(first) = a && let Some(second) = b {
                    Some(Box::new(f(first, second)) as Box<dyn std::any::Any>)
                } else {
                    None
                }
            }

            fn wrapper(args: &[&dyn std::any::Any]) -> Option<Box<dyn std::any::Any>> {
                infer($func_name as fn(&_, &_) -> _, args)
            }
            static FN_PTR: macros::problem_context::ErasedFn = wrapper;

            inventory::submit!(macros::problem_context::Analysis {
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
                let a: Option<&A> = if args.len() == 1 {args[0].downcast_ref::<A>()} else {None};
                if let Some(first) = a {
                    Some(Box::new(f(first)) as Box<dyn std::any::Any>)
                } else {
                    None
                }
            }

            fn wrapper(args: &[&dyn std::any::Any]) -> Option<Box<dyn std::any::Any>> {
                infer($func_name as fn(&_) -> _, args)
            }
            static FN_PTR: macros::problem_context::ErasedFn = wrapper;

            inventory::submit!(macros::problem_context::Analysis {
                benchmark_name: file!(),
                term_name: $term,
                analysis: &[FN_PTR]
            });
        };
    };
}

#[macro_export]
macro_rules! register_primitive {
    (1, $func_name:ident) => {
        const _: () = {
            fn infer<A: 'static, R: 'static>(
                f: fn(&A) -> R,
                args: &[&dyn std::any::Any],
            ) -> Option<Box<dyn std::any::Any>> {
                let a = if args.len() == 1 {args[0].downcast_ref::<A>()} else {None};

                if let Some(first) = a{
                    Some(Box::new(f(first)) as Box<dyn std::any::Any>)
                } else {
                    None
                }
            }

            fn wrapper(args: &[&dyn std::any::Any]) -> Option<Box<dyn std::any::Any>> {
                infer($func_name as fn(&_) -> _, args)
            }
            static FN_PTR: macros::problem_context::ErasedFn = wrapper;

            inventory::submit!(macros::problem_context::Primitive {
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
            static FN_PTR: macros::problem_context::ErasedFn = wrapper;

            inventory::submit!(macros::problem_context::Primitive {
                benchmark_name: std::file!(),
                func_name: stringify!($func_name),
                primitive: &FN_PTR,
            });
        };
    };
}
