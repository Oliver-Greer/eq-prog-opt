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

use crate::{FloatType, IntType, StringType, Result};

type Name = String;
type CostDesc = String;


/// [`Decl`] outlines all possible declarations in the benchmark DSL.
#[derive(PartialEq, Debug)]
pub enum Decl {
    /// `ImplementationFile` contains the path to the .rs file 
    /// that implements analysis and primitives for this benchmark if any.
    ImplementationFile(String),

    /// [`Sort`] declares a scoped type for a given benchmark.
    Sort(Sort),

    /// [`Constructor`] creates a node to be referenced in 
    /// rust implementations, [`Rewrite`], [`CostFunc`], and [`Optimize`] declarations.
    Constructor(Constructor),

    /// [`Rewrite`] declares equivalents patterns in this benchmark.
    /// Rewrites can be bidirectional or unidirectional.
    Rewrite(Rewrite),

    /// [`CostFunc`] declares a cost function that can be used in an [`Optimize`] [`Decl`].
    CostFunc(CostFunc),

    /// [`Optimize`] declares terms that the solver needs to simplify.
    Optimize(Optimize),
}


/// A [`Sort`] is a simple type declaration.
#[derive(PartialEq, Debug)]
pub struct Sort {
    /// The type as a string literal.
    pub name: Name,
}


/// [`Constructor`] declares a DSL node with a functional representation.
#[derive(PartialEq, Debug)]
pub struct Constructor {
    /// Name of the node as a string literal.
    pub name: Name,

    /// A vector of Sorts the argument should take in, referenced as strings.
    pub args: Vec<Name>,

    /// The [`Sort`] this node should return.
    pub ret: Name,
}


/// A [`Rewrite`] can be bidirectional or unidirectional.
#[derive(PartialEq, Debug)]
pub enum Rewrite {
    /// A unidirectional [`Rewrite`].
    Rewrite(RewriteVariant),

    /// A bidirectional [`Rewrite`].
    BiRewrite(RewriteVariant),
}


/// Both the [`Rewrite`] num variants have the same fields, 
/// wrapped in a [`RewriteVariant`] struct.
#[derive(PartialEq, Debug)]
pub struct RewriteVariant {
    /// Name of the [`Rewrite`] for proof explanations.
    pub name: Name,

    /// Lefthand side [`Term`] for searching/applying.
    pub lhs: Term,

    /// Righthand side [`Term`] for applying/applying.
    pub rhs: Term,

    /// An optional [`Term`] that evaluates to `true` or `false`.
    /// Rewrites are conditional on the result.
    pub cond: Option<Term>,
}


/// this will be redesigned shortly. N/A
#[derive(PartialEq, Debug)]
pub enum CostFuncType {
    Tree,
    Graph,
    Custom(CostDesc),
}


/// this will be redesigned shortly. N/A
#[derive(PartialEq, Debug)]
pub struct CostFunc {
    pub name: Name,
    pub func_type: CostFuncType,
    pub costs: Option<Vec<Term>>,
}


/// [`Optimize`] struct that denotes a required [`Term`] to simplify.
#[derive(PartialEq, Debug)]
pub struct Optimize {
    /// The [`Term`] to simplify.
    pub term: Term,
}


/// [`Term`] is the base ast node.
#[derive(PartialEq, Debug)]
pub enum Term {
    /// A variable that can be used symbolically in rewrites.
    Var(Name),

    /// A Constant integer literal with the type set globally [`IntType`]
    IntLit(IntType),

    /// A Constant floating pointer type set globally by [`FloatType`]
    FloatLit(FloatType),

    /// A globally set string literally type defined by [`StringType`]
    StringLit(StringType),
    
    /// A generic representation of all other terms.
    /// These are typically functional nodes defined by [`Constructor`].
    Call(Name, Vec<Term>),
}


impl std::fmt::Display for Term {
    /// Pretty print a term for debugging and [`Optimize`] results.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Var(v) => write!(f, "{}", v),
            Term::IntLit(n) => write!(f, "{}", n),
            Term::FloatLit(fl) => write!(f, "{}", fl),
            Term::StringLit(s) => write!(f, "{}", s),
            Term::Call(func, args) => {
                write!(f, "({}", func)?;
                // print all child terms
                for arg in args {
                    write!(f, " {}", arg)?;
                }
                write!(f, ")")
            }
        }
    }
}


/// The root of all benchmark files.
/// [`Program`] wraps all declarations and bridges to related .rs files.
pub struct Program {
    /// The filename of this benchmark. Used for scoping types .rs files.
    pub implementation_file: Name,

    /// Vector of [`Sort`] structs used in this benchmark.
    /// Most solvers won't need this.
    pub sorts: Vec<Sort>,

    /// Vector of [`Constructor`] structs used in this benchmark.
    /// Most solvers won't need this because the names are already 
    /// in Call terms.
    pub constructors: Vec<Constructor>,

    /// Vector of [`Rewrite`] structs used in this benchmark.
    pub rewrites: Vec<Rewrite>,

    /// this will be redesigned shortly. N/A
    pub costfuncs: Vec<CostFunc>,

    /// Vector of [`Optimize`] declarations for the solver to simplify.
    pub optimize: Vec<Optimize>,
}


/// Implementation of [`Program`] with methods to construct 
/// all fields from a given benchmark file.
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


    fn from_decls(decls: Vec<Decl>) -> Result<Self> {
        let mut prog = Program {
            implementation_file: String::new(),
            sorts: vec![],
            constructors: vec![],
            rewrites: vec![],
            costfuncs: vec![],
            optimize: vec![],
        };

        for decl in decls {
            prog.add_decl(decl)?;
        }

        Ok(prog)
    }


    /// Create a [`Program`] from a benchmark in string form.
    pub fn from_str(s: &str) -> Result<Self> {
        let decls: Vec<Decl> = crate::parse::parse_decls(s)?;
        Self::from_decls(decls)
    }


    /// Create a [`Program`] from a benchmark filepath.
    pub fn from_file(path: &str) -> Result<Self> {
        let src: String = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        Self::from_str(&src)
    }
}
