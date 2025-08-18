struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}
// checking if the code is correct

pub struct SimpleLinkedList<T> {
    // Delete this field
    // dummy is needed to avoid unused parameter error during compilation
    // dummy: ::std::marker::PhantomData<T>,
    len: usize,
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { len: 0, head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, _element: T) {
        let temp = Some(Box::new(Node {
            next: self.head.take(),
            val: _element,
        }));
        self.len += 1;
        self.head = temp;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(boxed_node) => {
                self.head = boxed_node.next;
                self.len -= 1;
                Some(boxed_node.val)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            None => None,
            Some(node) => Some(&node.val),
        }
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let (mut prev, mut curr) = (None, self.head.take());
        while let Some(mut boxed_curr) = curr {
            let next = boxed_curr.next.take();
            boxed_curr.next = prev;
            prev = Some(boxed_curr);
            curr = next;
        }
        self.head = prev;
        self
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        _iter
            .into_iter()
            .fold(SimpleLinkedList::new(), |mut acc, el| {
                acc.push(el);
                acc
            })
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.
//
// Please note that the "front" of the linked list should correspond to the "back"
// of the vector as far as the tests are concerned.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut ans = vec![];
        _linked_list=_linked_list.rev();
        while !_linked_list.is_empty() {
            ans.push(_linked_list.pop().unwrap());
        };
        ans
    }
}
