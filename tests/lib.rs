/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: crenault <crenault@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2015/07/13 19:29:19 by crenault          #+#    #+#             */
/*   Updated: 2015/07/14 22:51:21 by crenault         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

extern crate random_wheel;

use random_wheel::*;

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
    let numbers: Vec<_> = (0..10).collect();
    let mut wheel = RandomWheel::with_capacity(numbers.len());

    // need: wheel.push_vec(proba: f32, vec: Vec<T>);
    for x in numbers.iter() {
        wheel.push(1., x);
    }

    let mut total = Vec::new();
    for _ in (0..5_000) {
        total.push(wheel.peek());
    }

    // settings
    let margin_of_error = 0.20;
    let max_bound = 1. + margin_of_error;
    let min_bound = 1. - margin_of_error;

    // test
    for x in numbers.iter() {
        let it = total.iter().filter(|&a| a == &Some(&x));
        let proba = it.count() as f32 / total.len() as f32 * numbers.len() as f32;
        assert!(proba <= max_bound && proba >= min_bound,
            "proba: {}, max bound: {}, min bound: {}", proba, max_bound, min_bound);
    }
}

#[test]
fn clear_test() {
    let mut wheel = RandomWheel::new();

    wheel.push(1., 5);
    wheel.push(1., 3);
    wheel.push(1., 10);
    wheel.push(1., 21);
    wheel.push(1., 0);
    assert_eq!(wheel.len(), 5);

    wheel.clear();
    assert_eq!(wheel.len(), 0);
    assert_eq!(wheel.peek(), None);
    assert_eq!(wheel.pop(), None);
}

#[test]
fn proba_sum() {
    let mut wheel = RandomWheel::new();

    wheel.push(1., 5);
    wheel.push(0.5, 3);
    wheel.push(1.5, 10);
    wheel.push(3., 21);
    wheel.push(5.5, 0);
    assert_eq!(wheel.proba_sum(), 11.5);
}

/*
// no clone struct
#[derive(PartialEq, Eq, Debug)] // for assert!
struct NoClone {
    field: i32
}

// not properly working without clone
#[test]
fn no_clone_implement() {
    let mut wheel = RandomWheel::new();

    let noclone = NoClone{ field: 3 };
    wheel.push(1., noclone);
    assert_eq!(wheel.peek(), Some(&noclone));
    assert_eq!(wheel.pop(), Some(noclone));
    assert_eq!(wheel.pop(), None);
}
*/
