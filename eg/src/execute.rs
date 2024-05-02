use std::sync::Arc;

use anyhow::{anyhow, bail};
use formality_core::{Fallible, Map};

use crate::grammar::{Expr, FnDefnBoundData, Id, Program};

struct Frame {
    program: Arc<Program>,
    variables: Map<Id, Value>,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Value {
    Integer(u32),
    Tuple(Vec<Value>),
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(arg0) => write!(f, "{arg0}"),
            Self::Tuple(arg0) => {
                let mut d = &mut f.debug_tuple("");
                for elem in arg0 {
                    d = d.field(elem);
                }
                d.finish()
            }
        }
    }
}

pub fn execute(program: &Arc<Program>) -> Fallible<Value> {
    let mut frame = Frame {
        program: program.clone(),
        variables: Map::default(),
    };
    frame.execute_expr(&program.expr)
}

impl Frame {
    fn execute_expr(&mut self, expr: &Expr) -> Fallible<Value> {
        match expr {
            Expr::Let(v, initializer, body) => {
                let value = self.execute_expr(initializer)?;
                self.variables.insert(v.clone(), value);
                self.execute_expr(body)
            }
            Expr::Integer(v) => Ok(Value::Integer(*v)),
            Expr::Tuple(exprs) => exprs.iter().map(|expr| self.execute_expr(expr)).collect(),
            Expr::Add(left, right) => self.binary_op(left, right, |l, r| l + r),
            Expr::Sub(left, right) => self.binary_op(left, right, |l, r| l - r),
            Expr::Mul(left, right) => self.binary_op(left, right, |l, r| l * r),
            Expr::Div(left, right) => self.binary_op(left, right, |l, r| l / r),
            Expr::Var(v) => self
                .variables
                .get(v)
                .cloned()
                .ok_or_else(|| anyhow!("no variable named `{v:?}`")),
            Expr::Call(f, _, args) => {
                let program = self.program.clone();
                let fn_defn = program.fn_defn(f)?;
                let FnDefnBoundData {
                    args: fn_args,
                    return_ty: _,
                    body,
                } = fn_defn.binder.peek();

                if fn_args.len() != args.len() {
                    bail!(
                        "function `{f:?}` expects {} arguments, found {}",
                        fn_args.len(),
                        args.len()
                    )
                }

                let arg_values: Vec<Value> = args
                    .iter()
                    .map(|arg| self.execute_expr(arg))
                    .collect::<Fallible<_>>()?;

                let mut frame = Frame {
                    program: self.program.clone(),
                    variables: fn_args
                        .iter()
                        .zip(arg_values)
                        .map(|(fn_arg, arg_value)| (fn_arg.name.clone(), arg_value))
                        .collect(),
                };
                let result = frame.execute_expr(&body)?;
                Ok(result)
            }
        }
    }

    fn binary_op(
        &mut self,
        left: &Expr,
        right: &Expr,
        primitive: impl Fn(u32, u32) -> u32,
    ) -> Fallible<Value> {
        let left = self.execute_expr(left)?;
        let right = self.execute_expr(right)?;
        left.binary_op(&right, &primitive)
    }
}

impl FromIterator<Value> for Value {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
        let v: Vec<Value> = iter.into_iter().collect();
        Value::Tuple(v)
    }
}

impl Value {
    fn binary_op(&self, right: &Value, primitive: &impl Fn(u32, u32) -> u32) -> Fallible<Value> {
        match (self, right) {
            (Value::Integer(l), Value::Integer(r)) => Ok(Value::Integer(primitive(*l, *r))),
            (Value::Tuple(l), Value::Tuple(r)) => {
                if l.len() != r.len() {
                    bail!(
                        "cannot operate on tuples of different arity ({} vs {})",
                        l.len(),
                        r.len()
                    )
                } else {
                    l.iter()
                        .zip(r)
                        .map(|(l, r)| l.binary_op(r, primitive))
                        .collect()
                }
            }
            (Value::Integer(_), Value::Tuple(_)) | (Value::Tuple(_), Value::Integer(_)) => {
                bail!("cannot operate on tuples and integers together")
            }
        }
    }
}
