//! AST Representation for a benchmark file
//!
//! The base node is a Term which can be a variable, an integer, or a call expression.
//! All other nodes are various kinds of declaration.
//!
//! This would benefit from more granular typing.
//! For example a CostFunc should not take a vector of Terms,
//! but rather a vector of (TermName Func/Int) which is much more specific.
//! Currently we are creating ambiguity for the person implementing this.
//!
//! We should also offer type checking the AST to make writing benchmarks easier.
//! For example, names used in the cost function term list should
//! be declared previously as nodes.

use std::collections::HashMap;

use crate::{AnalysisBridge, PrimitiveBridge, Result};
use benchmarks::math;
use problem_ctx::{Analysis, IntType, Primitive};

type Name = String;
type CostDesc = String;


/// `Decl` outlines all possible declarations in the benchmark DSL.
#[derive(PartialEq, Debug)]
pub enum Decl {
    /// `ImplementationFile` contains the path to the .rs file 
    /// that implements analysis and primitives for this benchmark if any.
    ImplementationFile(String),

    /// `Sort` declares a scoped type for a given benchmark.
    Sort(Sort),

    /// `Constructor` creates a node to be referenced in 
    /// rust implementations, `Rewrite`, `CostFunc`, and `Optimize` declarations.
    Constructor(Constructor),

    /// `Rewrite` declares equivalents patterns in this benchmark.
    /// Rewrites can be bidirectional or unidirectional.
    Rewrite(Rewrite),

    /// `CostFunc` declares a cost function that can be used in an `Optimize` `Decl`.
    CostFunc(CostFunc),

    /// `Optimize` declares terms that the solver needs to simplify.
    Optimize(Optimize),
}


/// A `Sort` is a simple type declaration.
#[derive(PartialEq, Debug)]
pub struct Sort {
    /// The type as a string literal.
    pub name: Name,
}


/// `Constructor` declares a DSL node with a functional representation.
#[derive(PartialEq, Debug)]
pub struct Constructor {
    /// Name of the node as a string literal.
    pub name: Name,

    /// A vector of Sorts the argument should take in, referenced as strings.
    pub args: Vec<Name>,

    /// The Sort this node should return.
    pub ret: Name,
}


/// 
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
    IntLit(IntType),
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
    pub implementation_file: String,
    pub analysis_bridge: AnalysisBridge,
    pub primitive_bridge: PrimitiveBridge,
    pub sorts: Vec<Sort>,
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
        self.analysis_bridge
            .map
            .insert(String::from(analysis.term_name), analysis.analysis);
        Ok(())
    }

    fn add_primitive(&mut self, primitive: &Primitive) -> Result<()> {
        self.primitive_bridge
            .map
            .insert(String::from(primitive.func_name), primitive.primitive);
        Ok(())
    }

    fn from_decls(decls: Vec<Decl>) -> Result<Self> {
        let mut prog = Program {
            implementation_file: String::new(),
            analysis_bridge: AnalysisBridge {
                map: HashMap::new(),
            },
            primitive_bridge: PrimitiveBridge {
                map: HashMap::new(),
            },
            sorts: vec![],
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
        // Move away from inventory and write custom own plugin registry
        math::dummy();

        // Analysis and primitives are scoped based on benchmark file
        // This means one program per benchmark
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
