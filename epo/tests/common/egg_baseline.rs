/// Simple egg baseline for a selection of benchmarks
/// Supports both = and != as conditional calls, and +, -, *, and / for constant folding
/// Code needs significant cleanup and work, but provides a working prototype
/// Next step is adding basic numerical costs to AST nodes 
/// and automatically supporting birewrites

use egg::{Condition, ConditionEqual};
use ::egg::{AstSize, ConditionalApplier, DidMerge, ENodeOrVar, Extractor, RecExpr};
use ::egg::{Id, Pattern, PatternAst, Runner};
use ::egg::{Symbol, define_language};

use epo::Result;
use epo::Solver;
use epo::ast::*;

define_language! {
    pub enum Lang {
        Num(i64),
        Bool(bool),
        "+" = Add([Id; 2]),
        "-" = Sub([Id; 2]),
        "*" = Mul([Id; 2]),
        "/" = Div([Id; 2]),
        "=" = Eq([Id; 2]),
        "!=" = Neq([Id; 2]),
        Call(Symbol, Vec<Id>),
    }
}

type EGraph = ::egg::EGraph<Lang, MyAnalysis>;
type EggRewrite = ::egg::Rewrite<Lang, MyAnalysis>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DataTypes {
    Bool(bool),
    I64(i64),
}

impl std::fmt::Display for DataTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataTypes::I64(n) => write!(f, "{}", n),
            DataTypes::Bool(b) => write!(f, "{}", b),
        }
    }
}

#[derive(Default)]
struct MyAnalysis;
impl ::egg::Analysis<Lang> for MyAnalysis {
    type Data = Option<DataTypes>;

