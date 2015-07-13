/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   random_wheel.rs                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: crenault <crenault@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2015/07/08 21:18:06 by crenault          #+#    #+#             */
/*   Updated: 2015/07/13 23:48:08 by crenault         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

extern crate rand;
use pack::Pack;
use std::collections::BTreeSet;
use self::rand::{thread_rng, Rng};

pub struct RandomWheel<T> {
    /// the sum of all probabilities added
    sum_proba: f32,
    /// all the (probability, data) in a linked-list to pop easily
    cards: BTreeSet<Pack<T>>
}

impl<T> RandomWheel<T> {
    /// create a new empty random-wheel
    pub fn new() -> RandomWheel<T> {
        RandomWheel{
            sum_proba: 0.,
            cards: BTreeSet::new()
        }
    }

    /// Returns the number of elements in the wheel.
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// add an element associated with a probability
    pub fn push(&mut self, proba: f32, data: T) {
        // can do: if even then push back, else push_front
        //self.cards.push_back((proba, data));
        self.cards.insert(Pack::new(proba, data));
        self.sum_proba += proba;
    }

    /// return total of luck you add with push
    pub fn sum_proba(&self) -> f32 {
        self.sum_proba
    }

    /// return a random distance to browser between 0 and the probabilities sum
    fn gen_random_dist(&self) -> f32 {
        match self.sum_proba() {
            x if x > 0. => rand::thread_rng().gen_range(0., self.sum_proba()),
            _           => 0.
        }
    }

    /// return a ref to the randomly peeked element
    pub fn peek(&self) -> Option<&T> {
        if self.len() == 0 {
            None
        }
        else {
            let mut dist = self.gen_random_dist();
            for &Pack{ proba, ref data } in self.cards.iter() {
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
        /*if self.len() == 0 {
            None
        }
        else {
            let mut dist = self.gen_random_dist();
            let mut to_ret_del = None;
            for &pack in self.cards.iter() {
                let Pack{ ref proba, .. } = pack;
                dist -= *proba;
                if dist <= 0. {
                    to_ret_del = Some(pack);
                    break;
                }
            }
            if let Some(pack) = to_ret_del {
                let Pack{ ref data, .. } = pack;
                self.cards.remove(&pack);
                return Some(*data);
            }
            None
        }*/
        None
    }
}
