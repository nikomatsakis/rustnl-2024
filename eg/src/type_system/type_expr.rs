use crate::{
    grammar::{Expr, FnDefnBoundData, Ty},
    type_system::env::Env,
};
use formality_core::{judgment_fn, Cons};

judgment_fn! {
    pub fn type_expr_as(
        env: Env,
        expr: Expr,
        ty: Ty,
    ) => () {
        debug(expr, ty, env)

        (
            (type_expr(env, expr) => ty)
            (if ty == ty_expected)
            ------------------------------- ("type_expr_as")
            (type_expr_as(env, expr, ty_expected) => ())
        )
    }
}

judgment_fn! {
    pub fn type_expr(
        env: Env,
        expr: Expr,
    ) => Ty {
        debug(expr, env)

        (
            (type_expr(&env, &*initializer) => var_ty)
            (let env = env.with_program_variable(&var, var_ty))
            (type_expr(&env, &*body) => body_ty)
            ------------------------------- ("let")
            (type_expr(env, Expr::Let(var, initializer, body)) => body_ty)
        )

        (
            (type_exprs(&env, exprs) => tys)
            ------------------------------- ("tuple")
            (type_expr(env, Expr::Tuple(exprs)) => Ty::Tuple(tys))
        )

        (
            (type_binary_expr(&env, &*l, &*r) => ty)
            ------------------------------- ("add")
            (type_expr(env, Expr::Add(l, r)) => ty)
        )

        (
            (type_binary_expr(&env, &*l, &*r) => ty)
            ------------------------------- ("sub")
            (type_expr(env, Expr::Sub(l, r)) => ty)
        )

        (
            (type_binary_expr(&env, &*l, &*r) => ty)
            ------------------------------- ("mul")
            (type_expr(env, Expr::Mul(l, r)) => ty)
        )

        (
            (type_binary_expr(&env, &*l, &*r) => ty)
            ------------------------------- ("div")
            (type_expr(env, Expr::Div(l, r)) => ty)
        )

        (
            ------------------------------- ("literal")
            (type_expr(_env, Expr::Integer(_)) => Ty::U32)
        )

        (
            (let ty = env.program_variable_ty(v)?)
            ------------------------------- ("var")
            (type_expr(env, Expr::Var(v)) => ty)
        )

        (
            (let func = env.fn_defn(&func)?)
            (let FnDefnBoundData { args, return_ty, body: _ } = func.binder.instantiate_with(&types)?)
            (let arg_tys: Vec<_> = args.iter().map(|arg| &arg.ty).collect())
            (type_exprs_as(env, &exprs, arg_tys) => ())
            ------------------------------- ("call")
            (type_expr(env, Expr::Call(func, types, exprs)) => &return_ty)
        )
    }
}

judgment_fn! {
    fn type_exprs_as(
        env: Env,
        expr: Vec<Expr>,
        ty: Vec<Ty>,
    ) => () {
        debug(expr, ty, env)

        (
            ------------------------------- ("nil")
            (type_exprs_as(_env, (), ()) => ())
        )

        (
            (type_expr_as(&env, expr, ty) => ())
            (type_exprs_as(&env, &exprs, &tys) => ())
            ------------------------------- ("nil")
            (type_exprs_as(env, Cons(expr, exprs), Cons(ty, tys)) => ())
        )
    }
}

judgment_fn! {
    fn type_exprs(
        env: Env,
        exprs: Vec<Expr>,
    ) => Vec<Ty> {
        debug(exprs, env)

        (
            (type_expr(&env, head) => head_ty)
            (type_exprs(&env, &tail) => tail_tys)
            --------------------------------- ("cons")
            (type_exprs(env, Cons(head, tail)) => Cons(&head_ty, tail_tys))
        )

        (
            --------------------------------- ("nil")
            (type_exprs(_env, ()) => ())
        )
    }
}

judgment_fn! {
    fn type_binary_expr(
        env: Env,
        l: Expr,
        r: Expr,
    ) => Ty {
        debug(l, r, env)

        (
            (type_expr(&env, l) => ty)
            (type_expr_as(&env, &r, &ty) => ())
            ------------------------------- ("type_binary_expr")
            (type_binary_expr(env, l, r) => &ty)
        )
    }
}