    fn make(egraph: &mut EGraph, enode: &Lang, _id: Id) -> Self::Data {
        match enode {
            Lang::Num(n) => Some(DataTypes::I64(*n)),
            Lang::Bool(b) => Some(DataTypes::Bool(*b)),
            Lang::Add([a, b]) => {
                if let (Some(DataTypes::I64(a_val)), Some(DataTypes::I64(b_val))) =
                    (egraph[*a].data, egraph[*b].data)
                {
                    Some(DataTypes::I64(a_val + b_val))
                } else {
                    None
                }
            }
            Lang::Sub([a, b]) => {
                if let (Some(DataTypes::I64(a_val)), Some(DataTypes::I64(b_val))) =
                    (egraph[*a].data, egraph[*b].data)
                {
                    Some(DataTypes::I64(a_val - b_val))
                } else {
                    None
                }
            }
            Lang::Mul([a, b]) => {
                if let (Some(DataTypes::I64(a_val)), Some(DataTypes::I64(b_val))) =
                    (egraph[*a].data, egraph[*b].data)
                {
                    Some(DataTypes::I64(a_val * b_val))
                } else {
                    None
                }
            }
            Lang::Div([a, b]) => {
                if let (Some(DataTypes::I64(a_val)), Some(DataTypes::I64(b_val))) =
                    (egraph[*a].data, egraph[*b].data)
                {
                    if b_val != 0 {
                        Some(DataTypes::I64(a_val / b_val))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Lang::Eq([a, b]) => {
                if let (Some(DataTypes::I64(a_val)), Some(DataTypes::I64(b_val))) =
                    (egraph[*a].data, egraph[*b].data)
                {
                    Some(DataTypes::Bool(a_val == b_val))
                } else {
                    None
                }
            }
            Lang::Neq([a, b]) => {
                if let (Some(DataTypes::I64(a_val)), Some(DataTypes::I64(b_val))) =
                    (egraph[*a].data, egraph[*b].data)
                {
                    Some(DataTypes::Bool(a_val != b_val))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn merge(&mut self, to: &mut Self::Data, from: Self::Data) -> DidMerge {
        ::egg::merge_option(to, from, |l, r| {
            assert_eq!(*l, r, "Conflicting values in e-graph: {l} vs {r}");
            DidMerge(false, false)
        })
    }

    fn modify(egraph: &mut EGraph, id: Id) {
        if let Some(data) = egraph[id].data {
            let new_id;
            match data {
                DataTypes::Bool(b) => {
                    new_id = egraph.add(Lang::Bool(b));
                }
                DataTypes::I64(n) => {
                    new_id = egraph.add(Lang::Num(n));
                }
            }
            egraph.union(id, new_id);
        }
    }
}

#[derive(Default)]
pub struct EggSolver {
    rules: Vec<EggRewrite>,
    runner: Runner<Lang, MyAnalysis>,
}

fn term_to_pattern(term: &Term) -> Pattern<Lang> {
    let mut pat = PatternAst::default();
    term_to_pattern_rec(term, &mut pat);
    Pattern::new(pat)
}

fn term_to_pattern_rec(term: &Term, pat: &mut PatternAst<Lang>) -> Id {
    match term {
        Term::Var(v) => {
            let node = if v.starts_with('?') {
                ENodeOrVar::Var(v.parse().unwrap())
            } else {
                ENodeOrVar::ENode(Lang::Call(v.parse().unwrap(), vec![]))
            };
            pat.add(node)
        }
        Term::BoolLit(b) => {
            let node = match b {
                true => Lang::Bool(true),
                false => Lang::Bool(false),
            };
            pat.add(ENodeOrVar::ENode(node))
        }
        Term::IntLit(n) => pat.add(ENodeOrVar::ENode(Lang::Num(*n))),
        Term::Call(f, terms) => {
            let children: Vec<Id> = terms.iter().map(|t| term_to_pattern_rec(t, pat)).collect();
            let node = match f.as_str() {
                "+" => Lang::Add([children[0], children[1]]),
                "-" => Lang::Sub([children[0], children[1]]),
                "*" => Lang::Mul([children[0], children[1]]),
                "/" => Lang::Div([children[0], children[1]]),
                "=" => Lang::Eq([children[0], children[1]]),
                "!=" => Lang::Neq([children[0], children[1]]),
                _ => Lang::Call(f.parse().unwrap(), children),
            };
            pat.add(ENodeOrVar::ENode(node))
        }
    }
}

fn recexpr_to_term(expr: &RecExpr<Lang>, id: Id) -> Term {
    match &expr[id] {
        Lang::Num(n) => Term::IntLit(*n),
        Lang::Bool(b) => Term::BoolLit(*b),
        Lang::Call(f, children) => {
            let terms = children.iter().map(|&c| recexpr_to_term(expr, c)).collect();
            Term::Call(f.to_string(), terms)
        }
        Lang::Add(children) => {
            let terms = children.iter().map(|&c| recexpr_to_term(expr, c)).collect();
            Term::Call("+".into(), terms)
        }
        Lang::Sub(children) => {
            let terms = children.iter().map(|&c| recexpr_to_term(expr, c)).collect();
            Term::Call("-".into(), terms)
        }
        Lang::Mul(children) => {
            let terms = children.iter().map(|&c| recexpr_to_term(expr, c)).collect();
            Term::Call("*".into(), terms)
        }
        Lang::Div(children) => {
            let terms = children.iter().map(|&c| recexpr_to_term(expr, c)).collect();
            Term::Call("/".into(), terms)
        }
        Lang::Eq(children) => {
            let terms = children.iter().map(|&c| recexpr_to_term(expr, c)).collect();
            Term::Call("=".into(), terms)
        }
        Lang::Neq(children) => {
            let terms = children.iter().map(|&c| recexpr_to_term(expr, c)).collect();
            Term::Call("!=".into(), terms)
        }
    }
}

impl Solver for EggSolver {
    fn new() -> Self {
        Default::default()
    }

    fn declare_sort(&mut self, _sort: Sort) -> Result<()> {
        Ok(())
    }

    fn declare_function(&mut self, _func: Function) -> Result<()> {
        Ok(())
    }

    fn declare_rewrite(&mut self, rewrite: Rewrite) -> Result<()> {
        let lhs: Pattern<Lang> = term_to_pattern(&rewrite.lhs);
        let rhs: Pattern<Lang> = term_to_pattern(&rewrite.rhs);
        let cond: Option<(String, Pattern<Lang>, Pattern<Lang>)> = match rewrite.cond {
            Some(c) => {
                match c {
                    Term::Call(name, args) => {
                        // assume only = for now
                        Some((name, term_to_pattern(&args[0]), term_to_pattern(&args[1])))
                    }
                    // this shouldn't ever be the case (why have just a true or false condition), throw error?
                    _ => None
                }
            }
            None => None,
        };

        let egg_rw = match cond {
            Some(c) => {
                let eq_cond = ConditionEqual::new(c.1, c.2);
                let should_flip_cond = c.0 == "!=";
                EggRewrite::new(
                    &rewrite.name,
                    lhs,
                    ConditionalApplier {
                        condition: move |egraph: &mut egg::EGraph<Lang, MyAnalysis>, id: Id, subst: &egg::Subst| {
                            should_flip_cond ^ eq_cond.check(egraph, id, subst)
                        },
                        applier: rhs,
                    },
                )
            }
            .map_err(|e| e.to_string())?,
            None => EggRewrite::new(&rewrite.name, lhs, rhs).map_err(|e| e.to_string())?,
        };
        self.rules.push(egg_rw);
        Ok(())
    }

    fn optimize(&mut self, optimize: Optimize) -> Result<Term> {
        let pat = term_to_pattern(&optimize.term);
        let term: RecExpr<Lang> = pat
            .ast
            .iter()
            .map(|enode| match enode {
                ENodeOrVar::Var(v) => {
                    panic!("Unexpected variable: {v}");
                }
                ENodeOrVar::ENode(enode) => enode.clone(),
            })
            .collect();

        // Uses basic AstSize for now, which may not provide the best solution
        self.runner = Runner::default().with_expr(&term).run(&self.rules);
        let ext = Extractor::new(&self.runner.egraph, AstSize);
        let (_best_cost, best_expr) = ext.find_best(self.runner.roots[0]);
        let best_term = recexpr_to_term(&best_expr, best_expr.root());
        Ok(best_term)
    }
}
