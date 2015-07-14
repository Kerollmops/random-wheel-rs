/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   random_wheel.rs                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: crenault <crenault@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2015/07/08 21:18:06 by crenault          #+#    #+#             */
/*   Updated: 2015/07/14 16:20:25 by crenault         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

extern crate rand;
use std::collections::VecDeque;
use self::rand::{thread_rng, Rng};

/// a little implementation of a random-wheel
/// https://en.wikipedia.org/wiki/Fitness_proportionate_selection
pub struct RandomWheel<T> {
    /// the sum of all probabilities in this wheel
    proba_sum: f32,
    /// all the (probability, data) in a linked-list to pop easily
    cards: VecDeque<(f32, T)>
}

impl<T> RandomWheel<T> {
    /// create a new empty random-wheel
    pub fn new() -> RandomWheel<T> {
        RandomWheel{
            proba_sum: 0.,
            cards: VecDeque::new()
        }
    }

    /// returns the number of elements in the wheel
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// remove all elements in this wheel
    pub fn clear(&mut self) {
        self.cards.clear()
    }

    /// returns `true` if this wheel is empty else return `false`
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// add an element associated with a probability
    pub fn push(&mut self, proba: f32, data: T) {
        // can do: if even then push back, else push_front
        self.cards.push_back((proba, data));
        self.proba_sum += proba;
    }

    /// returns total of luck you pushed
    pub fn proba_sum(&self) -> f32 {
        self.proba_sum
    }

    /// returns a random distance to browser between 0 and the probabilities sum
    fn gen_random_dist(&self) -> f32 {
        match self.proba_sum() {
            sum if sum > 0. => rand::thread_rng().gen_range(0., sum),
            _               => 0.
        }
    }

    /// returns a random index in slef.cards
    fn get_random_index(&self) -> Option<usize> {
        if self.is_empty() == false {
            let mut dist = self.gen_random_dist();
            for (id, &(ref proba, _)) in self.cards.iter().enumerate() {
                dist -= *proba;
                if dist <= 0. {
                    return Some(id);
                }
            }
            None
        }
        else { None }
    }

    /// returns a ref to the randomly peeked element
    pub fn peek(&self) -> Option<&T> {
        if let Some(index) = self.get_random_index() {
            if let Some(&(_, ref data)) = self.cards.get(index) {
                Some(data)
            }
            else { None }
        }
        else { None }
    }

    /// removes a randomly peeked element and return it
    pub fn pop(&mut self) -> Option<T> {
        if let Some(index) = self.get_random_index() {
            if let Some((proba, data)) = self.cards.remove(index) {
                self.proba_sum -= proba;
                Some(data)
            }
            else { None }
        }
        else { None }
    }
}
