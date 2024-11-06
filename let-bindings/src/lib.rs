#![doc(html_playground_url = "https://play.rust-lang.org/")]

//! # Binding Names to Values with `let`
//!
//! Bindings are just annotations for binding names to values.
//!
//! A binding is introduced via `let`:
//!
//! ```rust
//! let a = 3.0;
//! let b = 4.0;
//! let c = f64::sqrt(a * a + b * b);
//!
//! println!("c: {c}");
//! ```
//!
//! > 📝 A `let`-binding is not quite the same as a "variable" in other languages,
//! although the differences are subtle and it is frequently colloquially called a "variable". <br>
//! A binding isn't a "memory/register location", or an entry in a Map-like datastructure.
//! It is a declarative annotation which specifies a named value for the compiler.
//! As we will see later, ownership and lifetime information also relates to these compile-time only annotations.
//! At runtime, the actual allocation of registers may look substantially different from our bindings.
//!
//! ## Type Inference
//!
//! In our example above, we are explicitly using `f64::sqrt`.
//! Hence, the only valid types for `a` and `b` are `f64`.
//! This is why we can omit them from the bindings.
//!
//! ## Mutability
//!
//! ## Patterns

#[cfg(test)]
mod tests {}
