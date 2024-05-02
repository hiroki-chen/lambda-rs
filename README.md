# lambda-rs
A lambda calculus interpreter written in Rust: now comes with dependent types!

## Why Dependent Types?

Modern proof assistants like Agda and Coq all utilize dependent types in their core type system and checker due to the expressiveness of DTs for capturing numerous non-trivial theorems and lemmas in hands, with which we can prove a lot of interesting properties like "this crypto algorithm meets the constant-time requirement given a 128-bit key". It is quite intuitive for us to implement a type checker or some sort of interpreter that can work on dependent types. Unfortunately, while there is a plethora of resources aiming to teach people about Simply Typed Lambda Calculus (STLC, or $\vec \lambda$), STLC interpreter with the support for DTs is quite subtle and hard to deal with. This is mainly because now types can themselves be the goal of computation (unification, normalization, and so forth). Consider for example the vector type:

$$v: \prod_{A: \cal U} \prod_{n: \mathbb N} \mathsf{Vec}\ A\ n,$$

$n$ can be a complicated expression that can be reduced, like $S (n)$. When type-checking expressions like $a \equiv b$ at the vector level, we also need to use more complicated inference rules to construct the equality type.

## How?

In the vanilla STLC, terms are just something that can be computed but since types are "simple" enough, we do not have to cope with types in terms. In DT, things get different. A way to encode computable types is to make terms contain types and type universes (levels can be "inferred").

$$ e, \rho ::= \lambda x.e \mid e : \rho \mid \forall x:\rho.\rho' \mid e_1e_2 \mid n \mid x \mid \mathcal{U}_{?} $$

## Usage

You are provided with an interactive shell with the following three commands:

- `def id :: t`: declare a new term with type `t`. 
- `eval e`: evaluate `e`.
- `show`: print the current context.

Some examples:

- **New term declaration:**
  ```shell
  $ cargo run -r --bin pi-interpreter -- --interactive
  Welcome to the Pi interpreter!
  Type 'exit' to quit.

  >>> def a :: ℕ -> ℕ;
  ∀ ℕ . ℕ
  >>> eval a;
  a
  ```
- **Alias and definitions:**
  ```shell
  $ cargo run -r --bin pi-interpreter -- --interactive
  Welcome to the Pi interpreter!
  Type 'exit' to quit.

  >>> let a := ℕ -> ℕ;
  ∀ ℕ . ℕ
  >>> let id := \ x -> x :: a;
  λ . _0
  >>> eval (id id);
  Type mismatch: Type mismatch: expected ℕ, found ∀ ℕ . ℕ
  >>> eval (id 1);
  S(0)
  ```

- Polymorphism:
  ```shell
  $ cargo run -r --bin pi-interpreter -- --interactive
  Welcome to the Pi interpreter!
  Type 'exit' to quit.

  >>> let id := \ a -> \ x -> x :: forall (a : U). a -> a;
  λ . λ . _0
  >>> eval (id Nat 0);
  0
  ```

- Eliminators (WIP):
  ```shell
  Welcome to the Pi interpreter!
  Type 'exit' to quit.

  >>> def NatElim ::
          forall (m : Nat -> U) .
              m 0 -> 
              (forall (l : Nat) . m l -> m (S l)) -> 
              (forall (k : Nat) . m k);
  ∀ ∀ ℕ . 𝒰 . ∀ App (_0)(O) . ∀ ∀ ℕ . ∀ App (_2)(_0) . App (_3)(S(_1)) . ∀ ℕ . App (_3)(_0)
  ```

  You now defined the eliminator for natural numbers! Congratulations!

## Known Issues

- Large numbers will cause *stack overflow* because we represent natural numbers as successors.
- There is no pretty printer so sometimes the output may not be readable.
- The parser is still buggy so some valid expressions will be rejected. Hopefully we can fix it.
- For simplicity now we only have one `Universe`.