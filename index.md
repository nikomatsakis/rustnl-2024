class: center
name: title
count: false

# Type theory for busy engineers

.p60[![Ferris](./images/rustdocs.png)]

.me[.grey[*by* **Nicholas Matsakis**]]
.left[.citation[View slides at `https://nikomatsakis.github.io/rustnl-2024/`]]

---

# Me

---

# What I am here today to talk about

`a-mir-formality`

???

I've been working on this project called "a mir formality".

The idea is to give us a more formal definition of the Rust type system, one we can easily play with.

--

or, more precisely, `formality-core`<sup>1</sup>

.footnote {
    <sup>1</sup> Credit: formality-core is heavily inspired
    by [PLT Redex](https://redex.racket-lang.org/),
    a similar system implemented in the
    Racket programming language.
}

???

But I'm not going to talk about a-mir-formality in detail today.

Instead, I want to talk about the systme that underlies it, dubbed `formality-core`.

---

# formality-core

Rust : Concurrency :: formality-core : Type Systems

???

Rust, thanks to memory safety, makes systems programming into something kind of fun.

The purpose of Rust, in my mind, is to take "systems programming"
from something "wizard-like" to something "everyday".

Formality-core aims to do the same for type 

---

# Introducing `eg`

```rust
let x = 22;
let y = 44;
x + y
```

prints....

--

```
66
```

.p40[![jawdrop](./images/jawdrop.gif)]

---

# But wait, there's more!

```rust
let x = (1, 2);
let y = (22, 44);
x + y
```

prints

```
(23, 46)
```

---

# Functions

```rust
fn add(x: u32, y: u32) -> u32 {
    x + y
}

@add(22, 44)
```

prints

```
66
```

---
name: generics

# Heck, even generics

```rust
fn add<type A>(x: A, y: A) -> A {
    x + y
}

let x = @add<u32>(22, 44);
let y = @add<(u32, u32)>((1, 2), (3, 4));
(x, y)
```

prints

```
(66, (4, 6))
```

---
template: generics

.line5[![Arrow](./images/Arrow.png)]

---
template: generics

.line6[![Arrow](./images/Arrow.png)]

---

# How do we define a program?

```rust
Program = FnDefn* Expr
```

Using a **grammar**.

---
name: program-defn

# In formality-core

```rust
// Program = FnDefn* Expr

#[term($*fn_defns $expr)]
pub struct Program {
    pub fn_defns: Vec<FnDefn>,
    pub expr: Expr,
}
```

---
template: program-defn

.line3[![Arrow](./images/Arrow.png)]

The `#[term]` macro indicates a part of the program AST.

Generates a lot of boilerplate traits.

Includes a grammar used for parsing and pretty-printing.

---
template: program-defn

.arrow.abspos.left90.top145.rotSE[![Arrow](./images/Arrow.png)]

Each `$` parses value of a field

---
template: program-defn

.arrow.abspos.left100.top145.rotSE[![Arrow](./images/Arrow.png)]

The `*` parses a "vec out of zero or more instances"

---
template: program-defn

.arrow.abspos.left200.top145.rotSE[![Arrow](./images/Arrow.png)]

Here we parse a single `Expr` as the value of `expr`

---

# Expression grammar

```rust
Expr = Integer             // e.g., 22
     | Id                  // e.g., `x`
     | ( Expr, ..., Expr )
     | ...
```

---
name: expr

# In formality

```rust
#[term]
pub enum Expr {
    #[grammar($v0)]
    Integer(u32),

    #[grammar($v0)]
    Var(Id),

    #[grammar($(v0))]
    Tuple(Vec<Expr>),

    // ...
}

formality_core::id!(Id);
```

---
template: expr

.arrow.abspos.left20.top95.rotSE[![Arrow](./images/Arrow.png)]

Term on an enum indicates many options

---
template: expr

.arrow.abspos.left45.top180[![Arrow](./images/Arrow.png)]

Each variant has a grammar attached to it

---
template: expr

.arrow.abspos.left220.top155.rotSW[![Arrow](./images/Arrow.png)]

Anonymous fields are called `v0`, `v1`, etc

---
template: expr

.arrow.abspos.left45.top290[![Arrow](./images/Arrow.png)]
.arrow.abspos.left10.top500[![Arrow](./images/Arrow.png)]

`formality_core::id!()` declares an "identifier" -- a string, basically

---
template: expr

.arrow.abspos.left210.top315.rotSW[![Arrow](./images/Arrow.png)]

`$(v0)` parses a comma separated list in parentheses

---

# Expression grammar, cont'd

```rust
Expr = ...
     | Expr + Expr
     | Expr - Expr
     | Expr * Expr
     | Expr / Expr
     | ...
```

---
name: exprop

# In formality

```rust
#[term]
pub enum Expr {
    // ...

    #[grammar($v0 + $v1)]
    #[precedence(0)]
    Add(Arc<Expr>, Arc<Expr>),

    // ...

    #[grammar($v0 * $v1)]
    #[precedence(1)]
    Mul(Arc<Expr>, Arc<Expr>),

    // ...
}
```

---
template: exprop

.arrow.abspos.left45.top262[![Arrow](./images/Arrow.png)]
.arrow.abspos.left40.top422[![Arrow](./images/Arrow.png)]

precedence annotations help avoid parsing ambiguity

---
template: exprop

.arrow.abspos.left100.top322.rotNE[![Arrow](./images/Arrow.png)]

recursive types typically use `Arc`

---
name: exprlet

# Expression grammar, cont'd

```rust
Expr = ...
     | let Id = Expr; Expr  // e.g., let x = 22 + 44; x * 3
     | ...
```

---
template: exprlet

.arrow.abspos.left130.top180.rotNE[![Arrow](./images/Arrow.png)]

Declare a local variable (e.g., `x`)...

---
template: exprlet

.arrow.abspos.left180.top180.rotNE[![Arrow](./images/Arrow.png)]

...with this initial value (e.g., `22+44`)...

---
template: exprlet

.arrow.abspos.left240.top180.rotNE[![Arrow](./images/Arrow.png)]

...and then executes this expression, with the variable in scope.

---
template: exprlet

In formality:

```rust
#[term]
pub enum Expr {
    // ...

    #[grammar(let $v0 = $v1; $v2)]
    Let(Id, Arc<Expr>, Arc<Expr>),

    // ...
}
```

---

# Example 0

```rust
let x = 22;
let y = 44;
x + y * 3 + 66
```

???

At this point we've seen enough grammar to express some non-trivial expressions.

We've also done enough work in formality to start testing our parser.

But before we can do that, I hvae to show you one bit of boilerplate.

---
name: eglang

# Declaring the language

```rust
formality_core::declare_language! {
    mod eg_lang {
        const NAME = "Eg";
        // ... 4 lines I'm not showing you yet ...
        const KEYWORDS = [
            "fn",
            "type",
            "u32",
            "let",
        ];
    }
}

use eg_lang::FormalityLang;
```

---
template: eglang

.arrow.abspos.left100.top180.rotNE[![Arrow](./images/Arrow.png)]

Declare the language module

---
template: eglang

.arrow.abspos.left95.top265.rotNE[![Arrow](./images/Arrow.png)]

Identifiers declared with `Id` will automatically not accept keywords.

---
template: eglang

.arrow.abspos.left10.top500[![Arrow](./images/Arrow.png)]

Macros like `#[term]` reference `crate::FormalityLang`

---

# Let's write some tests

```rust
#[test]
fn parse_expr_let() {
    let e: Expr = eg_lang::term(
        "
            let x = 22;
            let y = 44;
            x + y * 3 + 66
        ",
    );

    // ...
}
```

---
# Type-checking

```rust
22 + 44             // ✅
```

--

```rust
(22, 44) + (66, 88) // ✅
```

--

```rust
(22, 44) + 66       // ❌
```

---
name: egtypes

# Types in eg

```rust
T = u32
  | ( T, ..., T )
  | ...
```

--

in formality:

```rust
#[term]
pub enum Ty {
    #[grammar(u32)]
    U32,

    #[grammar($(v0))]
    Tuple(Vec<Ty>),

    // ...
}
```

---
name: infrules

# Inference rules

![Inference rules](./images/typerules.png)

--

.p40[![Confused](./images/confused.gif)]

---

# Inference rules

.arrow.abspos.left10.top70.rotNE[![Arrow](./images/Arrow.png)]

--

!["So I infer"](./images/so-i-infer.gif)

.arrow.abspos.left230.top400.rotSE[![Arrow](./images/Arrow.png)]

---

# Inference rules

```rust
    X               Y
    -------------------------------- R
    Z
```

Read as: "If X and Y, then Z"

R is just the name of the rule

Sometimes there are no conditions.

---
name: inf-rule-var

# Inference rule for variables

.p40[![Inference rule](./images/inf-rule-var.png)]

.footnote[
    Rendered with https://www.quicklatex.com/
]


---
name: gamma-x-t

# What is this?

.p20[![Gamma](./images/gamma-x-t.png)]

--

A *predicate* -- a single thing we can say is true or false

---
template: gamma-x-t

.arrow.abspos.left160.top170.rotNE[![Arrow](./images/Arrow.png)]

--

*Variables*<sup>1</sup> referencing the grammar.

.footnote[
    <sup>1</sup> More properly, *metavariables*, to distinguish them from the variables
    in the program being typechecked
]

--

T means "some type", like `u32`.

---
template: gamma-x-t

.arrow.abspos.left15.top170.rotNE[![Arrow](./images/Arrow.png)]

&Gamma; is often used for a typing *environment*

---

# Typing environments

Grammar

```rust
Γ = (x : T)*
```

--

In formality:

```rust
#[derive(Clone, Debug, Ord, Eq, PartialEq, PartialOrd, Hash)]
pub struct Env {
    program: Arc<Program>,
    program_variables: Map<Id, Ty>,
}
```

--

.arrow.abspos.left290.top430.rotNE[![Arrow](./images/Arrow.png)]

--

`Γ(x) = T` becomes `env.program_variables[&x]`

---
template: gamma-x-t

The other stuff? 

--

.arrow.abspos.left50.top170.rotNE[![Arrow](./images/Arrow.png)]

--

.arrow.abspos.left130.top170.rotNE[![Arrow](./images/Arrow.png)]

--

Arbitrary text. Effectively the 'name' of this predicate is `⊢:`

--

Think of Objective C method names like `insertObject:atIndex:`

```objc
[map insertObject:some_object atIndex:at_index]
```

.arrow.abspos.left70.top410.rotNE[![Arrow](./images/Arrow.png)]
.arrow.abspos.left320.top410.rotNE[![Arrow](./images/Arrow.png)]

---
template: gamma-x-t

Typical convention:

--

* `A ⊢` means "given assumptions `A`..."<sup>1</sup>
* `: T` means "has type `T`", `T` is kind of the "result"

.footnote[
    <sup>1</sup> True story: When I realized I could read `A ⊢ B` as "A *lets us say* B",
    it totally upped my game in terms of my ability to read type system papers.
]

--

So `Γ ⊢ e : T` could be read as...

* Assuming `Γ`, `e` has the type `T`

---
template: gamma-x-t

More programm-y...

* `fn type_expr(env: Env, expr: Expr) -> Ty`

---
# In formality

```rust
judgment_fn! {
    pub fn type_expr(
        env: Env,
        expr: Expr,
    ) => Ty {
        ...
    }
)
```


---
name: in-formality0

# In formality

```rust
judgment_fn! {
    pub fn type_expr(
        env: Env,
        expr: Expr,
    ) => Ty {
        //...
        (
            (let ty = env.program_variable_ty(x)?)
            ------------------------------- ("var")
            (type_expr(env, Expr::Var(x)) => ty)
        )
        //...
    }
)
```

---
template: in-formality0
name: in-formality1

.abspos.left400.top150.width200[![Inference rule](./images/inf-rule-var.png)]

---
template: in-formality1

.arrow.abspos.left250.top420.rotNE[![Arrow](./images/Arrow.png)]

---
template: in-formality1

.arrow.abspos.left300.top420.rotNE[![Arrow](./images/Arrow.png)]

---
template: in-formality1

.arrow.abspos.left400.top420.rotNE[![Arrow](./images/Arrow.png)]

---
template: in-formality1

.arrow.abspos.left240.top300.rotSE[![Arrow](./images/Arrow.png)]

---
template: in-formality1

.arrow.abspos.left500.top300.rotSE[![Arrow](./images/Arrow.png)]

--

.abspos.left200.top450.inset[
```rust
impl Env {
    pub fn program_variable_ty(&self, var: Id) -> Fallible<&Ty> {
        ...
    }
}
```
]

--

.arrow.abspos.left675.top470.rotSE[![Arrow](./images/Arrow.png)]

--

.abspos.left400.top560.inset[
```rust
type Fallible<T> = anyhow::Result<T>;
```
]

---
template: in-formality1

.arrow.abspos.left190.top300.rotSE[![Arrow](./images/Arrow.png)]

---
template: in-formality1

.arrow.abspos.left470.top420.rotNE[![Arrow](./images/Arrow.png)]

---
name: letrule 

# Typing lets

```rust
judgment_fn! {
    pub fn type_expr(
        env: Env,
        expr: Expr,
    ) => Ty {
        //...
        (
            (type_expr(&env, &*initializer) => var_ty)
            (let env = env.with_program_variable(&var, var_ty))
            (type_expr(&env, &*body) => body_ty)
            ------------------------------- ("let")
            (type_expr(env, Expr::Let(var, initializer, body)) => body_ty)
        )
        //...
    }
)
```

---
template: letrule

.arrow.abspos.left160.top480.rotNE[![Arrow](./images/Arrow.png)]

---
template: letrule

.arrow.abspos.left120.top330[![Arrow](./images/Arrow.png)]

---
template: letrule

.arrow.abspos.left120.top360[![Arrow](./images/Arrow.png)]

--

.arrow.abspos.left255.top420.rotNE[![Arrow](./images/Arrow.png)]

---
template: letrule

.arrow.abspos.left120.top390[![Arrow](./images/Arrow.png)]

---
template: letrule

.arrow.abspos.left680.top480.rotNE[![Arrow](./images/Arrow.png)]

---
name: binaryop

# Typing binary operators

```rust
judgment_fn! {
    pub fn type_expr(
        env: Env,
        expr: Expr,
    ) => Ty {
        //...
        (
            (type_expr(&env, &*l) => l_ty)
            (type_expr(&env, &*r) => r_ty)
            (if l_ty == r_ty)
            ------------------------------- ("add")
            (type_expr(env, Expr::Add(l, r)) => l_ty)
        )
        //...
    }
)
```

---
template: binaryop

.arrow.abspos.left120.top330[![Arrow](./images/Arrow.png)]

---
template: binaryop

.arrow.abspos.left120.top360[![Arrow](./images/Arrow.png)]

---
template: binaryop

.arrow.abspos.left120.top390[![Arrow](./images/Arrow.png)]

---
template: binaryop

.abspos.left400.top150.width200[![Inference rule](./images/inf-rule-add.png)]

--

.arrow.abspos.left700.top230.rotNW[![Arrow](./images/Arrow.png)]
.arrow.abspos.left500.top110.rotSE[![Arrow](./images/Arrow.png)]
.arrow.abspos.left700.top110.rotSE[![Arrow](./images/Arrow.png)]

---
# Demo time!

---
# Conclusion

formality-core: making type systems playful

--

still a WIP

--

check out the full demo on your own time!

```bash
git clone git@github.com:nikomatsakis/rustnl-2024
cd rustnl-2024/eg
cargo test --all --all-targets
```