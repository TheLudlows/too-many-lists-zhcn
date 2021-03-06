use std::cell::{RefCell, UnsafeCell, Ref};
use std::rc::Rc;

struct Node<T> {
    elem: T,
    pre: Link<T>,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            elem,
            pre: None,
            next: None,
        }))
    }
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let mut new_node = Node::new(elem);
        match self.head.take() {
            None => {
                self.tail = Some(new_node.clone());
            },
            Some(mut old_head) => {
                old_head.borrow_mut().pre = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_head);
            }
        }
        self.head = Some(new_node);
    }

    pub fn push_back(&mut self, elem: T) {
        let mut new_node = Node::new(elem);
        match self.tail.take() {
            None => {
                self.head = Some(new_node.clone());
            },
            Some(mut old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().pre = Some(old_tail);
            }
        }
        self.tail = Some(new_node);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(mut node) => {
                match node.borrow_mut().next.take() {
                    None => {
                        self.tail = None;
                    }
                    Some(next_node) => {
                        next_node.borrow_mut().pre.take();
                        self.head = Some(next_node);
                    }
                }
                Some(Rc::try_unwrap(node).ok().unwrap().into_inner().elem)
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match self.tail.take() {
            None => None,
            Some(mut node) => {
                match node.borrow_mut().pre.take() {
                    None => {
                        self.head = None;
                    }
                    Some(pre) => {
                        pre.borrow_mut().next.take();
                        self.tail = Some(pre);
                    }
                }
                Some(Rc::try_unwrap(node).ok().unwrap().into_inner().elem)
            }
        }
    }

    pub  fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|node|
           Ref::map(node.borrow(), |ref_node|  &ref_node.elem)
        )
    }

    pub  fn peek_back(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|node|
            Ref::map(node.borrow(), |ref_node|  &ref_node.elem)
        )
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() {}
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }
    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        list.push_front(1); list.push_front(2); list.push_front(3);

        assert_eq!(&*list.peek_front().unwrap(), &3);
    }
}