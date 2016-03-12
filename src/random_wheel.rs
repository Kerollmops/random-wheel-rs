extern crate rand;
use std::collections::VecDeque;
use self::rand::Rng;

/// a little implementation of a random-wheel.
pub struct RandomWheel<T> {
    /// the sum of all probabilities in this wheel.
    proba_sum: f32,
    /// all the (probability, data) in a linked-list to pop easily.
    cards: VecDeque<(f32, T)>
}

impl<T: Clone> Clone for RandomWheel<T> {
    fn clone(&self) -> RandomWheel<T> {
        RandomWheel{
            proba_sum: self.proba_sum,
            cards: self.cards.clone()
        }
    }
}

/*impl<T> Iterator for RandomWheel<T> {

    // we will be counting with usize
    type Item = (f32, T);

    // next() is the only required method
    fn next(&mut self) -> Option<(f32, T)> {
        self.cards.into_iter().next()
    }
}*/

impl<T> RandomWheel<T> {
    /// create a new random-wheel from vector.
    /// # Example
    ///
    /// ```
    /// use random_wheel::RandomWheel;
    ///
    /// let numbers: Vec<_> = (0..20).collect();
    ///
    /// // default probability is set to 1.0 for each element
    /// let rw: RandomWheel<u8> = RandomWheel::from_vec(numbers);
    /// ```
    pub fn from_vec(vector: Vec<T>) -> RandomWheel<T> {

        RandomWheel {

            proba_sum: vector.len() as f32,
            cards: vector.into_iter().map(|x| (1.0, x)).collect()
        }
    }

    /// create a new empty random-wheel.
    /// # Example
    ///
    /// ```
    /// use random_wheel::RandomWheel;
    ///
    /// let rw: RandomWheel<u8> = RandomWheel::new();
    /// ```
    pub fn new() -> RandomWheel<T> {

        RandomWheel {

            proba_sum: 0.,
            cards: VecDeque::new()
        }
    }

    /// Creates an empty RandomWheel with space for at least n elements.
    /// # Example
    ///
    /// ```
    /// use random_wheel::RandomWheel;
    ///
    /// let numbers: Vec<_> = (0..20).collect();
    /// let mut rw: RandomWheel<u8> = RandomWheel::with_capacity(numbers.len());
    ///
    /// assert_eq!(rw.len(), 0);
    /// ```
    pub fn with_capacity(n: usize) -> RandomWheel<T> {

        RandomWheel {

            proba_sum: 0.,
            cards: VecDeque::with_capacity(n)
        }
    }

