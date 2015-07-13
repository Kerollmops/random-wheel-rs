/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   pack.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: crenault <crenault@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2015/07/13 23:13:08 by crenault          #+#    #+#             */
/*   Updated: 2015/07/13 23:15:21 by crenault         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::cmp::{PartialEq, Eq, PartialOrd, Ord, Ordering};

pub struct Pack<T> {
    pub proba: f32,
    pub data: T
}

impl<T> Pack<T> {
	pub fn new(proba: f32, data: T) -> Pack<T> {
		Pack{ proba: proba, data: data }
	}
}

impl<T> PartialEq for Pack<T> {
    fn eq(&self, other: &Pack<T>) -> bool {
        self.proba == other.proba
    }
}

impl<T> PartialOrd for Pack<T> {
    fn partial_cmp(&self, other: &Pack<T>) -> Option<Ordering> {
        match self.proba {
            p if p > other.proba => Some(Ordering::Greater),
            p if p == other.proba => Some(Ordering::Equal),
            p if p < other.proba => Some(Ordering::Less),
            _ => None,
        }
    }
}

impl<T> Eq for Pack<T> {
    //
}

impl<T> Ord for Pack<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.proba {
            p if p > other.proba => Ordering::Greater,
            p if p == other.proba => Ordering::Equal,
            p if p < other.proba => Ordering::Less,
            _ => unreachable!(),
        }
    }
}
