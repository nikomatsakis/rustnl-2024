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

# My Goal with this talk

???

Rust has a problem

We need better tools for communication designs with precision

But 

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

