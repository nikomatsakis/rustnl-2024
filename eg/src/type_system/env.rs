use std::sync::Arc;

use anyhow::anyhow;
use formality_core::{cast_impl, Fallible, Map, Upcast};

use crate::{
    eg_lang::{
        grammar::{Binder, UniversalVar, VarIndex, Variable},
        Term,
    },
    grammar::{FnDefn, Id, Program, Ty},
};

#[derive(Clone, Debug, Ord, Eq, PartialEq, PartialOrd, Hash)]
pub struct Env {
    program: Arc<Program>,
    type_variables: Vec<Variable>,
    program_variables: Map<Id, Ty>,
}

cast_impl!(Env);

impl Env {
    /// Create an empty environment with no type variables in scope.
    pub fn new(program: &Arc<Program>) -> Self {
        Self {
            program: program.clone(),
            type_variables: vec![],
            program_variables: Map::default(),
        }
    }

    /// Create an environment with one universal type variable
    /// for each of the bound variables in `binder`;
    /// return the environment and the contents of binder.
    pub fn open<T: Term>(program: &Arc<Program>, binder: &Binder<T>) -> (Self, T) {
        let type_variables: Vec<_> = binder
            .kinds()
            .iter()
            .zip(0..)
            .map(|(&kind, index)| {
                UniversalVar {
                    kind,
                    var_index: VarIndex { index },
                }
                .upcast()
            })
            .collect();

        let term = binder.instantiate_with(&type_variables).unwrap();

        let env = Self {
            program: program.clone(),
            type_variables,
            program_variables: Map::default(),
        };

        (env, term)
    }

    pub fn with_program_variable(&self, var: impl Upcast<Id>, ty: impl Upcast<Ty>) -> Env {
        let mut env = self.clone();
        env.program_variables.insert(var.upcast(), ty.upcast());
        env
    }

    pub fn program_variable_ty(&self, var: Id) -> Fallible<&Ty> {
        if let Some(ty) = self.program_variables.get(&var) {
            Ok(ty)
        } else {
            anyhow::bail!("undefined variable `{var:?}`")
        }
    }

    pub fn fn_defn(&self, name: &Id) -> Fallible<&FnDefn> {
        self.program
            .fn_defns
            .iter()
            .find(|f| f.name == *name)
            .ok_or_else(|| anyhow!("no function named `{name:?}`"))
    }
}
