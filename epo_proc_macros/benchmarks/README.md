# Benchmark DSL and File Specification

This README provides a basis for understanding the benchmark file syntax and submission requirements, as well as what is expected of any implementation attempting to run the benchmark suite.

Benchmark files provide an implementation agnostic DSL for common program optimization tasks. The syntax is heavily inspired by Egglog's S-Expr syntax with the exclusion of features that "bake in" an EqSat approach.

This specification aims to provide features rich enough to express at least the following benchmarks:

- Math expression simplification
- Matrix chain multiplication
- Decompiling CAD into structured programs
- Proving inequalities in the Halide compiler

## Language Features

### Built-In Types

The language accepts three built-in types: Int, Float and String.

Integers are written as any sequence of digits 0 through 9 with no white-space in between and are internally represented as i64.

Floats are written as two integers with a period separating them, or as a single integer followed by a period. Internally these are doubles or f64.

Strings consist of any sequence of characters between two double quotes.

```
1 ;; this is a i64

1.5 ;; this is a f64

"this is a string" ;; this is a String
```

### Declarations

#### Sort

Custom types used in the benchmark files are declared with the `sort` keyword.

```
(sort Math)
```

Built-in types cannot be re-declared as sorts. These include `String`, `i64`, `f64`.

#### Constructors

Constructors define a function that maps a list of argument sorts to a return sort.

Constructor declarations take three arguments: the function name, the function argument types, and the function return type.

```
(constructor Name (ArgSort1 ArgSort2 ... ArgSortN) ReturnSort)
```

#### Rewrites

Rewrites are declared with the either the `rewrite` keyword or the `birewrite` keyword. The difference between the two is self evident. Enforcing this difference correctly is intentionally left up to the user. Rewrites at minimum require a left hand side and a right hand size, with an optional condition as a third argument prefaced by `:when`. Rewrites can also be named. Symbolic variables that can match any term are defined with a `?`.

TODO: Can primitives be referenced in RHS? Or only in the condition field?

```
;; A named rewrite
(rewrite MulCancel (Mul (Num 0) ?a) (Num 0))

;; A birewrite with a conditional term
(birewrite (Div ?x ?x) (Num 1) :when (Neq ?x 0))
```

TODO: Cost Functions. Let's discuss.

#### Optimize

The optimize declaration is usually the last part of the benchmark. It defines what term the solver should attempt to optimize using the defined rewrites and costs.

```
(sort Math)
(constructor Mul (Math Math) Math)
(constructor Add (Math Math) Math)
(constructor Num (i64) Math)
(constructor Var (String) Math)

(optimize (Mul (Num 0) (Add (Num 0) (Var "x"))))
```
