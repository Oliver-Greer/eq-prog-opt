use crate::Result;

type Name = String;

#[derive(PartialEq, Debug)]
pub enum Decl {
    Sort(Sort),
    Constructor(Constructor),
    Primitive(Primitive),
    Rewrite(Rewrite),
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
pub struct Primitive {
    pub name: Name,
    pub args: Vec<Name>,
    pub ret: Name,
    pub desc: Option<String>
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
    pub cond: Option<Term>
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
            _ => Ok(()),
        }
    }
}

pub struct Program {
    pub sorts: Vec<Sort>,
    pub constructors: Vec<Constructor>,
    pub primitives: Vec<Primitive>,
    pub rewrites: Vec<Rewrite>,
    pub optimize: Vec<Optimize>,
}

impl Program {
    pub fn add_decl(&mut self, decl: Decl) -> Result<()> {
        match decl {
            Decl::Sort(s) => self.sorts.push(s),
            Decl::Constructor(c) => self.constructors.push(c),
            Decl::Primitive(p) => self.primitives.push(p),
            Decl::Rewrite(mut r) => {
                match &mut r {
                    Rewrite::Rewrite(re) => {
                        // unique-ify rewrite names by appending the current number of rewrites
                        re.name = format!("{}.{}", re.name, self.rewrites.len());
                    },
                    Rewrite::BiRewrite(bire) => {
                        // unique-ify rewrite names by appending the current number of rewrites
                        bire.name = format!("{}.{}", bire.name, self.rewrites.len());
                    }
                };
                self.rewrites.push(r)
            }
            Decl::Optimize(o) => self.optimize.push(o),
        }
        Ok(())
    }

    pub fn from_decls(decls: Vec<Decl>) -> Result<Self> {
        let mut prog = Program {
            sorts: vec![],
            constructors: vec![],
            primitives: vec![],
            rewrites: vec![],
            optimize: vec![],
        };
        for decl in decls {
            prog.add_decl(decl)?;
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
