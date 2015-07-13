/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   random_wheel.rs                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: crenault <crenault@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2015/07/08 21:18:06 by crenault          #+#    #+#             */
/*   Updated: 2015/07/13 19:28:36 by crenault         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

/*
A Little implementation of the random-wheel principle, `RandomWheel<T>`.

# Examples

You can explicitly create a `RandomWheel<T>` with `new()`:

```
let rw: RandomWheel<i32> = RandomWheel::new();
```

You can `push` values onto the random-wheel (which will grow the wheel as needed):

Popping values works in much the same way:

```
let one = 1;
let two = 2;
{
    let mut rw = RandomWheel::new();

    rw.push(&one);
    rw.push(&two);

    let one_or_two = rw.pop();
}
```

*/

extern crate rand;
extern crate linked_list;
use self::linked_list::LinkedList;
use self::rand::{thread_rng, Rng};

pub struct RandomWheel<T> {

	sum_proba: f32,
	cards: LinkedList<(f32, T)>
}

impl<T> RandomWheel<T> {

	/// create a new empty random-wheel
	pub fn new() -> RandomWheel<T> {

		RandomWheel{
			sum_proba: 0.,
			cards: LinkedList::new()
		}
	}

	/// Returns the number of elements in the wheel.
	pub fn len(&self) -> usize {

		self.cards.len()
	}

	/// add an element associated with a probability
	pub fn push(&mut self, proba: f32, data: T) {

		// can do: if even then push back, else push_front
		self.cards.push_back((proba, data));
		self.sum_proba += proba;
	}

	/// return total of luck you add with push
	pub fn sum_proba(&self) -> f32 {

		self.sum_proba
	}

	/// return a random distance to browser between 0 and the probabilities sum
	fn gen_random_dist(&self) -> f32 {

		rand::thread_rng().gen_range(0., self.sum_proba())
	}

	/// return a ref to the randomly peeked element
	pub fn peek(&self) -> Option<&T> {

		if self.len() == 0 {
			None
		}
		else {
			let mut dist = self.gen_random_dist();
			for &(proba, ref data) in self.cards.iter() {

				dist -= proba;
				if dist <= 0. {
					return Some(data);
				}
			}
			None
		}
	}

	/// Removes a randomly peeked element and return it
	pub fn pop(&mut self) -> Option<T> {

		let mut dist = self.gen_random_dist();
		let mut chosen_id = None;
		for (index, &(proba, _)) in self.cards.iter().enumerate() {

			dist -= proba;
			if dist <= 0. {
				chosen_id = Some(index);
				break;
			}
		}
		// return and remove data
		if let Some(id) = chosen_id {
			if let Some((_, data)) = self.cards.remove(id) {
				Some(data)
			}
			else { None }
		}
		else { None }
	}
}
