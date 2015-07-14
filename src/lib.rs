/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: crenault <crenault@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2015/07/08 21:14:36 by crenault          #+#    #+#             */
/*   Updated: 2015/07/14 22:05:20 by crenault         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

//! A Little implementation of the random-wheel principle, `RandomWheel<T>`.
//! https://en.wikipedia.org/wiki/Fitness_proportionate_selection
//!
//! # Examples
//!
//! You can explicitly create a `RandomWheel<T>` with `new()`:
//!
//! ```
//! use random_wheel::RandomWheel;
//!
//! let rw: RandomWheel<i32> = RandomWheel::new();
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
//! rw.push(5., 1);
//! rw.push(1., 2);
//!
//! // you have 5 chances out of 6 to hit $one
//! let one_or_two = rw.pop();
//! ```

mod random_wheel;
pub use random_wheel::RandomWheel;
