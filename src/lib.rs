/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: crenault <crenault@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2015/07/08 21:14:36 by crenault          #+#    #+#             */
/*   Updated: 2015/07/13 19:22:38 by crenault         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

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
