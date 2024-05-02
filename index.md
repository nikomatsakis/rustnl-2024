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

# Ever seen things like this?

![Type rules](./images/typerules.png)

--

.hugerr[🤔]

---

# Rule of thumb

Things look hard because...

--

...they are so simple<sup>1</sup>...

.footnote[
    <sup>1</sup> To be clear, the ideas behind the notation are not always simple.
    But the notation itself is often as big or bigger a barrier.
]

--

...but you don't know the jargon.


--

...or in this case, the **notation**.

---

# How many here know TCL?

--

Me either.

???

Yeah, me either, though I've written a few programs in it once upon a time.

But what I do know is that the main data structure for TCL is *strings*.

---



---

# The "eg" programming language

```
fn foo(x: u32, y: u32) -> u32 {
    let z: (u32, u32) = (x, y);
    z.0 + z.1
}

foo(22)
```

---

# Inference rules

![Type rules](./images/typerules.png)



---

# Inference rules

