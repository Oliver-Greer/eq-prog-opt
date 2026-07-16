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
//! Primitive       -> '(' WhiteSpace 'primitive' WhiteSpace Identifier WhiteSpace
//!                         '(' (WhiteSpace Identifier)* WhiteSpace ')'
//!                         WhiteSpace Identifier WhiteSpace ')'
//!
//! NOTE: Rewrites can also have names but those are left out here for conciseness
//! Bi/RewriteDecl  -> '(' WhiteSpace ('rewrite' / 'birewrite')
//!                         WhiteSpace Term WhiteSpace Term WhiteSpace ')'
//!                     | '(' WhiteSpace ('rewrite' / 'birewrite')
//!                         WhiteSpace Term WhiteSpace Term WhiteSpace
//!                         ":when" WhiteSpace Term WhiteSpace ')'
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

        rule primitive_decl() -> Decl
            = "(" ws() "primitive" ws() name:identifier() ws()
              "(" args:(ws() a:identifier() { a })* ws() ")" ws()
              ret:identifier() ws() ")" {
                Decl::Primitive(Primitive { name, args, ret })
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

        rule optimize_decl() -> Decl
            = "(" ws() "optimize" ws() term:term() ws() ")" {
                Decl::Optimize(Optimize { term })
            }

        rule decl() -> Decl
            = sort_decl()
            / constructor_decl()
            / primitive_decl()
            / rewrite_decl()
            / birewrite_decl()
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
    fn parse_primitive() {
        // intentionally testing whitespace
        let input: &str = "( primitive \n MyName (Sort1 \n Sort2  )  Ret)";
        let output: Result<Decl> = parse_decl(input);
        let expected_output: Decl = Decl::Primitive(Primitive {
            name: "MyName".to_string(),
            args: vec!["Sort1".to_string(), "Sort2".to_string()],
            ret: "Ret".to_string()
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
