//! AST Representation for a benchmark file
//!
//! The base node is a Term which can be a variable, an integer, or a call expression.
//! All other nodes are various kinds of declaration.
//!
//! This would benefit from more granular typing.
//! For example a CostFunc should not take a vector of Terms,
//! but rather a vector of (TermName Int) which is much more specific.
//! This creates ambiguity for the person implementing it.
//!
//! We should also offer type checking the AST to make writing benchmarks easier.
//! For example, names used in the cost function term list should
//! be declared previously as nodes.

use std::collections::HashMap;

use crate::{AnalysisMap, PrimitiveMap, Result};
use benchmarks::math;
use macros::problem_context::{Analysis, Primitive};

type Name = String;
type CostDesc = String;

#[derive(PartialEq, Debug)]
pub enum Decl {
    Sort(Sort),
    ImplementationFile(String),
    Constructor(Constructor),
    Rewrite(Rewrite),
    CostFunc(CostFunc),
    Optimize(Optimize),
}

#[derive(PartialEq, Debug)]
pub struct Sort {
    pub name: Name,
}

#[derive(PartialEq, Debug)]
pub struct Constructor {
    pub name: Name,
    pub args: Vec<Name>,
    pub ret: Name,
}

#[derive(PartialEq, Debug)]
pub enum Rewrite {
    Rewrite(RewriteVariant),
    BiRewrite(RewriteVariant),
}

#[derive(PartialEq, Debug)]
pub struct RewriteVariant {
    pub name: Name,
    pub lhs: Term,
    pub rhs: Term,
    pub cond: Option<Term>,
}

#[derive(PartialEq, Debug)]
pub enum CostFuncType {
    Tree,
    Graph,
    Custom(CostDesc),
}

#[derive(PartialEq, Debug)]
pub struct CostFunc {
    pub name: Name,
    pub func_type: CostFuncType,
    pub costs: Option<Vec<Term>>,
}

#[derive(PartialEq, Debug)]
pub struct Optimize {
    pub term: Term,
}

#[derive(PartialEq, Debug)]
pub enum Term {
    Var(Name),
    IntLit(i64),
    Call(Name, Vec<Term>),
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Var(v) => write!(f, "{}", v),
            Term::IntLit(n) => write!(f, "{}", n),
            Term::Call(func, args) => {
                write!(f, "({}", func)?;
                for arg in args {
                    write!(f, " {}", arg)?;
                }
                write!(f, ")")
            }
        }
    }
}

pub struct Program {
    pub sorts: Vec<Sort>,
    pub implementation_file: String,
    pub analysis_impl: AnalysisMap,
    pub primitive_impl: PrimitiveMap,
    pub constructors: Vec<Constructor>,
    pub rewrites: Vec<Rewrite>,
    pub costfuncs: Vec<CostFunc>,
    pub optimize: Vec<Optimize>,
}

impl Program {
    fn add_decl(&mut self, decl: Decl) -> Result<()> {
        match decl {
            Decl::Sort(s) => self.sorts.push(s),
            Decl::ImplementationFile(s) => self.implementation_file = s,
            Decl::Constructor(c) => self.constructors.push(c),
            Decl::Rewrite(mut r) => {
                match &mut r {
                    Rewrite::Rewrite(re) => {
                        // unique-ify rewrite names by appending the current number of rewrites
                        re.name = format!("{}.{}", re.name, self.rewrites.len());
                    }
                    Rewrite::BiRewrite(bire) => {
                        // unique-ify rewrite names by appending the current number of rewrites
                        bire.name = format!("{}.{}", bire.name, self.rewrites.len());
                    }
                };
                self.rewrites.push(r)
            }
            Decl::CostFunc(c) => self.costfuncs.push(c),
            Decl::Optimize(o) => self.optimize.push(o),
        }
        Ok(())
    }

    fn add_analysis(&mut self, analysis: &Analysis) -> Result<()> {
        self.analysis_impl
            .map
            .insert(String::from(analysis.term_name), analysis.analysis);
        Ok(())
    }

    fn add_primitive(&mut self, primitive: &Primitive) -> Result<()> {
        self.primitive_impl
            .map
            .insert(String::from(primitive.func_name), primitive.primitive);
        Ok(())
    }

    fn from_decls(decls: Vec<Decl>) -> Result<Self> {
        let mut prog = Program {
            sorts: vec![],
            implementation_file: String::new(),
            analysis_impl: AnalysisMap {
                map: HashMap::new(),
            },
            primitive_impl: PrimitiveMap {
                map: HashMap::new(),
            },
            constructors: vec![],
            rewrites: vec![],
            costfuncs: vec![],
            optimize: vec![],
        };

        for decl in decls {
            prog.add_decl(decl)?;
        }

        // Hack to ensure the benchmarks crate doesnt get trimmed.
        // Definitely need to solve this later because users will add more benchmark files
        // Maybe move away from inventory and write my own plugin registry
        math::dummy();

        for analysis in inventory::iter::<Analysis> {
            if analysis.benchmark_name == prog.implementation_file {
                prog.add_analysis(analysis)?;
            }
        }

        for primitive in inventory::iter::<Primitive> {
            if primitive.benchmark_name == prog.implementation_file {
                prog.add_primitive(primitive)?;
            }
        }

        Ok(prog)
    }

    pub fn from_str(s: &str) -> Result<Self> {
        let decls: Vec<Decl> = crate::parse::parse_decls(s)?;
        Self::from_decls(decls)
    }

    pub fn from_file(path: &str) -> Result<Self> {
        let src: String = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        Self::from_str(&src)
    }
}
