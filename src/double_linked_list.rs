use std::cell::RefCell;
use std::rc::Rc;

type LinkNode<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    value: T,
    prev: LinkNode<T>,
    next: LinkNode<T>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            value,
            prev: None,
            next: None,
        }))
    }
}

struct DoubleLinkedList<T> {
    head: LinkNode<T>,
    tail: LinkNode<T>,
    length: usize,
}

impl<T> DoubleLinkedList<T>
where
    T: PartialEq,
{
    pub fn new() -> Self {
        DoubleLinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn add(&mut self, value: T) {
        let head = Node::new(value);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(head.clone());
                head.borrow_mut().next = Some(old_head.clone());
            }
            None => {
                self.tail = Some(head.clone());
            }
        }
        self.head = Some(head.clone());
        self.length += 1;
    }

    pub fn append(&mut self, value: T) {
        let tail = Node::new(value);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(tail.clone());
                tail.borrow_mut().prev = Some(old_tail);
            }
            None => {
                self.head = Some(tail.clone());
            }
        }
        self.tail = Some(tail);
        self.length += 1;
    }

    pub fn insert(&mut self, value: T, index: usize) {
        if index == 0 {
            self.add(value)
        } else if index >= self.length {
            self.append(value)
        } else {
            let node = Node::new(value);
            let old_node = self
                .traverse(|_| true)
                .nth(index)
                .expect("index out of range");

            let old_node_prev = old_node.borrow_mut().prev.take();
            node.borrow_mut().next = Some(old_node.clone());
            node.borrow_mut().prev = old_node_prev.clone();
            old_node.borrow_mut().prev = Some(node.clone());

            if let Some(old_node_prev) = old_node_prev {
                old_node_prev.borrow_mut().next = Some(node.clone());
            }

            self.length += 1;
        }
    }

    pub fn search(&self, value: T) -> bool {
        self.traverse(|node| node.borrow().value == value)
            .next()
            .is_some()
    }

    pub fn remove(&mut self, value: T) -> Option<T> {
        if let Some(node) = self.traverse(|node| node.borrow().value == value).next() {
            let prev = node.borrow().prev.clone();
            let next = node.borrow().next.clone();

            match prev.clone() {
                Some(prev) => prev.borrow_mut().next = next.clone(),
                None => self.head = next.clone(),
            }

            match next {
                Some(next) => next.borrow_mut().prev = prev.clone(),
                None => self.tail = prev.clone(),
            }
            self.length -= 1;

            return Some(Rc::try_unwrap(node).ok().unwrap().into_inner().value);
        }
        None
    }

    pub fn traverse<F>(&self, f: F) -> impl Iterator<Item = Rc<RefCell<Node<T>>>>
    where
        F: Fn(&Rc<RefCell<Node<T>>>) -> bool,
    {
        NodeIter {
            next: self.head.clone(),
            f,
        }
    }
}

struct NodeIter<T, F>
where
    T: PartialEq,
    F: Fn(&Rc<RefCell<Node<T>>>) -> bool,
{
    next: Option<Rc<RefCell<Node<T>>>>,
    f: F,
}

impl<T, F> Iterator for NodeIter<T, F>
where
    T: PartialEq,
    F: Fn(&Rc<RefCell<Node<T>>>) -> bool,
{
    type Item = Rc<RefCell<Node<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.next.clone() {
            self.next = node.borrow().next.clone();
            if (self.f)(&node) {
                return Some(node);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::double_linked_list::DoubleLinkedList;

    #[test]
    fn test_double_linked_list() {
        let mut double_linked_list = DoubleLinkedList::new();

        println!("Add nodes 1,2,4:");
        double_linked_list.append(1);
        double_linked_list.append(2);
        double_linked_list.append(4);
        double_linked_list
            .traverse(|_| true)
            .for_each(|node| print!("{:?}-", node.borrow().value));
        println!("\n");

        println!("Insert 3 in index 2, and insert 5 in index 4:");
        double_linked_list.insert(3, 2);
        double_linked_list.insert(5, 4);
        double_linked_list
            .traverse(|_| true)
            .for_each(|node| print!("{:?}-", node.borrow().value));
        println!("\n");

        println!("Add 0 as head:");
        double_linked_list.add(0);
        double_linked_list
            .traverse(|_| true)
            .for_each(|node| print!("{:?}-", node.borrow().value));
        println!("\n");

        println!("Remove node 4:");
        double_linked_list.remove(4);
        double_linked_list
            .traverse(|_| true)
            .for_each(|node| print!("{:?}-", node.borrow().value));
        println!("\n");

        println!("Check that node 3 exists:");
        let has_node_3 = double_linked_list.search(3);
        println!("Has node 3->{:?}", has_node_3);
        println!();

        println!("The double-linked list length:");
        println!("{:?}", double_linked_list.length);
    }
}
