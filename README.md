# Equational Program Optimization

This is a set of benchmarks for equational program optimization.

## Worklist And Implementation Notes

Below is a list of items to complete, as well as open questions covering many of the non-trivial parts of designing an implementation agnostic bench-marking suite.

### Rewrites

- [x] Birewrites and rewrites implemented as a rewrite enum for solvers to match on.
- [X] Variables to pattern match represented by ?x.
- [ ] Conditions: design struct that can represent conditions. Conditions should be implemented as functions in a .rs file and automatically collected into a registry. These functions should be automatically parsed into Fn pointers in the condition struct. We provide a map from `condName` to `Condition` Something like:

```rust
struct Condition<T> {
  benchmarkName: String, // For scoping when collecting
  conditionName: String,
  conditionFn: fn(&[T]) -> bool,
}
```

Open questions:

- Do conditions need to return anything other than bool?
  - **Answer:** 
- Should we wrap in a trait to define arg types and parameterize the solver over that type?
  - **Answer:** 
- Should the solver be responsible for recursively evaluating the cond if nested, or should we have a trait do that work internally and have a condition have sub conditions?
  - **Answer:** 
- How do we automatically register conditions without dead code elim getting in the way?
  - **Answer:** build.rs parsing?

### Analysis

- [ ] Figure out a clean way of bridging analysis forms to the solver implementation. This requires answering the following questions:
  - How to we ensure the analysis implementation can accommodate both multiple analysis as a tuple (egg approach) and multiple analysis as separate structs?
    - **Answer:** We define an analysis trait with the requirement that we have one function per node. Something like:

      ```rust
      impl Analysis for MyAnalysis {
        fn fold_add(a: type, b: type) -> Option<type> {
          a + b
        }
      }

      impl Analysis for OtherAnalysis {
        fn min_add(a: othertype, b: othertype) -> Option<othertype> {
          min(a, b)
        }
      }
      ```

      Types here are parameterized per impl. Then we have the problem context contain a vec of `<A: Analysis>` and we have the user relate function names to node names. The problem context can have a function that takes a node name and returns a vector of functions, but also a function that exposed the vector of analysis. This has issues but it could work.

Open questions:

- How do we resolve the fact that functions will have different signatures and therefore cannot be packed into a single vector without `dyn Any` or even more indirection?
  - **Answer:** Restrict all types to `u64`? This may work.
- In some benchmarks, we want a function from term -> term. An egraph doesn't have a concept of a term. This means we need to make the analysis generic over a term type defined by the solver, but that causes an issue. The solver is already parameterized over the problem context meaning the dependency is circular. I think this can be resolved if we are careful, but how?
  - **Answer:** 
- If we do the above and have the solver define what a term is, we need a lightweight term api. What should this contain? What does a "term" need?
  - **Answer:** 
- As far as I know, tuple layouts cannot be constructed at runtime. Therefore there is probably some amount of compile time definitions necessary if we use the above plan. How do we handle this?
  - **Answer:**
 
### Cost Functions

- [X] Define two types of built-in cost functions with enum variants: Tree and DAG.
- [X] Give cost functions a name that can be referenced in optimize declarations.
- [ ] Formally define the structure of a built-in cost function. Currently the plan is the following:
  - Cost functions have one or both of the following: A list of node names and IntLit for simple numerical costs per node or a list of node names and primitive function names that take in the nodes children and return a cost. Something like:

    ```lisp
    (Add 1)
    (Sub 1)
    (Mul 10)
    ;; OR
    (Add addCost)
    (Sub subCost)
    (Mul mulCost)
    ;; OR
    (Add 1)
    (Sub subCost)
    (Mul 10)
    ```
  
Open questions:

- If cost functions can reference functions defined in a .rs file, we need to add that to the problem context. How do we do this?
  - **Answer:** 
- How do we address custom cost functions? Should this somehow be completely defined by a .rs function? What arguments would it take?
  - **Answer:** Maybe the function is a just a big match statement over node names.

### Bench-marking

- [ ] Define the types of metrics we are interested in collecting. Memory usage? Runtime?
- [ ] Formalize the goal of the solver. Decide how to measure success.
- [ ] Figure out ways of displaying results.
