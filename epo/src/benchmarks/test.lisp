(impl "benchmarks/src/math.rs")

(sort Math)

(constructor Add (Math Math) Math)
(constructor Num (i64) Math)

(optimize (Add (Num 1) (Num 2)))
