/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: crenault <crenault@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2015/07/13 19:29:19 by crenault          #+#    #+#             */
/*   Updated: 2015/07/13 19:34:44 by crenault         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

extern crate random_wheel;

use random_wheel::*;

#[test]
fn peek_and_pop_to_empty() {

    let a = 20;
    let mut wheel = RandomWheel::new();
    wheel.push(1., a);

    assert_eq!(wheel.peek(), Some(&a));
    assert_eq!(wheel.pop(), Some(a));
    assert_eq!(wheel.pop(), None);
}
