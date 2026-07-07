# Benchmark DSL and File Specification

This README provides a basis for understanding the benchmark file syntax and format, as well as what is expected of any implementation attempting to run the benchmark suite.

Benchmark files provide an implementation agnostic DSL for common program optimization tasks. The syntax is heavily inspired by Egglog's S-Expr syntax with the exclusion of features that "bake in" an EqSat approach.

This specification aims to provide features rich enough to express at least the following benchmarks:

- Math expression simplification
- Matrix chain multiplication
- Decompiling CAD into structured programs
- Proving inequalities in the Halide compiler

## Language Keywords

### Primitives

The language accepts three primitive types: Int, String, and Bool.

Integers are written as any sequence of digits 0 through 9, with no whitespace in between.

Strings consist of any sequence of characters between two double quotes.

There are only two acceptable Bools in the DSL: True and False. These are reserved keywords and cannot be used as user-defined sorts.

```
1

"this is a string"

True

False
```

### Sorts

Types used in the benchmark files are declared with the `sort` keyword.

```
(sort Math)
```
Primitive sorts are built-in and cannot be re-declared. These tentatively include `String`, `i64`, and `Bool`.

### Functions

Functions provide a richer type that maps a list of argument sorts to a return sort.

```
(function FunctionSort (ArgSort1 ArgSort2 ... ArgSortN) ReturnSort)
```
Functions also can take a 4th argument specifying the cost. This can be used for fancier cost functions than simply AST size. This design also allows the cost to be a function itself. The default cost assumed if a cost argument is not given is `(Num 1)`.

Importantly, while declared function names can be used anywhere in the benchmark file, the implementation of these functions is left entirely up to the optimization implementation.

### Properties

Properties provide a way to express analyses that propagate through rewrites. They also allow for conditional rewrites, which are an important element of the majority of interesting benchmarks. Similarly to the function implementation, properties are expressed in the most general way possible, requiring the implementation to define how propagation is carried out, as well as how rewrite conditions are evaluated. Intuitively these can be thought of as functions from terms to lattices:

```
;; The general syntax is the following
(property Name (ArgSort1 ArgSort2 ... ArgSortN) ReturnSort)

;; Math example
(property IsNonZero (Math) Bool)
```

### Rewrites

Rewrites are declared with the either the `rewrite` keyword or the `birewrite` keyword. The difference between the two is self evident. Again enforcing this difference correctly is intentionally left up to the user. Rewrites at minimum require a left hand side and a right hand size, with an optional condition as a third argument. Symbolic variables that can match any expression are defined with a `?`, reminiscent of the egg toolkit syntax.

```
(rewrite (Mul (Num 0) ?a) (Num 0))

(birewrite (Div ?x ?x) (Num 1) (IsNonZero ?x))
```
Rewrites can also use properties within the terms.
```
(rewrite (LessThan (UpperBound ?x) (Num 0)) (IsNonZero ?x))
```

### Optimize Calls

The optimize call is usually the last part of the benchmark. It defines what term the implementation should optimize using the defined rewrites and costs.

```
(optimize (Mul (Num 0) (Add (Num 0) (Var "x"))))
```