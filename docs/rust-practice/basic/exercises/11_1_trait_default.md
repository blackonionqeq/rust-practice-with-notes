# 11.1 Trait Default

## Goal

Practice trait default methods. This is close to one common use of an abstract class: shared behavior plus required behavior.

Rust does not have abstract classes. A trait can define:

- Required methods: each implementor must write its own version.
- Default methods: shared code that implementors can use without rewriting.

## Project

Create project:

```bash
cargo new practice/basic/11_1_trait_default --name trait_default
```

## Required Trait

Define:

```rust
trait Named {
    fn name(&self) -> &str;

    fn introduce(&self) -> String {
        format!("Hello, I am {}", self.name())
    }
}
```

`name` is required.

`introduce` is a default method. It calls `self.name()`.

## Required Structs

Define:

```rust
struct Student {
    name: String,
    grade: u32,
}
```

Define:

```rust
struct Teacher {
    name: String,
    subject: String,
}
```

## Required Implementations

Implement `Named` for `Student`:

- `name` returns `&self.name`.
- Do not write `introduce`; use the default method.

Implement `Named` for `Teacher`:

- `name` returns `&self.name`.
- Override `introduce` and return `Hello, I teach {subject}. My name is {name}`.

## Main Program

In `main`:

1. Create a `Student` named `black` with grade `6`.
2. Create a `Teacher` named `ferris` with subject `Rust`.
3. Print `student.introduce()`.
4. Print `teacher.introduce()`.

## Constraints

- Use one trait with one required method and one default method.
- `introduce` must call `self.name()` in the default implementation.
- `Student` must use the default `introduce`.
- `Teacher` must override `introduce`.
- Do not use inheritance syntax; Rust does not have classes.
- Do not use generics.
- Do not use `dyn Trait`.
- Do not use `clone`.
