(sort Math)

;; Direct implementation required
(primitive Add (Math Math) Math)
(primitive Sub (Math Math) Math)
(primitive Mul (Math Math) Math)
(primitive Div (Math Math) Math)
(primitive IsNotZero (Math) Bool
	:desc "Return True if the input argument != 0, else False.")

(lattice Const 
	:desc   "Option<i64> for constant folding analysis."
	:make   "For Add, Sub, or Mul as Op return Some(Op(a, b)). 
			 For Div reduce a / b to simplest fractional form.
			 Else return None."
	:merge  "Assert both constants are identical, otherwise error.")

(analysis GetConst (Math) Const)


;; Direct implementation not required
(constructor Num (i64) Math)
(constructor Var (String) Math)


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
		 :when (IsNotZero ?a))

;; flip multiplication
(rewrite (Mul ?a (Div (Num 1) ?a))
		 (Num 1)
		 :when (IsNotZero ?a))

(optimize (Add (Mul (Var "y") (Add (Var "x") (Var "y")))
			   (Sub (Add (Var "x") (Num 2)) (Add (Var "x") (Var "x")))))
(optimize (Add (Div (Num 1) 
					(Sub (Div (Add (Num 0) (Num 2)) (Num 2)) 
						 (Div (Sub (Num 1) (Num 4)) (Num 1))))
			   (Var "x")))