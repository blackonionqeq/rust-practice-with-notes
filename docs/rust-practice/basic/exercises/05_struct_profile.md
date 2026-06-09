# 05 Struct Profile

## Goal

Practice struct definition, struct construction, field access, struct update syntax, and `Debug` output.

## Project

Create project:

```bash
cargo new practice/basic/05_struct_profile --name struct_profile
```

## Required Struct

Define:

```rust
#[derive(Debug)]
struct UserProfile {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

## Required Function

```rust
fn build_user(username: String, email: String) -> UserProfile
```

Return a `UserProfile` where:

- `username` comes from the parameter.
- `email` comes from the parameter.
- `sign_in_count` is `1`.
- `active` is `true`.

## Main Program

In `main`:

1. Build a user named `black` with email `black@example.com`.
2. Print the username and email through field access.
3. Create a second user with struct update syntax:

```rust
let user2 = UserProfile {
    email: String::from("second@example.com"),
    ..user1
};
```

4. Print `user2` with `Debug`.

## Constraints

- Use field init shorthand in `build_user`.
- Use struct update syntax for `user2`.
- Do not use `clone`.
- After creating `user2` with `..user1`, do not use moved fields from `user1`.
