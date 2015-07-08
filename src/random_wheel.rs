/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   random_wheel.rs                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: crenault <crenault@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2015/07/08 21:18:06 by crenault          #+#    #+#             */
/*   Updated: 2015/07/08 23:28:12 by crenault         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

extern crate rand;
use self::rand::{thread_rng, Rng};
use std::collections::BinaryHeap;
use pack::Pack;

pub struct RandomWheel<'a, T: 'a> {

	sum_proba: f32,
	cards: BinaryHeap<Pack<'a, T>>
}

impl<'a, T: 'a> RandomWheel<'a, T> {

	pub fn new() -> RandomWheel<'a, T> {

		RandomWheel{
			sum_proba: 0.,
			cards: BinaryHeap::<Pack<T>>::new()
		}
	}

	pub fn with_capacity(capacity: usize) -> RandomWheel<'a, T> {

		RandomWheel{
			sum_proba: 0.,
			cards: BinaryHeap::<Pack<T>>::with_capacity(capacity)
		}
	}

	/// add an element associated with a probability
	pub fn push(&mut self, proba: f32, data: &'a T) {

		let pack = Pack::new(proba, data);
		self.cards.push(pack);
		self.sum_proba += proba;
	}

	/// return total of luck you add with push
	pub fn sum_proba(&self) -> f32 {

		self.sum_proba
	}

	/// return a ref to the randomly peeked element
	pub fn peek(&self) -> Option<&'a T> {

		let mut rand = rand::thread_rng().gen_range(0., self.sum_proba());
		if self.cards.len() != 0 {

			let mut peeked = None;
			for x in self.cards.iter() {

				rand -= x.proba;
				if rand == 0. {
					peeked = Some(x.data);
					break;
				}
			}
			peeked
		}
		else { None }
	}

	/// Removes a randomly peeked element and return it
	pub fn pop(&mut self) -> Option<&'a T> {

		None
	}
}
