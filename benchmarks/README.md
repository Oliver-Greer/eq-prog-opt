# Benchmark DSL and File Specification

This README provides a basis for understanding the benchmark file syntax and format, as well as what is expected of any implementation attempting to run the benchmark suite.

Benchmark files provide an implementation agnostic DSL for common program optimization tasks. The syntax is heavily inspired by Egglog's S-Expr syntax with the exclusion of features that "bake in" an EqSat approach.

This specification aims to provide features rich enough to express at least the following benchmarks:

- Math expression simplification
- Matrix chain multiplication
- Decompiling CAD into structured programs
- Proving inequalities in the Halide compiler

## Language Keywords

### Primitive Types

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

Functions also can take a 4th argument specifying the cost. This can be used for fancier cost functions than simply AST size. The default cost assumed if a cost argument is not given is `1`.

### Rewrites

Rewrites are declared with the either the `rewrite` keyword or the `birewrite` keyword. The difference between the two is self evident. Again enforcing this difference correctly is intentionally left up to the user. Rewrites at minimum require a left hand side and a right hand size, with an optional condition as a third argument. Rewrites can also be named. Symbolic variables that can match any term are defined with a `?`, reminiscent of the egg toolkit syntax.

```
;; A named rewrite
(rewrite MulCancel (Mul (Num 0) ?a) (Num 0))

;; A birewrite with a conditional term
(birewrite (Div ?x ?x) (Num 1) :when (!= ?x 0))
```

### Optimize Calls

The optimize call is usually the last part of the benchmark. It defines what term the implementation should optimize using the defined rewrites and costs.

```
(optimize (Mul (Num 0) (Add (Num 0) (Var "x"))))
```
