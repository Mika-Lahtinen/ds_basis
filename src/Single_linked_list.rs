use std::cell::RefCell;
use std::rc::Rc;

type CoreNode<T> = Rc<RefCell<Node<T>>>;
type LinkNode<T> = Option<CoreNode<T>>;

#[derive(Debug)]
struct LinkedList<T> {
    length: i32,
    head: LinkNode<T>,
    tail: LinkNode<T>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            length: 0,
            head: None,
            tail: None,
        }
    }

    fn append(&mut self, data: T) {
        let node = Node::new(data);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(node.clone()),
            None => self.head = Some(node.clone()),
        };
        self.length += 1;
        self.tail = Some(node);
    }

    fn prepend(&mut self, data: T) {
        let node = Node::new(data);
        node.borrow_mut().next = self.head.take();
        match self.tail.take() {
            Some(x) => self.tail = Some(x),
            None => self.tail = Some(node.clone()),
        };
        self.length += 1;
        self.head = Some(node);
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Something is terribly wrong")
                .into_inner()
                .data
        })
    }
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: LinkNode<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> CoreNode<T> {
        Rc::new(RefCell::new(Node {
            data: value,
            next: None,
        }))
    }
}

fn main() {
    let mut t: LinkedList<i32> = LinkedList::new();
    t.append(2);
    t.append(3);
    println!("{:?}", t);
    t.prepend(1);
    println!("{:?}", t);
}