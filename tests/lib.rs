extern crate random_wheel;

use random_wheel::*;

#[test]
fn peek_and_pop_to_empty() {
	let a = 20;
	let mut wheel = RandomWheel::new();
	wheel.push(1., &a);

	assert_eq!(wheel.peek(), Some(&a));
	assert_eq!(wheel.pop(), Some(&a));
	assert_eq!(wheel.pop(), None);
}
