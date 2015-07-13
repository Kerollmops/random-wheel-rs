/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: crenault <crenault@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2015/07/08 21:14:36 by crenault          #+#    #+#             */
/*   Updated: 2015/07/13 19:55:57 by crenault         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

//! A Little implementation of the random-wheel principle, `RandomWheel<T>`.
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
//! let one = 1;
//! let two = 2;
//!
//! let mut rw = RandomWheel::new();
//!
//! rw.push(5., one);
//! rw.push(1., two);
//!
//! // you get 5 on 6 luck to get one
//! let one_or_two = rw.pop();
//!
//! ```

mod random_wheel;
pub use random_wheel::RandomWheel;

#[test]
fn peek_and_pop_to_empty() {

    let a = 20;
    let mut wheel = RandomWheel::new();
    wheel.push(1., a);

    assert_eq!(wheel.peek(), Some(&a));
    assert_eq!(wheel.pop(), Some(a));
    assert_eq!(wheel.pop(), None);
}

#[test]
fn check_probability() {

    let numbers: Vec<_> = (0..100).collect();

    //
}
