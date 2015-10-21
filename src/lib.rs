/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: crenault <crenault@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2015/07/08 21:14:36 by crenault          #+#    #+#             */
/*   Updated: 2015/10/21 22:38:20 by crenault         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

//! A Little implementation of the random-wheel principle, `RandomWheel<T>`.
//! https://wikipedia.org/wiki/Fitness_proportionate_selection
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
