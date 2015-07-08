/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   pack.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: crenault <crenault@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2015/07/08 23:18:57 by crenault          #+#    #+#             */
/*   Updated: 2015/07/08 23:38:56 by crenault         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::cmp::Ordering;

pub struct Pack<'a, T: 'a> {

	pub proba: f32,
	pub data: &'a T
}

impl<'a, T: 'a> Pack<'a, T> {

	pub fn new(proba: f32, data: &'a T) -> Pack<'a, T> {

		Pack{ proba: proba, data: data }
	}
}

impl<'a, T: 'a> PartialOrd for Pack<'a, T> {

	fn partial_cmp(&self, other: &Pack<T>) -> Option<Ordering> {

		match self.proba {

			x if x > other.proba	=> Some(Ordering::Greater),
			x if x == other.proba	=> Some(Ordering::Equal),
			x if x < other.proba	=> Some(Ordering::Less),
			_							=> None
		}
	}
}

impl<'a, T: 'a> PartialEq for Pack<'a, T> {

	fn eq(&self, other: &Pack<T>) -> bool {

		self.proba == other.proba
	}
}

impl<'a, T: 'a> Eq for Pack<'a, T> {}

impl<'a, T: 'a> Ord for Pack<'a, T> {

	fn cmp(&self, other: &Pack<'a, T>) -> Ordering {

		match self.proba {

			x if x > other.proba	=> Ordering::Greater,
			x if x == other.proba	=> Ordering::Equal,
			x if x < other.proba	=> Ordering::Less,
			_							=> unreachable!()
		}
	}
}
