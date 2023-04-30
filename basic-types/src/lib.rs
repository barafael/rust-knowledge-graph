//! # Basic Types
//!
//! The simplest types, like [`bool`] and [`u32`], are fundamental to datastructures, traits, and functions.
//! They are the easiest to work with (due to the [`Copy`] trait).
//! They all have dedicated (lowercase) keywords.
//! The types of many simple literals and expressions are inferred reliably.
//! For integers, when there are no other constraints, the default type is [`i32`].
//!
//! ```rust
//! let no /*: bool */= false;
//! let yes /*: bool */ = !no;
//!
//! let sum /*: i32 */ = 1 + 2 + 3 + 4;
//! ```
#[cfg(test)]
mod tests {}
