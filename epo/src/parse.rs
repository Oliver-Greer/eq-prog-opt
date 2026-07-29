//! Parser for the benchmark file specification.
//!
//! The benchmark DSL has the following grammar:
//!
//! WhiteSpaceChar  -> ' ' | '\t' | '\n' | '\r'
//!
//! Comment         -> ';' [^'\n']* ('\n' | EOF)
//!
//! WhiteSpace      -> (WhiteSpaceChar | Comment)*
//!
//! Identifier      -> !(WhiteSpace | '(' | ')' | ';')
//!
//! Variable        -> '?' Identifier
//!
//! IntegerLiteral  -> '-'? [0..9]+
//!
//! StringLiteral   -> '"' [_] '"'
//!
//! TermAtom        -> BoolLiteral | IntegerLiteral | StringLiteral | Variable | Identifier
//!
//! TermList        -> '(' WhiteSpace Identifier (WhiteSpace Term)* WhiteSpace ')'
//!
//! Term            -> TermList | TermAtom
//!
//! SortDecl        -> '(' WhiteSpace 'sort' WhiteSpace Identifier WhiteSpace ')'
//!
//! Constructor     -> '(' WhiteSpace 'constructor' WhiteSpace Identifier WhiteSpace
//!                         '(' (WhiteSpace Identifier)* WhiteSpace ')'
//!                         WhiteSpace Identifier WhiteSpace ')'
//! 
//! Description     -> WhiteSpace ":desc" WhiteSpace StringLiteral WhiteSpace
//! 
//! Make            -> WhiteSpace ":make" WhiteSpace StringLiteral WhiteSpace
//! 
//! Merge           -> WhiteSpace ":merge" WhiteSpace StringLiteral WhiteSpace
//! 
//! Lattice         -> '(' WhiteSpace 'lattice' WhiteSpace Identifier (Description | WhiteSpace)
//!                         (Make | WhiteSpace) (Merge | WhiteSpace) ')'
//! 
//! Analysis        -> '(' WhiteSpace 'analysis' WhiteSpace Identifier WhiteSpace
//!                         '(' (WhiteSpace Identifier)* WhiteSpace ')'
//!                         WhiteSpace Identifier (Description | WhiteSpace) ')'
//! 
//! Primitive       -> '(' WhiteSpace 'primitive' WhiteSpace Identifier WhiteSpace
//!                         '(' (WhiteSpace Identifier)* WhiteSpace ')'
//!                         WhiteSpace Identifier (Description | WhiteSpace) ')'
//!
//! NOTE: Rewrites can also have names but those are left out here for conciseness
//! Bi/RewriteDecl  -> '(' WhiteSpace ('rewrite' / 'birewrite')
//!                         WhiteSpace Term WhiteSpace Term WhiteSpace ')'
//!                     | '(' WhiteSpace ('rewrite' / 'birewrite')
//!                         WhiteSpace Term WhiteSpace Term WhiteSpace
//!                         ":when" WhiteSpace Term WhiteSpace ')'
//!
//! CostType        -> '"Tree"' | '"Graph"' | StringLiteral
//! 
//! NodeCost        -> '(' WhiteSpace Identifier WhiteSpace IntegerLiteral WhiteSpace ')'
//! 
//! CostFunc        -> '(' WhiteSpace 'cost-fn' WhiteSpace Identifier WhiteSpace
//!                     ':type' WhiteSpace CostType WhiteSpace (NodeCost WhiteSpace)* (WhiteSpace | '') ')'
//! 
//! Optimize        -> '(' WhiteSpace 'optimize' WhiteSpace Term (WhiteSpace  ')'

use crate::*;

