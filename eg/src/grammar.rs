use std::sync::Arc;

use formality_core::{anyhow, language::HasKind, term, Fallible};

use crate::eg_lang::grammar::{Binder, Variable};

mod cast_impls;

#[term($*fn_defns $expr)]
pub struct Program {
    pub fn_defns: Vec<FnDefn>,
    pub expr: Expr,
}

impl Program {
    pub fn fn_defn(&self, name: &Id) -> Fallible<&FnDefn> {
        self.fn_defns
            .iter()
            .find(|f| f.name == *name)
            .ok_or_else(|| anyhow!("no function named `{name:?}`"))
    }
}

#[term(fn $name $binder)]
pub struct FnDefn {
    pub name: Id,
    pub binder: Binder<FnDefnBoundData>,
}

#[term($(args) -> $return_ty { $body })]
pub struct FnDefnBoundData {
    pub args: Vec<Arg>,
    pub return_ty: Ty,
    pub body: Expr,
}

#[term($name: $ty)]
pub struct Arg {
    pub name: Id,
    pub ty: Ty,
}

#[term]
pub enum Expr {
    #[grammar(let $v0 = $v1; $v2)]
    Let(Id, Arc<Expr>, Arc<Expr>),

    #[grammar($v0)]
    Integer(u32),

    #[grammar($(v0))]
    Tuple(Vec<Expr>),

    #[grammar($v0 + $v1)]
    #[precedence(0)]
    Add(Arc<Expr>, Arc<Expr>),

    #[grammar($v0 - $v1)]
    #[precedence(0)]
    Sub(Arc<Expr>, Arc<Expr>),

    #[grammar($v0 * $v1)]
    #[precedence(1)]
    Mul(Arc<Expr>, Arc<Expr>),

    #[grammar($v0 / $v1)]
    #[precedence(1)]
    Div(Arc<Expr>, Arc<Expr>),

    #[grammar($v0)]
    Var(Id),

    // HACK: we should extend formality_core parser
    // so that it's possible to resolve the reduce-reduce
    // conflict with `Var` variant -- but how!
    //
    // Can we thread down lookahead?
    #[grammar(@ $v0 $<?v1> $(v2))]
    Call(Id, Vec<Ty>, Vec<Expr>),
}

#[term]
pub enum Kind {
    Ty,
}

impl Copy for Kind {}

#[term]
pub enum Parameter {
    #[cast]
    Ty(Ty),
}

impl HasKind<crate::FormalityLang> for Parameter {
    fn kind(&self) -> formality_core::language::CoreKind<crate::FormalityLang> {
        match self {
            Parameter::Ty(_) => Kind::Ty,
        }
    }
}

#[term]
pub enum Ty {
    #[variable(Kind::Ty)]
    Var(Variable),

    #[grammar($(v0))]
    Tuple(Vec<Ty>),

    #[grammar(u32)]
    U32,
}

formality_core::id!(Id);
