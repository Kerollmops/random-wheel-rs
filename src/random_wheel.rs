/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   random_wheel.rs                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: crenault <crenault@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2015/07/08 21:18:06 by crenault          #+#    #+#             */
/*   Updated: 2015/07/14 00:42:59 by crenault         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

extern crate rand;

use pack::Pack;
use std::collections::BTreeSet;
use rand::{thread_rng, Rng};

pub struct RandomWheel<T: Clone> {
    /// the sum of all probabilities added
    sum_proba: f32,
    /// all the (probability, data) in a linked-list to pop easily
    cards: BTreeSet<Pack<T>>
}

impl<T: Clone> RandomWheel<T> {
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

    /// add a pack
    pub fn add_pack(&mut self, pack: Pack<T>) {
        self.sum_proba += pack.proba;
        self.cards.insert(pack);
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
        match self.get_pack() {
            Some(p) => Some(&p.data),
            None => None
        }
    }

    // get a random pack from the BTreeSet
    fn get_pack(&self) -> Option<&Pack<T>> {
        if self.len() > 0 {
            let mut dist = self.gen_random_dist();
            for pack in self.cards.iter() {
                dist -= pack.proba;
                if dist <= 0. {
                    return Some(pack);
                }
            }
        }
        None
    }

    /// Removes a randomly peeked element and return it
    pub fn pop(&mut self) -> Option<T> {
        let mut pack: Pack<T>;
        if let Some(p) = self.get_pack() {
            pack = Pack::new(p.proba, p.data.clone());
        }
        else {
            return None;
        }
        self.sum_proba -= pack.proba;
        self.cards.remove(&pack);
        Some(pack.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_tests() {
        let mut rw: RandomWheel<String> = RandomWheel::new();
        rw.push(5., "testing".to_string());
        assert_eq!(rw.len(), 1);
        assert_eq!(rw.sum_proba(), 5.);
        assert_eq!(rw.peek(), Some(&"testing".to_string()));
        assert_eq!(rw.pop(), Some("testing".to_string()));

        rw.push(12., "another".to_string());
        rw.push(7., "one_more".to_string());
        assert_eq!(rw.len(), 2);
        assert_eq!(rw.sum_proba(), 19.);
        rw.pop();
        rw.pop();
        assert_eq!(rw.len(), 0);
    }
}
