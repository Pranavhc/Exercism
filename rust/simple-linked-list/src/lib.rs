pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T, next: Option<Box<Node<T>>>) -> Self {
        Self { data, next }
    }
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    pub fn push(&mut self, _element: T) {
        let node = Node::new(_element, self.head.take());
        self.head = Some(Box::new(node));
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                self.length -= 1;
                Some(node.data)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            None => None,
            Some(node) => Some(&node.data),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn len(&self) -> usize {
        self.length.clone()
    }

    #[must_use]
    pub fn rev(&mut self) -> SimpleLinkedList<T> {
        let mut curr = self.head.take();
        let mut prev: Option<Box<Node<T>>> = None;

        while let Some(mut node) = curr.take() {
            curr = node.next.take();
            node.next = prev.take();
            prev = Some(node);
        }

        Self {
            head: prev,
            length: self.length,
        }
    }
}

// defines how SimpleLinkedList will be created from an iterator of a collection such as Vec
impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut ll = SimpleLinkedList::<T>::new();
        for i in _iter {
            ll.push(i);
        }
        ll
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//  let vec: Vec<_> = simple_linked_list.into_iter().collect();

// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

// Converts LinkedList into Vec - provides .into() method to the SimpleLinkedList<T>
impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut curr = _linked_list.head;
        let mut list = Vec::<T>::new();

        while let Some(node) = curr {
            list.push(node.data);
            curr = node.next;
        }
        list.reverse();
        list
    }
}
