use crate::eg_lang::grammar::{BoundVar, ExistentialVar, UniversalVar};

use super::*;
use formality_core::{cast_impl, Downcast, DowncastTo, Upcast, UpcastFrom};

impl UpcastFrom<Variable> for Parameter {
    fn upcast_from(term: Variable) -> Self {
        match term.kind() {
            Kind::Ty => Ty::var(term).upcast(),
        }
    }
}

impl DowncastTo<Variable> for Parameter {
    fn downcast_to(&self) -> Option<Variable> {
        match self {
            Parameter::Ty(t) => t.downcast(),
        }
    }
}

impl DowncastTo<UniversalVar> for Ty {
    fn downcast_to(&self) -> Option<UniversalVar> {
        let v: Variable = self.downcast()?;
        v.downcast()
    }
}

cast_impl!((BoundVar) <: (Variable) <: (Parameter));
cast_impl!((ExistentialVar) <: (Variable) <: (Parameter));
cast_impl!((UniversalVar) <: (Variable) <: (Parameter));
