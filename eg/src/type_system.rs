use std::sync::Arc;

use formality_core::Fallible;

use crate::grammar::{Arg, Expr, FnDefn, FnDefnBoundData, Program};

use self::env::Env;
mod env;
mod type_expr;

pub fn check_program(program: &Arc<Program>) -> Fallible<()> {
    let Program { fn_defns, expr } = &**program;

    for fn_defn in fn_defns {
        check_fn_defn(program, fn_defn)?;
    }

    check_expr(program, expr)?;

    Ok(())
}

fn check_fn_defn(program: &Arc<Program>, fn_defn: &FnDefn) -> Fallible<()> {
    let (
        mut env,
        FnDefnBoundData {
            args,
            return_ty,
            body,
        },
    ) = Env::open(program, &fn_defn.binder);

    for Arg { name, ty } in args {
        env = env.with_program_variable(name, ty);
    }

    type_expr::type_expr_as(&env, body, return_ty).check_proven()?;

    Ok(())
}

fn check_expr(program: &Arc<Program>, expr: &Expr) -> Fallible<()> {
    let env = Env::new(program);
    type_expr::type_expr(&env, expr).into_set()?;
    Ok(())
}
