//! Simple egg baseline for running benchmarks

use std::collections::HashMap;
use std::marker::PhantomData;

use ::egg::{AstSize, DidMerge, ENodeOrVar, Extractor, RecExpr};
use ::egg::{Id, Pattern, PatternAst, Runner};
use ::egg::{Symbol, define_language};
use ::egg::EGraph;

use epo::ast::*;
use epo::context::{ProblemContext};
use epo::{IntType, Result, Solver};

define_language! {
    pub enum Lang {
        Num(IntType),
        //String(StringType),
        Call(Symbol, Vec<Id>),
    }
}

#[derive(Default)]
struct MyAnalysis<C: ProblemContext> {
    context: C,
}



impl<C: ProblemContext> ::egg::Analysis<Lang> for MyAnalysis<C> {
    type Data = Option<C::AnalysisTupleReturnType>;

    fn make(egraph: &mut EGraph<Lang, MyAnalysis<C>>, enode: &Lang, _id: Id) -> Self::Data {
        match enode {
            Lang::Num(n) => {
                C::get_all_analysis_results_as_tuple(&mut egraph.analysis.context, "Num", *n)
            },
            Lang::Call(name, ids) => {
                todo!()
            }
        }
    }

    fn merge(&mut self, to: &mut Self::Data, from: Self::Data) -> DidMerge {
        ::egg::merge_option(to, from, |_l, _r| {
            //assert_eq!(**l, *r, "Conflicting values in e-graph: {l} vs {r}");
            DidMerge(false, false)
        })
    }

    fn modify(egraph: &mut EGraph, id: Id) {
        if let Some(data) = &egraph[id].data {
            let new_data = data.downcast_ref::<IntType>();
            match new_data {
                Some(data) => {
                    let new_id = egraph.add(Lang::Num(*data));
                    egraph.union(id, new_id);
                }
                None => {}
            }
        }
    }
    
    fn remake(egraph: &mut egg::EGraph<Lang, Self>, enode: &Lang, id: Id) -> Self::Data {
        Self::make(egraph, enode, id)
    }
    
    fn pre_union(
        egraph: &egg::EGraph<Lang, Self>,
        id1: Id,
        id2: Id,
        justification: &Option<egg::Justification>,
    ) {
    }
    
    fn allow_ematching_cycles(&self) -> bool {
        true
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
        Term::IntLit(n) => pat.add(ENodeOrVar::ENode(Lang::Num(*n))),
        Term::Call(f, terms) => {
            let children: Vec<Id> = terms.iter().map(|t| term_to_pattern_rec(t, pat)).collect();
            let node = match f.as_str() {
                _ => Lang::Call(f.parse().unwrap(), children),
            };
            pat.add(ENodeOrVar::ENode(node))
        }
        _ => todo!(),
    }
}

fn recexpr_to_term(expr: &RecExpr<Lang>, id: Id) -> Term {
    match &expr[id] {
        Lang::Num(n) => Term::IntLit(*n),
        Lang::Call(f, children) => {
            let terms = children.iter().map(|&c| recexpr_to_term(expr, c)).collect();
            Term::Call(f.to_string(), terms)
        }
    }
}

impl<C: ProblemContext> Solver<C> for EggSolver {
    fn new() -> Self {
        Default::default()
    }

    fn declare_analysis(&mut self, context: C) -> Result<()> {
        todo!()
    }

    fn declare_primitives(&mut self, context: C) -> Result<()> {
        todo!()
    }

    fn declare_sort(&mut self, _sort: Sort) -> Result<()> {
        Ok(())
    }

    fn declare_constructor(&mut self, _cons: Constructor) -> Result<()> {
        Ok(())
    }

    fn declare_rewrite(&mut self, rewrite: Rewrite) -> Result<()> {
        match rewrite {
            Rewrite::Rewrite(re) => {
                let lhs: Pattern<Lang> = term_to_pattern(&re.lhs);
                let rhs: Pattern<Lang> = term_to_pattern(&re.rhs);
                let egg_rw: egg::Rewrite<Lang, MyAnalysis> = EggRewrite::new(&re.name, lhs, rhs)?;
                self.rules.push(egg_rw);
            }
            Rewrite::BiRewrite(bire) => {
                let lhs: Pattern<Lang> = term_to_pattern(&bire.lhs);
                let rhs: Pattern<Lang> = term_to_pattern(&bire.rhs);
                // Since pattern cant be cloned need to create new patterns again
                // Better way of doing this?
                let bi_rhs: Pattern<Lang> = term_to_pattern(&bire.lhs);
                let bi_lhs: Pattern<Lang> = term_to_pattern(&bire.rhs);
                let egg_rw: egg::Rewrite<Lang, MyAnalysis> = EggRewrite::new(&bire.name, lhs, rhs)?;
                let egg_bi_rw: egg::Rewrite<Lang, MyAnalysis> =
                    EggRewrite::new(&bire.name, bi_lhs, bi_rhs)?;
                self.rules.push(egg_rw);
                self.rules.push(egg_bi_rw);
            }
        }
        Ok(())
    }

    fn declare_cost(&mut self, _costs: CostFunc) -> Result<()> {
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
        self.runner = Runner::new(MyAnalysis {
            _context: PhantomData,
        })
        .with_expr(&term)
        .run(&self.rules);
        let ext = Extractor::new(&self.runner.egraph, AstSize);
        let (_best_cost, best_expr) = ext.find_best(self.runner.roots[0]);
        let best_term = recexpr_to_term(&best_expr, best_expr.root());
        Ok(best_term)
    }
}
