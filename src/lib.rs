/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: crenault <crenault@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2015/07/08 21:14:36 by crenault          #+#    #+#             */
/*   Updated: 2015/07/09 14:33:29 by crenault         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub mod random_wheel;

use random_wheel::RandomWheel;

#[test]
fn peek_and_pop_to_empty() {

	let a = 20;

	{
		let mut wheel = RandomWheel::new();
		wheel.push(1., &a);

		assert!(wheel.peek() == Some(&a));
		assert!(wheel.pop() == Some(&a));
		assert!(wheel.pop() == None);
	}
}
