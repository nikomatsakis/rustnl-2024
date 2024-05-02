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

or, more precisely, `formality-core`

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

```
66
```

--

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

Term macro indicates a part of the program.

Generates a lot of traits, including one for parsing.

---
template: program-defn

.arrow.abspos.left90.top145.rotSE[![Arrow](./images/Arrow.png)]

In the grammar, each `$` means "recursively parse to get value of this field"

---
template: program-defn

.arrow.abspos.left100.top145.rotSE[![Arrow](./images/Arrow.png)]

The `*` means "parse a vec out of zero or more instances"

---
template: program-defn

.arrow.abspos.left200.top145.rotSE[![Arrow](./images/Arrow.png)]

Here we parse the value of `expr`

---

# Expression grammar

```rust
Expr = Integer
     | Variable
     | Expr + Expr         // and other binary operators
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

.arrow.abspos.left210.top315.rotSW[![Arrow](./images/Arrow.png)]

`$(v0)` parses a comma separated list in parentheses