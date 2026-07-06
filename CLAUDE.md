## Project Context

This is a Rust tutorial project. The user is working through "The Rust Programming Language" book (aka "the book").

## Rules

- NEVER write code for the user — this is a learning project
- Offer guidance, hints, explanations, or point in the right direction without giving away answers
- The user knows programming but is learning Rust idioms (ownership, borrowing, pattern matching, etc.)

## Progress

### Hello World (completed)
- First program — raw `rustc` compilation without Cargo
- `println!` macro with `!` indicating it's a macro, not a function
- `fn main()` as the entry point

### Hello Cargo (completed)
- Same hello world but using `cargo new`
- `cargo build` / `cargo run` workflow
- TOML manifest (`Cargo.toml`), `src/` layout convention

### Guessing Game (completed)
- Interactive CLI game — guess a random number 1–100
- `use` imports: `rand::Rng` (trait), `std::cmp::Ordering`, `std::io`
- `let mut` for mutable bindings vs immutable-by-default `let`
- `String::new()` — associated function (like a static method)
- `io::stdin().read_line(&mut guess)` — mutable reference for writing into the buffer
- `.expect()` on `Result` — crash with message on `Err`, unwrap on `Ok`
- Variable shadowing: reuse `guess` name to convert `String` → `u32` (idiomatic type conversion)
- `match` on `guess.trim().parse()` — `Ok(num) => num`, `Err(_) => continue` for input validation
- `match` on `.cmp()` returning `Ordering::Less | Greater | Equal` — exhaustive pattern matching
- `loop {}` for infinite loop, `break` to exit
- `rand::thread_rng().gen_range(1..=100)` — inclusive range expression
- External crate dependency via `Cargo.toml` `[dependencies]`

## Project Structure

- One folder per book chapter/exercise at the repo root
- Each folder is its own Cargo project (`Cargo.toml` + `src/`)
- `hello_world/` is the exception — raw `rustc`, no Cargo

## Rust Notes (things learned along the way)

- `let` is immutable by default; `let mut` to opt into mutation — opposite of most languages
- `&mut` for mutable references — Rust's borrowing system enforces one mutable ref OR many immutable refs
- `println!("{var}")` — inline variable in format string (like C# string interpolation)
- `;` is required — statements vs expressions distinction matters (no semicollon = expression/return value)
- `match` must be exhaustive — compiler forces handling every variant (like a switch that won't let you forget a case)
- Variable shadowing — reusing a name with a new `let` in the same scope is idiomatic for type conversions (not reassignment — it's a new binding)
- `::` for associated functions (like static methods), `.` for method calls on instances
- `Result<T, E>` — Rust's error handling type; `.expect("msg")` unwraps or panics, `match` for graceful handling
- Traits must be in scope to use their methods (`use rand::Rng` to call `.gen_range()`)
- `1..=100` inclusive range, `1..100` exclusive of end — like Python's range but with syntax
