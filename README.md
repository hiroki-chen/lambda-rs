# lambda-rs
A lambda calculus interpreter written in Rust: now comes with dependent types!

## Why Dependent Types?

Modern proof assistants like Agda and Coq all utilize dependent types in their core type system and checker due to the expressiveness of DTs for capturing numerous non-trivial theorems and lemmas in hands, with which we can prove a lot of interesting properties like "this crypto algorithm meets the constant-time requirement given a 128-bit key". It is quite intuitive for us to implement a type checker or some sort of interpreter that can work on dependent types. Unfortunately, while there is a plethora of resources aiming to teach people about Simply Typed Lambda Calculus (STLC, or $\vec \lambda$), STLC interpreter with the support for DTs is quite subtle and hard to deal with. This is mainly because now types can themselves be the goal of computation (unification, normalization, and so forth). Consider for example the vector type:

$$v: \prod_{A: \cal U} \prod_{n: \mathbb N} \mathsf{Vec}\ A\ n,$$

$n$ can be a complicated expression that can be reduced, like $S (n)$. When type-checking expressions like $a \equiv b$ at the vector level, we also need to use more complicated inference rules to construct the equality type.

## How?

In the vanilla STLC, terms are just something that can be computed but since types are "simple" enough, we do not have to cope with types in terms. In DT, things get different. A way to encode computable types is to make terms contain types and type universes (levels can be "inferred").

$$ e, \rho ::= \lambda x.e \mid e : \rho \mid \forall x:\rho.\rho' \mid e_1e_2 \mid n \mid x \mid \mathcal{U}_{?} $$
