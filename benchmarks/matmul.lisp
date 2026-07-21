(sort Matrix)

;; analysis
(lattice i64)
(analysis Rows (Matrix) Option<i64>)
(analysis Cols (Matrix) Option<i64>)

;; MakeMatrix takes a name and rows and columns
(constructor MakeMatrix (String i64 i64) Matrix)
;; Multiple two Matrices
(constructor MatMul (Matrix Matrix) Matrix)

;; associativity
(birewrite Associativity
    (MatMul (MatMul ?A ?B) ?C) 
    (MatMul ?A (MatMul ?B ?C)) 
    :when (= (Cols ?B) (Rows ?C)))

(optimize 
    (MatMul
        (MakeMatrix "A" 10 100)
        (MatMul (MakeMatrix "B" 100 5) (MakeMatrix "C" 5 50))
    )
)