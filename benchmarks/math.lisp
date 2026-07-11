(sort Math)

(function Num (i64) Math)
(function Var (String) Math)
(function Add (Math Math) Math)
(function Sub (Math Math) Math)
(function Mul (Math Math) Math)
(function Div (Math Math) Math)

;; add comm/assoc
(rewrite (Add ?a ?b)
         (Add ?b ?a))
(rewrite (Add (Add ?a ?b) ?c)
		 (Add ?a (Add ?b ?c)))

;; mul comm/assoc
(rewrite (Mul ?a ?b)
		 (Mul ?b ?a))
(rewrite (Mul (Mul ?a ?b) ?c)
		 (Mul ?a (Mul ?b ?c)))

;; sub-canon
(rewrite (Sub ?a ?b)
		 (Add ?a (Mul (Num -1) ?b)))

;; add simplify
(rewrite (Add ?a ?a)
		 (Mul (Num 2) ?a))

;; distributivity
(rewrite (Mul ?a (Add ?b ?c))
         (Add (Mul ?a ?b) (Mul ?a ?c)))
;; factor
(rewrite (Add (Mul ?a ?b) (Mul ?a ?c))
		 (Mul ?a (Add ?b ?c)))

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

;; x + Mul(a, x) -> Mul(a + 1, x)
(rewrite (Add ?x (Mul (Num ?a) ?x))
         (Mul (Add (Num 1) (Num ?a)) ?x))

;; Mul(b, x) + Mul(a, x) -> Mul(a + b, x)
(rewrite (Add (Mul (Num ?a) ?x) (Mul (Num ?b) ?x))
         (Mul (Add (Num ?a) (Num ?b)) ?x))

;; div cancel
(rewrite (Div ?a ?a)
		 (Num 1)
		 :when (!= ?a (Num 0)))

;; flip multiplication
(rewrite (Mul ?a (Div (Num 1) ?a))
		 (Num 1)
		 :when (!= ?a (Num 0)))

;; folding
(rewrite (Add (Num ?a) (Num ?b))
		 (Num (+ ?a ?b)))
(rewrite (Sub (Num ?a) (Num ?b))
		 (Num (- ?a ?b)))
(rewrite (Mul (Num ?a) (Num ?b))
		 (Num (* ?a ?b)))
(rewrite (Div (Num ?a) (Num ?b))
		 (Num (/ ?a ?b)))

(optimize (Add 
			(Mul (Var "y") (Add (Var "x") (Var "y")))
			(Sub (Add (Var "x") (Num 2)) (Add (Var "x") (Var "x")))))
(optimize (Add (Div 
					(Num 1) 
					(Sub
						(Div (Add (Num 0) (Num 2)) (Num 2)) 
						(Div (Sub (Num 1) (Num 4)) (Num 1))))
				(Var "x")))