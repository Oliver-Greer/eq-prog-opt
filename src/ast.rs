use crate::Result;

type Name = String;

#[derive(Clone)]
#[derive(PartialEq)]
pub enum Term {
    Var(Name),
    Num(i64),
    Call(Name, Vec<Term>),
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Var(v) => write!(f, "{}", v),
            Term::Num(n) => write!(f, "{}", n),
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
    pub funcs: Vec<Function>,
    pub rewrites: Vec<Rewrite>,
    pub optimize: Vec<Optimize>,
}

impl Program {
    pub fn add_decl(&mut self, decl: Decl) -> Result<()> {
        match decl {
            Decl::Sort(s) => self.sorts.push(s),
            Decl::Function(f) => self.funcs.push(f),
            Decl::Rewrite(mut r) => {
                // unique-ify rewrite names by appending the current number of rewrites
                r.name = format!("{}.{}", r.name, self.rewrites.len());
                self.rewrites.push(r)
            }
            Decl::Optimize(o) => self.optimize.push(o),
        }
        Ok(())
    }

    pub fn from_decls(decls: Vec<Decl>) -> Result<Self> {
        let mut prog = Program {
            sorts: vec![],
            funcs: vec![],
            rewrites: vec![],
            optimize: vec![],
        };
        for decl in decls {
            prog.add_decl(decl)?;
        }
        Ok(prog)
    }

    pub fn from_str(s: &str) -> Result<Self> {
        let decls = crate::parse::parse_decls(s)?;
        Self::from_decls(decls)
    }

    pub fn from_file(path: &str) -> Result<Self> {
        let src = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        Self::from_str(&src)
    }
}

pub enum Decl {
    Sort(Sort),
    Function(Function),
    Rewrite(Rewrite),
    Optimize(Optimize),
}

pub struct Sort {
    pub name: Name,
}

pub struct Function {
    pub name: Name,
    pub args: Vec<Name>,
    pub ret: Name,
}

pub struct Rewrite {
    pub name: Name,
    pub lhs: Term,
    pub rhs: Term,
}

pub struct Optimize {
    pub term: Term,
}


impl Clone for Rewrite {
    fn clone(&self) -> Self {
        Rewrite { 
            name: self.name.clone(), 
            lhs: self.lhs.clone(), 
            rhs: self.rhs.clone() 
        }
    }
}