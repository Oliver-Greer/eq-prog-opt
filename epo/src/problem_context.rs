//! This file provides helper macros that allow
//! benchmarks to automatically link problem context.

use std::any::Any;

pub type ErasedFn = fn(&[&dyn Any]) -> Box<dyn Any>;
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

#[macro_export]
macro_rules! register_analysis {
    ($term:expr, 1, $func_name:ident) => {
        const _: () = {
            fn infer<A: 'static, R: 'static>(
                f: fn(&A) -> R,
                args: &[&dyn std::any::Any],
            ) -> Box<dyn std::any::Any> {
                let a: &A = args[0].downcast_ref::<A>().unwrap();

                Box::new(f(a)) as Box<dyn std::any::Any>
            }

            fn wrapper(args: &[&dyn std::any::Any]) -> Box<dyn std::any::Any> {
                infer($func_name as fn(&_) -> _, args)
            }
            static FN_PTR: epo::problem_context::ErasedFn = wrapper;

            inventory::submit!(epo::problem_context::Analysis {
                benchmark_name: file!(),
                term_name: $term,
                analysis: &[FN_PTR]
            });
        };
    };
    ($term:expr, 2, $func_name:ident) => {
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
            static FN_PTR: epo::problem_context::ErasedFn = wrapper;

            inventory::submit!(epo::problem_context::Analysis {
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
            ) -> Box<dyn std::any::Any> {
                let a: &A = args[0].downcast_ref::<A>().unwrap();

                Box::new(f(a)) as Box<dyn std::any::Any>
            }

            fn wrapper(args: &[&dyn std::any::Any]) -> Box<dyn std::any::Any> {
                infer($func_name as fn(&_) -> _, args)
            }
            static FN_PTR: epo::problem_context::ErasedFn = wrapper;

            inventory::submit!(epo::problem_context::Primitive {
                benchmark_name: file!(),
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
            static FN_PTR: epo::problem_context::ErasedFn = wrapper;

            inventory::submit!(epo::problem_context::Primitive {
                benchmark_name: file!(),
                func_name: stringify!($func_name),
                primitive: &FN_PTR,
            });
        };
    };
}
