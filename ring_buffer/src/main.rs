use std::rc::Rc;
use std::ops::Deref;
use std::clone::Clone;

fn main() {
}

#[derive(Debug)]
struct Node<T> {
    value: Option<T>,
    next: Option<Rc<Node<T>>>,
}

impl <T> Node<T> {
    fn initial(v: Option<T>) -> Node<T> {
        Node {
            value: v,
            next: None
        }
    }

    fn is_empty(&self) -> bool {
        self.value.is_none()
    }

    fn prepend_empty(self) -> Node<T> {
        let next = Some(Rc::new(self));
        Node {
            value: None,
            next,
        }
    }

    fn assign_value(&mut self, v: T) -> Option<T>{
        self.value.replace(v)
    }

    fn prepend(self, v: T) -> Node<T> {
        let mut ret = self.prepend_empty();
        ret.assign_value(v);
        ret
    }

    fn strip(mut self) -> (Option<T>, Option<Node<T>>) {
        let value = self.value.take();
        let next = self.next.take().map(|n| Rc::try_unwrap(n).ok().unwrap());

        (value, next)
    }

    fn value_ref(&self) -> Option<&T> {
        self.value.as_ref()
    }

    fn next_ref(&self) -> Option<&Node<T>> {
        self.next.as_ref().map(|n| n.deref())
    }
}

#[derive(Debug)]
struct List<T> {
    size: usize,
    head: Option<Node<T>>,
}

impl<T> List<T> {
    fn new() -> List<T> {
        List {
            size: 0,
            head: None,
        }
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn push(&mut self, v: T) {
        if self.is_empty() {
            assert_eq!(self.size, 0);
            self.head = Some(Node::initial(Some(v)));
            self.size = 1;
        } else {
            let mut head = self.head.take().unwrap();
            head = head.prepend(v);
            self.head.replace(head);
            self.size += 1;
        }
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let head = self.head.take().unwrap();
            let (val, he) = head.strip();

            self.head = he;
            self.size -= 1;

            val
        }
    }

    fn get(&self, i: usize) -> Option<&T> {
        assert!(i < self.size);

        if self.is_empty() {
            None
        } else {
            let mut node: Option<&Node<T>> = self.head.as_ref();
            for _i in 0..i {
                node = node.and_then(|n| n.next_ref());
            }
            node.and_then(|n| n.value_ref())
        }
    }
}

#[test]
fn test_ll() {
    let mut ll = List::<i32>::new();
    assert_eq!(ll.size, 0);
    assert!(ll.is_empty());

    ll.push(1);
    assert_eq!(ll.size, 1);
    ll.push(2);
    assert_eq!(ll.size, 2);

    {
        assert_eq!(format!("{:?}", ll), "List { size: 2, head: Some(Node { value: Some(2), next: Some(Node { value: Some(1), next: None }) }) }");
        let v = ll.get(0);
        assert_eq!(v, Some(&2));
        let v = ll.get(1);
        assert_eq!(v, Some(&1));
    }

    let poped = ll.pop();
    assert_eq!(poped, Some(2));
    assert_eq!(ll.size, 1);
    {
        let head = ll.get(0);
        assert_eq!(head, Some(&1));
    }

    let poped = ll.pop();
    assert_eq!(poped, Some(1));
    assert_eq!(ll.size, 0);
}

#[derive(Debug)]
struct RingBuffer<T> {
    size: usize,
    capacity: usize,
    head: Rc<Node<T>>,
}

impl<T> RingBuffer<T> {
    fn new(capacity: usize) -> RingBuffer<T> {

        let head = Node::initial(None);
        let mut head = Rc::new(head);
        let head_self = head.clone();
        // 実行できない: head_selfを作っているのでmutがNoneを返す
        //Rc::get_mut(&mut head).unwrap().next.replace(head_self);

        let rb = RingBuffer::<T> {
            size: 0,
            capacity: capacity,
            head: head,
        };

        rb
    }
}

#[test]
fn test_rb() {
    let rc = RingBuffer::<i32>::new(1);
    assert_eq!(rc.size, 0);
}
