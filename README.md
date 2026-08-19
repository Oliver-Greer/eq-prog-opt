# Equational Program Optimization

This is a set of benchmarks for equational program optimization.

## Worklist And Implementation Notes

Below is a list of items to complete, as well as open questions covering many of the non-trivial parts of designing an implementation agnostic bench-marking suite.

### Rewrites

- [x] Birewrites and rewrites implemented as a rewrite enum for solvers to match on.
- [X] Variables to pattern match represented by ?x.
- [X] Conditions: Basic conditions are done, but they only return bool. Additionally there is no way in the library to recursively evaluate complex conditions containing multiple function calls. This has to be done by the Solver. There might be more work to do here.

Open questions:

- Do conditions need to return anything other than bool?
  - **Answer:** 
- Should we wrap in a trait to define arg types and parameterize the solver over that type?
  - **Answer:** Arg types are restricted to i64 for ease of use. However the solver trait is still parameterized over the ProblemContext so that it can extract the mapping of string to                     function pointers. 
- Should the solver be responsible for recursively evaluating the cond if nested, or should we have a trait do that work internally and have a condition have sub conditions?
  - **Answer:**
- How do we automatically register conditions without dead code elim getting in the way?
  - **Answer:** Resolved. Use proc_macro to collected and inline Context at compile time. Could be cleaner though.

### Analysis

- [X] Figure out a clean way of bridging analysis forms to the solver implementation. This requires answering the following questions:
  - How to we ensure the analysis implementation can accommodate both multiple analysis as a tuple (egg approach) and multiple analysis as separate structs?
    - **Answer:** We don't. We restrict types to `i64` and require the solver to intern anything else. Still, the tuple problem is significant. Right now there is no easy way to have                          multiple analysis functions per node name.

Open questions:

- How do we resolve the fact that functions will have different signatures and therefore cannot be packed into a single vector without `dyn Any` or even more indirection?
  - **Answer:** Restrict all types to `i64`. This is the native IntType anyways so Solver implementations are relatively straight forward.
- In some benchmarks, we want a function from term -> term. An egraph doesn't have a concept of a term. This means we need to make the analysis generic over a term type defined by the solver, but that causes an issue. The solver is already parameterized over the problem context meaning the dependency is circular. I think this can be resolved if we are careful, but how?
  - **Answer:** 
- If we do the above and have the solver define what a term is, we need a lightweight term api. What should this contain? What does a "term" need?
  - **Answer:** 
- As far as I know, tuple layouts cannot be constructed at runtime. Therefore there is probably some amount of compile time definitions necessary if we use the above plan. How do we handle this?
  - **Answer:** We may not support this and find another way around this. This seems to be a significant hurdle.
 
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
