fn main() {
    // for loop with inclusive range (1..=100 includes 100, 1..100 would stop at 99)
    // `i` is immutable — you get a fresh binding each iteration, no `let mut` needed
    for i in 1..=100 {
        // check the combined case FIRST — if you check % 3 or % 5 first,
        // numbers like 15/30/45 would match early and never reach the combined check
        if i % 15 == 0 {
            println!("Fizzbuzz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else {
            // {} is a format placeholder — Rust's println! is a macro (note the !)
            // that does compile-time format string checking
            println!("{}", i);
        }
    }
}
