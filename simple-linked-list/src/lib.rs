use std::iter::FromIterator;

pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut len = 0_usize;
        let mut node = &self.head;
        while node.is_some() {
            len += 1;
            node = &node.as_ref().unwrap().next;
        }

        len
    }

    pub fn push(&mut self, _element: T) {
        self.head = Some(Box::new(Node {
            value: _element,
            next: self.head.take(),
        }))
    }

    pub fn pop(&mut self) -> Option<T> {
        if let None = self.head {
            None
        } else {
            let mut node = self.head.take().unwrap();
            self.head = node.next.take();
            Some(node.value)
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if let None = self.head {
            None
        } else {
            Some(&self.head.as_ref().unwrap().value)
        }
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let v: Vec<_> = self.into();
        v.into_iter().rev().collect()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = Self::new();
        _iter.into_iter().for_each(|x| list.push(x));
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut v = Vec::new();
        while _linked_list.head.is_some() {
            v.push(_linked_list.pop().unwrap())
        }
        v.reverse();
        v
    }
}
