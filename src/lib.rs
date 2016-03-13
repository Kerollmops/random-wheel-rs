//! A Little implementation of the random-wheel principle, `RandomWheel<T>`.
//! https://wikipedia.org/wiki/Fitness_proportionate_selection
//!
//! # Usage
//!
//! You can get this package from: [crates.io/random-wheel](https://crates.io/crates/random-wheel).
//!
//! # Examples
//!
//! You can explicitly create a `RandomWheel<T>` with `new()`:
//!
//! ```
//! use random_wheel::RandomWheel;
//!
//! let rw: RandomWheel<u8> = RandomWheel::new();
//! ```
//!
//! You can `push` values onto the random-wheel (which will grow the wheel as needed):
//!
//! Popping values works in much the same way:
//!
//! ```
//! use random_wheel::RandomWheel;
//!
//! let mut rw = RandomWheel::new();
//!
//! rw.push(5., 'a');
//! rw.push(1., 'b');
//!
//! // you have 5 chances out of 6 to pop 'a'
//! let a_or_b = rw.pop();
//! ```

mod random_wheel;
pub use random_wheel::RandomWheel;