peg::parser! {
    grammar sexp_parser() for str {
        rule ws_char()
            = [' ' | '\t' | '\n' | '\r']

        rule comment()
            = ";" [^'\n']* ("\n" / ![_])

        rule ws()
            = (ws_char() / comment())*

        rule identifier() -> String
            = s:$((!(['(' | ')' | ';'] / ws_char()) [_])+) { s.to_string() }

        rule int_lit() -> i64
            = n:$("-"? ['0'..='9']+)    {? n.parse().map_err(|_| "invalid integer") }

        rule string_lit() -> String
            = "\"" s:$([^'\"']*) "\""   { s.to_string() }

        rule term_atom() -> Term
            = n:int_lit()               { Term::IntLit(n) }
            / s:string_lit()            { Term::Var(s) }
            / i:identifier()            { Term::Var(i) }

        rule term_list() -> Term
            = "(" ws() f:identifier() args:(ws() t:term() { t })* ws() ")" {
                Term::Call(f, args)
            }

        rule term() -> Term
            = ws() t:(term_list() / term_atom()) ws() { t }

        rule sort_decl() -> Decl
            = "(" ws() "sort" ws() name:identifier() ws() ")" {
                Decl::Sort(Sort { name })
            }

        rule constructor_decl() -> Decl
            = "(" ws() "constructor" ws() name:identifier() ws()
              "(" args:(ws() a:identifier() { a })* ws() ")" ws()
              ret:identifier() ws() ")" {
                Decl::Constructor(Constructor { name, args, ret })
            }

        rule make() -> String
            = ws() ":make" ws() make:string_lit() ws() {
                make
            }

        rule merge() -> String
            = ws() ":merge" ws() merge:string_lit() ws() {
                merge
            }

        rule description() -> String
            = ws() ":desc" ws() desc:string_lit() ws() {
                desc
            }

        rule lattice_decl() -> Decl
            = "(" ws() "lattice" ws() name:identifier() ws() ")" {
                Decl::Lattice(Lattice { 
                    name, 
                    desc: None, 
                    make: None, 
                    merge: None,  
                })
            }
            / "(" ws() "lattice" ws() name:identifier() ws() desc:description() ws() ")" {
                Decl::Lattice(Lattice { 
                    name, 
                    desc: Some(desc), 
                    make: None, 
                    merge: None, 
                })
            }
            / "(" ws() "lattice" ws() name:identifier() ws() make:make() ws() ")" {
                Decl::Lattice(Lattice { 
                    name, 
                    desc: None, 
                    make: Some(make), 
                    merge: None, 
                })
            }
            / "(" ws() "lattice" ws() name:identifier() ws() merge:merge() ws() ")" {
                Decl::Lattice(Lattice { 
                    name, 
                    desc: None, 
                    make: None, 
                    merge: Some(merge), 
                })
            }
            / "(" ws() "lattice" ws() name:identifier() ws() desc:description() ws()
                make:make() ws() ")" {
                    Decl::Lattice(Lattice { 
                        name, 
                        desc: Some(desc), 
                        make: Some(make), 
                        merge: None
                    })
            }
            / "(" ws() "lattice" ws() name:identifier() ws() desc:description() ws()
                merge:merge() ws() ")" {
                    Decl::Lattice(Lattice { 
                        name, 
                        desc: Some(desc), 
                        make: None, 
                        merge: Some(merge)
                    })
            }
            / "(" ws() "lattice" ws() name:identifier() ws() make:make() ws()
                merge:merge() ws() ")" {
                    Decl::Lattice(Lattice { 
                        name, 
                        desc: None, 
                        make: Some(make), 
                        merge: Some(merge)
                    })
            }
            / "(" ws() "lattice" ws() name:identifier() ws() desc:description() ws()
                make:make() ws() merge:merge() ws() ")" {
                Decl::Lattice(Lattice { 
                    name, 
                    desc: Some(desc), 
                    make: Some(make), 
                    merge: Some(merge),
                })
            }

        rule analysis_decl() -> Decl
            = "(" ws() "analysis" ws() name:identifier() ws()
              "(" args:(ws() a:identifier() { a })* ws() ")" ws()
              ret:identifier() ws() desc:description() ")" {
                Decl::Analysis(Analysis { name, args, ret, desc: Some(desc) })
            }
            / "(" ws() "analysis" ws() name:identifier() ws()
              "(" args:(ws() a:identifier() { a })* ws() ")" ws()
              ret:identifier() ws() ")" {
                Decl::Analysis(Analysis { name, args, ret, desc: None })
            }

        rule primitive_decl() -> Decl
            = "(" ws() "primitive" ws() name:identifier() ws()
              "(" args:(ws() a:identifier() { a })* ws() ")" ws()
              ret:identifier() ws() desc:description() ")" {
                Decl::Primitive(Primitive { name, args, ret, desc: Some(desc) })
            }
            / "(" ws() "primitive" ws() name:identifier() ws()
              "(" args:(ws() a:identifier() { a })* ws() ")" ws()
              ret:identifier() ws() ")" {
                Decl::Primitive(Primitive { name, args, ret, desc: None })
            }

        rule rewrite_decl() -> Decl
            = "(" ws() "rewrite" ws() name:identifier() ws() lhs:term()
                ws() rhs:term() ws() ":when" ws() c:term() ws() ")" {
                Decl::Rewrite(
                    Rewrite::Rewrite( RewriteVariant {
                        name,
                        lhs,
                        rhs,
                        cond: Some(c)
                    })
                )
            }
            / "(" ws() "rewrite" ws() name:identifier() ws() lhs:term() ws() rhs:term() ws() ")" {
                Decl::Rewrite(
                    Rewrite::Rewrite( RewriteVariant {
                        name,
                        lhs,
                        rhs,
                        cond: None
                    })
                )
            }
            / "(" ws() "rewrite" ws() lhs:term() ws() rhs:term() ws() ":when" ws() c:term() ws() ")" {
                Decl::Rewrite(
                    Rewrite::Rewrite( RewriteVariant {
                        name: String::from(""),
                        lhs,
                        rhs,
                        cond: Some(c)
                    })
                )
            }
            / "(" ws() "rewrite" ws() lhs:term() ws() rhs:term() ws() ")" {
                Decl::Rewrite(
                    Rewrite::Rewrite( RewriteVariant {
                        name: String::from(""),
                        lhs,
                        rhs,
                        cond: None
                    })
                )
            }

        rule birewrite_decl() -> Decl
            = "(" ws() "birewrite" ws() name:identifier() ws() lhs:term()
                ws() rhs:term() ws() ":when" ws() c:term() ws() ")" {
                Decl::Rewrite(
                    Rewrite::BiRewrite( RewriteVariant {
                        name,
                        lhs,
                        rhs,
                        cond: Some(c)
                    })
                )
            }
            / "(" ws() "birewrite" ws() name:identifier() ws() lhs:term() ws() rhs:term() ws() ")" {
                Decl::Rewrite(
                    Rewrite::BiRewrite( RewriteVariant {
                        name,
                        lhs,
                        rhs,
                        cond: None
                    })
                )
            }
            / "(" ws() "birewrite" ws() lhs:term() ws() rhs:term() ws() ":when" ws() c:term() ws() ")" {
                Decl::Rewrite(
                    Rewrite::BiRewrite( RewriteVariant {
                        name: String::from(""),
                        lhs,
                        rhs,
                        cond: Some(c)
                    })
                )
            }
            / "(" ws() "birewrite" ws() lhs:term() ws() rhs:term() ws() ")" {
                Decl::Rewrite(
                    Rewrite::BiRewrite( RewriteVariant {
                        name: String::from(""),
                        lhs,
                        rhs,
                        cond: None
                    })
                )
            }

        rule cost_term() -> Term
            = "(" ws() name:identifier() ws() i:int_lit() ws() ")" {
                Term::Call(
                    name,
                    vec![Term::IntLit(i)]
                )
            }

        rule node_costs() -> Vec<Term>
            = ws() items:(t:cost_term() ws() {t})* { items }

        rule cost_decl() -> Decl
            = "(" ws() "cost-fn" ws() name:identifier() ws() ":type" ws() "Tree"
                ws() costs:node_costs() ws() ")" {
                    Decl::CostFunc(
                        CostFunc { name, func_type: CostFuncType::Tree, costs: Some(costs) }
                    )
            }
            / "(" ws() "cost-fn" ws() name:identifier() ws() ":type" ws() "Graph"
                ws() costs:node_costs() ws() ")" {
                    Decl::CostFunc(
                        CostFunc { name, func_type: CostFuncType::Graph, costs: Some(costs) }
                    )
            }
            / "(" ws() "cost-fn" ws() name:identifier() ws() ":type" ws() desc:string_lit()
                ws() ")" {
                    Decl::CostFunc(
                        CostFunc { name, func_type: CostFuncType::Custom(desc), costs: None }
                    )
            }

        rule optimize_decl() -> Decl
            = "(" ws() "optimize" ws() term:term() ws() ")" {
                Decl::Optimize(Optimize { term })
            }

        rule decl() -> Decl
            = sort_decl()
            / constructor_decl()
            / primitive_decl()
            / lattice_decl()
            / analysis_decl()
            / rewrite_decl()
            / birewrite_decl()
            / cost_decl()
            / optimize_decl()

        pub rule parse_term() -> Term
            = ws() t:(term_list() / term_atom()) ws() ![_] { t }

        pub rule parse_decl() -> Decl
            = ws() d:decl() ws() ![_] { d }

        pub rule parse_decls() -> Vec<Decl>
            = ws() ds:(decl() ** ws()) ws() ![_] { ds }
    }
}

