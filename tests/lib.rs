/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: crenault <crenault@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2015/07/13 19:29:19 by crenault          #+#    #+#             */
/*   Updated: 2015/10/21 22:32:25 by crenault         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

/*#![feature(test)]
extern crate test;*/
extern crate random_wheel;

#[cfg(test)]
mod tests {

    use random_wheel::*;

    // need nightly
    /*use test;
    #[bench]
    fn bench_create(b: &mut test::Bencher) {

        b.iter(|| RandomWheel::<i32>::new());
    }*/

    #[test]
    fn test_peek_and_pop_to_empty() {

        let value = 'r';
        let mut wheel = RandomWheel::new();

        wheel.push(1., value);
        assert_eq!(wheel.peek(), Some(&value));
        assert_eq!(wheel.pop(), Some(value));
        assert_eq!(wheel.pop(), None);
    }

    #[test]
    fn test_check_probability() {

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
    fn test_clear() {

        let mut wheel = RandomWheel::new();

        wheel.push(1., 'a');
        wheel.push(1., 'c');
        wheel.push(1., 'f');
        wheel.push(1., 'r');
        wheel.push(1., 'z');
        assert_eq!(wheel.len(), 5);

        wheel.clear();
        assert_eq!(wheel.len(), 0);
        assert_eq!(wheel.peek(), None);
        assert_eq!(wheel.pop(), None);
    }

    #[test]
    fn test_proba_sum() {

        let mut wheel = RandomWheel::new();

        wheel.push(1., 'a');
        wheel.push(0.5, 'c');
        wheel.push(1.5, 'f');
        wheel.push(3., 'r');
        wheel.push(5.5, 'z');
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
    fn test_no_clone_implement() {
        let mut wheel = RandomWheel::new();

        let noclone = NoClone{ field: 3 };
        wheel.push(1., noclone);
        assert_eq!(wheel.peek(), Some(&noclone));
        assert_eq!(wheel.pop(), Some(noclone));
        assert_eq!(wheel.pop(), None);
    }
    */
}
