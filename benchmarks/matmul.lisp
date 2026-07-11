(sort Matrix)

;; MakeMatrix takes a name and rows and columns
(function MakeMatrix (String i64 i64) Matrix)
;; Multiple two Matrices
(function MatMul (Matrix Matrix) Matrix)
;; Extract rows and columns
(function Rows (Matrix) i64)
(function Cols (Matrix) i64)

;; get rows and columns
(rewrite GetRows
    (Rows (MakeMatrix ?name ?r ?c))
    ?r
)
(rewrite GetCols
    (Cols (MakeMatrix ?name ?r ?c))
    ?c
)

;; associativity
(rewrite Associativity
    (MatMul (MatMul ?A ?B) ?C) 
    (MatMul ?A (MatMul ?B ?C)) 
    :when (= (Cols ?B) (Rows ?C))
)

;; otherway, will use birewrite later
(rewrite AssociativityRight
    (MatMul ?A (MatMul ?B ?C)) 
    (MatMul (MatMul ?A ?B) ?C)
    :when (= (Cols ?A) (Rows ?B))
)

(optimize 
    (MatMul
        (MakeMatrix "A" 10 100)
        (MatMul (MakeMatrix "B" 100 5) (MakeMatrix "C" 5 50))
    )
)