pub fn parse_term(input: &str) -> Result<Term> {
    sexp_parser::parse_term(input).map_err(|e| e.to_string())
}

pub fn parse_decl(input: &str) -> Result<Decl> {
    sexp_parser::parse_decl(input).map_err(|e| e.to_string())
}

pub fn parse_decls(input: &str) -> Result<Vec<Decl>> {
    sexp_parser::parse_decls(input).map_err(|e| e.to_string())
}


/// Parsing unit tests with limited but sufficient coverage.
/// More edge cases will be handled by integration tests down the pipeline.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_sort() {
        // intentionally testing whitespace
        let input: &str = "(  sort   Math)";
        let output: Result<Decl> = parse_decl(input);
        let expected_output: Decl = Decl::Sort(Sort {
            name: "Math".to_string(),
        });
        assert!(output.is_ok());
        assert!(output.unwrap() == expected_output);
    }

    #[test]
    fn parse_constructor() {
        // intentionally testing whitespace
        let input: &str = "(constructor \n MyName (Sort1 \n Sort2  )  Ret)";
        let output: Result<Decl> = parse_decl(input);
        let expected_output: Decl = Decl::Constructor(Constructor {
            name: "MyName".to_string(),
            args: vec!["Sort1".to_string(), "Sort2".to_string()],
            ret: "Ret".to_string()
        });
        assert!(output.is_ok());
        assert!(output.unwrap() == expected_output);
    }

    #[test]
    fn parse_primitive_no_desc() {
        // intentionally testing whitespace
        let input: &str = "( primitive \n MyName (Sort1 \n Sort2  )  Ret)";
        let output: Result<Decl> = parse_decl(input);
        let expected_output: Decl = Decl::Primitive(Primitive {
            name: "MyName".to_string(),
            args: vec!["Sort1".to_string(), "Sort2".to_string()],
            ret: "Ret".to_string(),
            desc: None
        });
        assert!(output.is_ok());
        assert!(output.unwrap() == expected_output);
    }

    #[test]
    fn parse_primitive_with_desc() {
        // intentionally testing whitespace
        let input: &str = "( primitive \n MyName (Sort1 \n Sort2  )  Ret  :desc \"This is it\" )";
        let output: Result<Decl> = parse_decl(input);
        let expected_output: Decl = Decl::Primitive(Primitive {
            name: "MyName".to_string(),
            args: vec!["Sort1".to_string(), "Sort2".to_string()],
            ret: "Ret".to_string(),
            desc: Some("This is it".to_string())
        });
        assert!(output.is_ok());
        assert!(output.unwrap() == expected_output);
    }

    #[test]
    fn parse_lattice_no_desc() {
        // intentionally testing whitespace
        let input: &str = "( lattice \n MyName )";
        let output: Result<Decl> = parse_decl(input);
        let expected_output: Decl = Decl::Lattice( Lattice {
            name: "MyName".to_string(),
            desc: None,
            make: None,
            merge: None
        });
        assert!(output.is_ok());
        assert!(output.unwrap() == expected_output);
    }

    #[test]
    fn parse_lattice_with_desc() {
        // intentionally testing whitespace
        let input: &str = "( lattice \n MyName :desc \"description\" )";
        let output: Result<Decl> = parse_decl(input);
        let expected_output: Decl = Decl::Lattice( Lattice {
            name: "MyName".to_string(),
            desc: Some("description".to_string()),
            make: None,
            merge: None
        });
        assert!(output.is_ok());
        assert!(output.unwrap() == expected_output);
    }

    #[test]
    fn parse_lattice_with_desc_and_make() {
        // intentionally testing whitespace
        let input: &str = "( lattice \n MyName :desc \"description\" \n :make \t \"make\")";
        let output: Result<Decl> = parse_decl(input);
        let expected_output: Decl = Decl::Lattice( Lattice {
            name: "MyName".to_string(),
            desc: Some("description".to_string()),
            make: Some("make".to_string()),
            merge: None
        });
        assert!(output.is_ok());
        assert!(output.unwrap() == expected_output);
    }

    #[test]
    fn parse_lattice_with_desc_and_make_plus_merge() {
        // intentionally testing whitespace
        let input: &str = "( lattice \n MyName :desc \"description\" \n :make \t \"make\"
        \t\t :merge \t \"merge\"\t )";
        let output: Result<Decl> = parse_decl(input);
        let expected_output: Decl = Decl::Lattice( Lattice {
            name: "MyName".to_string(),
            desc: Some("description".to_string()),
            make: Some("make".to_string()),
            merge: Some("merge".to_string()),
        });
        assert!(output.is_ok());
        assert!(output.unwrap() == expected_output);
    }

    #[test]
    fn parse_rewrite_with_name() {
        // intentionally testing whitespace
        // one way rewrite with name
        let input: &str = "(rewrite \n MyName ?a \t ?b)";
        let output: Result<Decl> = parse_decl(input);
        let expected_output: Decl = Decl::Rewrite(Rewrite::Rewrite(RewriteVariant{
            name: "MyName".to_string(),
            lhs: Term::Var("?a".to_string()),
            rhs: Term::Var("?b".to_string()),
            cond: None,
        }));
        assert!(output.is_ok());
        assert!(output.unwrap() == expected_output);
    }

    #[test]
    fn parse_birewrite_with_name() {
        // two way rewrite with name
        let input: &str = "(birewrite \n MyName ?a \t ?b)";
        let output: Result<Decl> = parse_decl(input);
        let expected_output: Decl = Decl::Rewrite(Rewrite::BiRewrite( RewriteVariant {
            name: "MyName".to_string(),
            lhs: Term::Var("?a".to_string()),
            rhs: Term::Var("?b".to_string()),
            cond: None,
        }));
        assert!(output.is_ok());
        assert!(output.unwrap() == expected_output);
    }

    #[test]
    fn parse_rewrite_with_name_and_cond() {
        // one way rewrite with name and cond
        let input: &str = "(rewrite \n MyName ?a \t ?b :when True)";
        let output: Result<Decl> = parse_decl(input);
        let expected_output: Decl = Decl::Rewrite(Rewrite::Rewrite( RewriteVariant {
            name: "MyName".to_string(),
            lhs: Term::Var("?a".to_string()),
            rhs: Term::Var("?b".to_string()),
            cond: Some(Term::Var("True".to_string())),
        }));
        assert!(output.is_ok());
        assert!(output.unwrap() == expected_output);
    }

    #[test]
    fn parse_rewrite_with_cond_no_name() {
        // one way rewrite without name and cond
        let input: &str = "(rewrite \n ?a \t ?b :when \t False)";
        let output: Result<Decl> = parse_decl(input);
        let expected_output: Decl = Decl::Rewrite(Rewrite::Rewrite(RewriteVariant {
            name: String::from(""),
            lhs: Term::Var("?a".to_string()),
            rhs: Term::Var("?b".to_string()),
            cond: Some(Term::Var("False".to_string())),
        }));
        println!("{:?}", output);
        assert!(output.is_ok());
        assert!(output.unwrap() == expected_output);
    }

    #[test]
    fn parse_tree_cost() {
        let input: &str = "( cost-fn \n MyName :type Tree (Add 1) (Sub 1) (Der 10))";
        let output: Result<Decl> = parse_decl(input);
        let expected_output: Decl = Decl::CostFunc(CostFunc { 
            name: "MyName".to_string(), 
            func_type: CostFuncType::Tree, 
            costs: Some(vec![Term::Call(
                "Add".to_string(), 
                vec![Term::IntLit(1)]),
                Term::Call(
                "Sub".to_string(), 
                vec![Term::IntLit(1)]),
                Term::Call(
                "Der".to_string(), 
                vec![Term::IntLit(10)])]) 
        });
        println!("{:?}", output);
        assert!(output.is_ok());
        assert!(output.unwrap() == expected_output);
    }

    #[test]
    fn parse_graph_cost() {
        let input: &str = "( cost-fn \n MyName :type Graph 
            (Add 1) 
            (Sub 1) 
            (Der 10))";
        let output: Result<Decl> = parse_decl(input);
        let expected_output: Decl = Decl::CostFunc(CostFunc { 
            name: "MyName".to_string(), 
            func_type: CostFuncType::Graph, 
            costs: Some(vec![Term::Call(
                "Add".to_string(), 
                vec![Term::IntLit(1)]),
                Term::Call(
                "Sub".to_string(), 
                vec![Term::IntLit(1)]),
                Term::Call(
                "Der".to_string(), 
                vec![Term::IntLit(10)])]) 
        });
        println!("{:?}", output);
        assert!(output.is_ok());
        assert!(output.unwrap() == expected_output);
    }

    #[test]
    fn parse_custom_cost() {
        // Testing whether I can start a custom string with Graph
        let input: &str = "( cost-fn \n MyName :type \"Graph MyCustomGraphCost\"\t)";
        let output: Result<Decl> = parse_decl(input);
        let expected_output: Decl = Decl::CostFunc(CostFunc { 
            name: "MyName".to_string(), 
            func_type: CostFuncType::Custom("Graph MyCustomGraphCost".to_string()), 
            costs: None 
        });
        println!("{:?}", output);
        assert!(output.is_ok());
        assert!(output.unwrap() == expected_output);
    }

    #[test]
    fn parse_optimize() {
        let input: &str = "(optimize (Mul (Num 1) (Num 2)))";
        let output: Result<Decl> = parse_decl(input);
        let expected_output: Decl = Decl::Optimize(Optimize {
            term: Term::Call(
                "Mul".to_string(),
                vec![
                    Term::Call("Num".to_string(), vec![Term::IntLit(1)]),
                    Term::Call("Num".to_string(), vec![Term::IntLit(2)]),
                ],
            ),
        });
        assert!(output.is_ok());
        assert!(output.unwrap() == expected_output);
    }
}