    /// Reserves capacity for at least `additional` more elements to be inserted in the given `Ringbuf`.
    /// The collection may reserve more space to avoid frequent reallocations.
    /// # Example
    ///
    /// ```
    /// use random_wheel::RandomWheel;
    ///
    /// let mut rw: RandomWheel<u8> = RandomWheel::new();
    /// rw.reserve(20);
    ///
    /// assert_eq!(rw.len(), 0);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        self.cards.reserve(additional);
    }

    /// Returns the number of elements the RandomWheel can hold without reallocating.
    /// # Example
    ///
    /// ```
    /// use random_wheel::RandomWheel;
    ///
    /// let rw: RandomWheel<u8> = RandomWheel::new();
    ///
    /// println!("actual capacity: {}", rw.capacity());
    /// ```
    pub fn capacity(&self) -> usize {
        self.cards.capacity()
    }

    /// returns the number of elements in the wheel.
    /// # Example
    ///
    /// ```
    /// use random_wheel::RandomWheel;
    ///
    /// let mut rw = RandomWheel::new();
    ///
    /// assert_eq!(rw.len(), 0);
    ///
    /// rw.push(1., 'r');
    /// rw.push(1., 'c');
    /// rw.push(1., 'a');
    ///
    /// assert_eq!(rw.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// remove all elements in this wheel.
    /// # Example
    ///
    /// ```
    /// use random_wheel::RandomWheel;
    ///
    /// let mut rw = RandomWheel::new();
    ///
    /// rw.push(1., 'r');
    /// rw.push(1., 'c');
    /// rw.push(1., 'a');
    ///
    /// assert_eq!(rw.len(), 3);
    ///
    /// rw.clear();
    ///
    /// assert_eq!(rw.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.cards.clear()
    }

    /// returns `true` if this wheel is empty else return `false`.
    /// # Example
    ///
    /// ```
    /// use random_wheel::RandomWheel;
    ///
    /// let mut rw = RandomWheel::new();
    ///
    /// assert_eq!(rw.is_empty(), true);
    ///
    /// rw.push(1., 'r');
    /// rw.push(1., 'c');
    /// rw.push(1., 'a');
    ///
    /// assert_eq!(rw.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// add an element associated with a probability.
    /// # Example
    ///
    /// ```
    /// use random_wheel::RandomWheel;
    ///
    /// let mut rw = RandomWheel::new();
    ///
    /// rw.push(1., 'r');
    /// rw.push(1., 'c');
    /// rw.push(1., 'a');
    ///
    /// assert_eq!(rw.len(), 3);
    /// ```
    pub fn push(&mut self, proba: f32, data: T) {

        assert!(proba > 0.0, "proba {} is lower or equal to zero!", proba);
        self.cards.push_back((proba, data));
        self.proba_sum += proba;
    }

    /// returns total of luck you pushed.
    /// # Example
    ///
    /// ```
    /// use random_wheel::RandomWheel;
    ///
    /// let mut rw = RandomWheel::new();
    ///
    /// rw.push(1.5, 'r');
    /// rw.push(2., 'c');
    /// rw.push(3., 'a');
    ///
    /// assert_eq!(rw.proba_sum(), 6.5);
    /// ```
    pub fn proba_sum(&self) -> f32 {
        self.proba_sum
    }

    /// returns a random distance to browser between 0 and the probabilities sum.
    fn gen_random_dist(&self) -> f32 {

        match self.proba_sum() {

            sum if sum > 0. => rand::thread_rng().gen_range(0., sum),
            _               => 0.
        }
    }

    /// returns a random index in self.cards.
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

    /// returns a ref to the randomly peeked element.
    /// # Example
    ///
    /// ```
    /// use random_wheel::RandomWheel;
    ///
    /// let mut rw = RandomWheel::new();
    ///
    /// rw.push(1., 'r');
    ///
    /// assert_eq!(rw.peek(), Some(&'r'));
    /// assert_eq!(rw.peek(), Some(&'r'));
    /// ```
    pub fn peek(&self) -> Option<&T> {

        if let Some(index) = self.get_random_index() {

            if let Some(&(_, ref data)) = self.cards.get(index) {
                Some(data)
            }
            else { None }
        }
        else { None }
    }

    /// returns a ref to the randomly peeked element.
    /// # Example
    ///
    /// ```
    /// use random_wheel::RandomWheel;
    ///
    /// let mut rw = RandomWheel::new();
    ///
    /// rw.push(1., 'r');
    ///
    /// match rw.peek_mut() {
    ///     Some(val) => *val = 'b',
    ///     None => {}
    /// }
    ///
    /// assert_eq!(rw.peek(), Some(&'b'));
    /// ```
    pub fn peek_mut(&mut self) -> Option<&mut T> {

        if let Some(index) = self.get_random_index() {

            if let Some(&mut (_, ref mut data)) = self.cards.get_mut(index) {
                Some(data)
            }
            else { None }
        }
        else { None }
    }

    /// returns a ref to the randomly peeked element.
    /// # Example
    ///
    /// ```
    /// use random_wheel::RandomWheel;
    ///
    /// let mut rw = RandomWheel::new();
    ///
    /// rw.push(1., 'r');
    ///
    /// assert_eq!(rw.peek_with_proba(), Some((1.0, &'r')));
    /// assert_eq!(rw.peek_with_proba(), Some((1.0, &'r')));
    /// ```
    pub fn peek_with_proba(&self) -> Option<(f32, &T)> {

        if let Some(index) = self.get_random_index() {

            if let Some(&(proba, ref data)) = self.cards.get(index) {
                Some((proba, data))
            }
            else { None }
        }
        else { None }
    }

    /// returns a ref to the randomly peeked element.
    /// # Example
    ///
    /// ```
    /// use random_wheel::RandomWheel;
    ///
    /// let mut rw = RandomWheel::new();
    ///
    /// rw.push(1., 'r');
    ///
    /// match rw.peek_mut_with_proba() {
    ///     Some((_, val)) => *val = 'b',
    ///     None => {}
    /// }
    ///
    /// assert_eq!(rw.peek_with_proba(), Some((1.0, &'b')));
    /// ```
    pub fn peek_mut_with_proba(&mut self) -> Option<(f32, &mut T)> {

        if let Some(index) = self.get_random_index() {

            if let Some(&mut (proba, ref mut data)) = self.cards.get_mut(index) {
                Some((proba, data))
            }
            else { None }
        }
        else { None }
    }

    /// removes a randomly peeked element and return it.
    /// # Example
    ///
    /// ```
    /// use random_wheel::RandomWheel;
    ///
    /// let mut rw = RandomWheel::new();
    ///
    /// rw.push(1., 'r');
    ///
    /// assert_eq!(rw.peek(), Some(&'r'));
    /// assert_eq!(rw.pop(), Some('r'));
    ///
    /// // once you pop the value, it doesn't exist anymore
    /// assert_eq!(rw.peek(), None);
    /// assert_eq!(rw.pop(), None);
    /// ```
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

    /// removes a randomly peeked element and return it.
    /// # Example
    ///
    /// ```
    /// use random_wheel::RandomWheel;
    ///
    /// let mut rw = RandomWheel::new();
    ///
    /// rw.push(1., 'r');
    ///
    /// assert_eq!(rw.peek(), Some(&'r'));
    /// assert_eq!(rw.pop(), Some('r'));
    ///
    /// // once you pop the value, it doesn't exist anymore
    /// assert_eq!(rw.peek(), None);
    /// assert_eq!(rw.pop(), None);
    /// ```
    pub fn pop_with_proba(&mut self) -> Option<(f32, T)> {

        if let Some(index) = self.get_random_index() {

            if let Some((proba, data)) = self.cards.remove(index) {

                self.proba_sum -= proba;
                Some((proba, data))
            }
            else { None }
        }
        else { None }
    }
}
