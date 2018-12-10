use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

pub type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            prev: None,
            next: None,
        }))
    }
}

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
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::clone(&new_head));
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(Rc::clone(&new_head));
                self.head = Some(new_head);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }

            match Rc::try_unwrap(old_head) {
                Ok(old_head) => old_head.into_inner().elem,
                Err(_) => panic!("(pop_front) multiple references to 'old_head'."),
            }
        })
    }

    pub fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(Rc::clone(&new_tail));
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.tail = Some(Rc::clone(&new_tail));
                self.head = Some(new_tail);
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }

            match Rc::try_unwrap(old_tail) {
                Ok(old_tail) => old_tail.into_inner().elem,
                Err(_) => panic!("(pop_back) multiple references to 'old_tail'."),
            }
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn peek_front_mut(&self) -> Option<RefMut<T>> {
        self.head
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn peek_back_mut(&self) -> Option<RefMut<T>> {
        self.tail
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }

    // --- intrusive operations ---

    pub fn get_front_link(&self) -> Link<T> {
        self.head.as_ref().map(|node| Rc::clone(node))
    }

    pub fn get_back_link(&self) -> Link<T> {
        self.tail.as_ref().map(|node| Rc::clone(node))
    }

    pub fn get_next_link(&self, link: &Link<T>) -> Link<T> {
        match link {
            Some(node) => node.borrow().next.as_ref().map(|next| Rc::clone(next)),
            None => None,
        }
    }

    pub fn get_prev_link(&self, link: &Link<T>) -> Link<T> {
        match link {
            Some(node) => node.borrow().prev.as_ref().map(|prev| Rc::clone(prev)),
            None => None,
        }
    }

    pub fn get_nth_next_link_cyclic(&self, link: &Link<T>, n: u32) -> Link<T> {
        let mut current_link = link.as_ref().map(|link| Rc::clone(link));

        for _ in 0..n {
            let next_link = match self.get_next_link(&current_link) {
                Some(next) => Some(next),
                None => self.get_front_link(),
            };
            current_link = next_link;
        }

        current_link
    }

    pub fn get_nth_prev_link_cyclic(&self, link: &Link<T>, n: u32) -> Link<T> {
        let mut current_link = link.as_ref().map(|link| Rc::clone(link));

        for _ in 0..n {
            let prev_link = match self.get_prev_link(&current_link) {
                Some(prev) => Some(prev),
                None => self.get_back_link(),
            };
            current_link = prev_link;
        }

        current_link
    }

    pub fn pop_link(&mut self, mut link: Link<T>) -> Option<T> {
        link.take().map(|popped_node| {
            let next = popped_node.borrow_mut().next.take();
            let prev = popped_node.borrow_mut().prev.take();

            match (next, prev) {
                (Some(next), Some(prev)) => {
                    next.borrow_mut().prev = Some(Rc::clone(&prev));
                    prev.borrow_mut().next = Some(Rc::clone(&next));
                }
                (None, Some(new_tail)) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                (Some(new_head), None) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                (None, None) => {
                    self.head.take();
                    self.tail.take();
                }
            }

            match Rc::try_unwrap(popped_node) {
                Ok(popped_node) => popped_node.into_inner().elem,
                Err(_) => panic!("(pop_link) multiple references to 'popped_node'."),
            }
        })
    }

    pub fn push_after_link(&mut self, link: &Link<T>, elem: T) {
        let new_node = Node::new(elem);
        match link {
            Some(current_node) => {
                match current_node.borrow_mut().next.take() {
                    Some(next) => {
                        next.borrow_mut().prev = Some(Rc::clone(&new_node));
                        new_node.borrow_mut().next = Some(next);
                    }
                    None => {
                        self.tail = Some(Rc::clone(&new_node));
                    }
                };

                new_node.borrow_mut().prev = Some(Rc::clone(&current_node));
                current_node.borrow_mut().next = Some(new_node);
            }
            None => {
                self.tail = Some(Rc::clone(&new_node));
                self.head = Some(new_node);
            }
        }
    }

    pub fn peek_link<'a>(&'a self, link: &'a Link<T>) -> Option<Ref<T>> {
        link.as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn peek_link_mut<'a>(&'a self, link: &'a Link<T>) -> Option<RefMut<T>> {
        link.as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn front_operations() {
        let mut list = List::new();

        assert_eq!(list.pop_front(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(*list.peek_front().unwrap(), 3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(*list.peek_front().unwrap(), 2);
        assert_eq!(list.pop_front(), Some(2));

        list.push_front(4);
        list.push_front(5);

        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(*list.peek_front().unwrap(), 1);
        assert_eq!(list.pop_front(), Some(1));
        assert!(list.peek_front().is_none());
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn back_operations() {
        let mut list = List::new();

        assert_eq!(list.pop_back(), None);

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(*list.peek_back().unwrap(), 3);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(*list.peek_back().unwrap(), 2);
        assert_eq!(list.pop_back(), Some(2));

        list.push_back(4);
        list.push_back(5);

        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(*list.peek_back().unwrap(), 1);
        assert_eq!(list.pop_back(), Some(1));
        assert!(list.peek_back().is_none());
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn front_and_back_operations() {
        let mut list = List::new();

        assert_eq!(list.pop_back(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), Some(2));

        list.push_back(4);
        list.push_back(5);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn intrusive_operations() {
        let mut list = List::new();

        let link = list.get_front_link();
        assert!(link.is_none());

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        list.push_back(5);
        list.push_back(6);
        list.push_back(7);
        list.push_back(8);

        let link = list.get_front_link();
        assert_eq!(list.pop_link(link), Some(1));

        let link = list.get_front_link();
        assert_eq!(*list.peek_link(&link).unwrap(), 2);
        let link = list.get_next_link(&link);
        assert_eq!(*list.peek_link(&link).unwrap(), 3);
        let link = list.get_nth_next_link_cyclic(&link, 3);
        assert_eq!(*list.peek_link(&link).unwrap(), 6);
        let link = list.get_prev_link(&link);
        assert_eq!(*list.peek_link(&link).unwrap(), 5);
        let link = list.get_nth_prev_link_cyclic(&link, 3);
        assert_eq!(*list.peek_link(&link).unwrap(), 2);

        list.push_front(1);

        let link = list.get_prev_link(&link);
        assert_eq!(*list.peek_link(&link).unwrap(), 1);
        let link = list.get_nth_prev_link_cyclic(&link, 3);
        assert_eq!(*list.peek_link(&link).unwrap(), 6);
        let link = list.get_nth_next_link_cyclic(&link, 4);
        assert_eq!(*list.peek_link(&link).unwrap(), 2);

        let link = list.get_back_link();
        list.push_after_link(&link, 10);
        list.push_after_link(&link, 9);
        drop(link);

        assert_eq!(list.pop_back(), Some(10));
        assert_eq!(list.pop_back(), Some(9));
        assert_eq!(list.pop_back(), Some(8));
    }
}
