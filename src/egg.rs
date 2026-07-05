use ::egg::{AstSize, DidMerge, ENodeOrVar, Extractor, RecExpr};
use ::egg::{Id, Pattern, PatternAst, Runner};
use ::egg::{Symbol, define_language};

use crate::*;

define_language! {
    pub enum Lang {
        Num(i64),
        "+" = Add([Id; 2]),
        "*" = Mul([Id; 2]),
        Call(Symbol, Vec<Id>),
    }
}

type EGraph = ::egg::EGraph<Lang, MyAnalysis>;
type EggRewrite = ::egg::Rewrite<Lang, MyAnalysis>;

#[derive(Default)]
struct MyAnalysis;
impl ::egg::Analysis<Lang> for MyAnalysis {
    type Data = Option<i64>;

    fn make(egraph: &mut EGraph, enode: &Lang, _id: Id) -> Self::Data {
        match enode {
            Lang::Num(n) => Some(*n),
            Lang::Add([a, b]) => {
                let a = egraph[*a].data?;
                let b = egraph[*b].data?;
                Some(a + b)
            }
            Lang::Mul([a, b]) => {
                let a = egraph[*a].data?;
                let b = egraph[*b].data?;
                Some(a * b)
            }
            Lang::Call(_, _) => None,
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
            let new_id = egraph.add(Lang::Num(data));
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
        Term::Num(n) => pat.add(ENodeOrVar::ENode(Lang::Num(*n))),
        Term::Call(f, terms) => {
            let children: Vec<Id> = terms.iter().map(|t| term_to_pattern_rec(t, pat)).collect();
            let node = match f.as_str() {
                "+" => Lang::Add([children[0], children[1]]),
                "*" => Lang::Mul([children[0], children[1]]),
                _ => Lang::Call(f.parse().unwrap(), children),
            };
            pat.add(ENodeOrVar::ENode(node))
        }
    }
}

fn recexpr_to_term(expr: &RecExpr<Lang>, id: Id) -> Term {
    match &expr[id] {
        Lang::Num(n) => Term::Num(*n),
        Lang::Call(f, children) => {
            let terms = children.iter().map(|&c| recexpr_to_term(expr, c)).collect();
            Term::Call(f.to_string(), terms)
        }
        Lang::Add(children) => {
            let terms = children.iter().map(|&c| recexpr_to_term(expr, c)).collect();
            Term::Call("+".into(), terms)
        }
        Lang::Mul(children) => {
            let terms = children.iter().map(|&c| recexpr_to_term(expr, c)).collect();
            Term::Call("*".into(), terms)
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

        let egg_rw: ::egg::Rewrite<Lang, MyAnalysis> = EggRewrite::new(&rewrite.name, lhs, rhs).map_err(|e| e.to_string())?;
        self.rules.push(egg_rw);
        Ok(())
    }

    fn optimize(&mut self, optimize: Optimize) -> Result<Term> {
        let pat: Pattern<Lang> = term_to_pattern(&optimize.term);
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

        self.runner = Runner::default().with_expr(&term).run(&self.rules);
        let ext: Extractor<'_, AstSize, Lang, MyAnalysis> = Extractor::new(&self.runner.egraph, AstSize);
        let (_best_cost, best_expr) = ext.find_best(self.runner.roots[0]);
        let best_term: Term = recexpr_to_term(&best_expr, best_expr.root());
        Ok(best_term)
    }
}
