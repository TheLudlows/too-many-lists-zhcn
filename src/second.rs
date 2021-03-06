struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, v: T) {
        let next = self.head.take();
        let node = Box::new(Node {
            elem: v,
            next,
        });
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        match self.head {
            None => { None }
            Some(ref node) => {
                Some(&node.elem)
            }
        }

        // self.head.as_ref().map(|node| &node.elem)
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        //Iter { cur: self.head.as_ref().map(|node| &**node) }
        Iter { cur: self.head.as_deref() }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { cur: self.head.as_deref_mut() }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    cur: Option<&'a Node<T>>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur.map(|node| {
            self.cur = node.next.as_deref();
            &node.elem
        })
        /*match self.cur {
            Some(node) => {
                let r = Some(&node.elem);
                self.cur = node.next.as_deref();
                r
            },
            None => None
        }*/
    }
}

pub struct IterMut<'a, T> {
    cur: Option<&'a mut Node<T>>
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur.take().map(|node| {
            self.cur = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}


impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur = self.head.take();
        while let Some(mut node) = cur {
            cur = node.next.take()
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);

        assert_eq!(list.peek(), None);

        list.push(1);
        assert_eq!(list.peek(), Some(&1));

        let mut _node = list.peek_mut().map(|i| *i = 10);
        println!("{:?}", list.peek());

        for i in list.into_iter() {
            println!("{:?}", i);
        }
        let mut v = List::new();
        v.push(1);

        for i in v.iter() {
            println!("{:?}", i);
        }

        let mut v = List::new();
        v.push(3);
        for p in v.iter_mut() {
            *p += 1;
        }
        for i in v.iter() {
            println!("{}", i);
        }
    }

    #[test]
    fn test_option() {
        let some = Some(1);
        println!("{:p}", &some);
        println!("{:p}", some.as_ref().unwrap());
    }

    #[test]
    fn test_mut() {}
}
