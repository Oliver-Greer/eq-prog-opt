(sort Math)

(constructor Num (i64) Math)
(constructor Var (String) Math)
(constructor Add (Math Math) Math)
(constructor Sub (Math Math) Math)
(constructor Mul (Math Math) Math)
(constructor Div (Math Math) Math)

;; add comm
(rewrite (Add ?a ?b)
         (Add ?b ?a))

;; mul comm
(rewrite (Mul ?a ?b)
		 (Mul ?b ?a))

;; sub-canon
(rewrite (Sub ?a ?b)
		 (Add ?a (Mul (Num -1) ?b)))

;; add simplify
(rewrite (Add ?a ?a)
		 (Mul (Num 2) ?a))

;; add cancel
(rewrite (Add (Num 0) ?a)
		 ?a)

;; sub cancel
(rewrite (Sub ?a ?a)
		 (Num 0))

;; mul cancel
(rewrite (Mul (Num 0) ?a) 
         (Num 0))
(rewrite (Mul (Num 1) ?a)
		 ?a)

;; distributivity
(rewrite (Mul ?a (Add ?b ?c))
         (Add (Mul ?a ?b) (Mul ?a ?c)))
;; factor
(rewrite (Add (Mul ?a ?b) (Mul ?a ?c))
		 (Mul ?a (Add ?b ?c)))

;; x + Mul(a, x) -> Mul(a + 1, x)
(rewrite (Add ?x (Mul (Num ?a) ?x))
         (Mul (Add (Num 1) (Num ?a)) ?x))

;; Mul(b, x) + Mul(a, x) -> Mul(a + b, x)
(rewrite (Add (Mul (Num ?a) ?x) (Mul (Num ?b) ?x))
         (Mul (Add (Num ?a) (Num ?b)) ?x))

;; test const fold
(optimize (Mul (Add (Sub (Num 10) (Num 5)) (Num 2)) (Num 2)))

;; other tests
(optimize (Add (Mul (Var "y") (Add (Var "x") (Var "y")))
			   (Sub (Add (Var "x") (Num 2)) (Add (Var "x") (Var "x")))))
(optimize (Add (Div (Num 1) 
					(Sub (Div (Add (Num 0) (Num 2)) (Num 2)) 
						 (Div (Sub (Num 1) (Num 4)) (Num 1))))
			   (Var "x")))