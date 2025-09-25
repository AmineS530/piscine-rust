#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        let new = Node {
            value,
            next: self.head.take().map(Box::new),
        };
        self.head = Some(new);
    }

    pub fn pop(&mut self) {
        if let Some(node) = self.head.take() {
            self.head = node.next.map(|boxed_node| *boxed_node);
        }
    }

    pub fn len(&self) -> usize {
        let mut counter = 0;
        let mut cur: Option<&Node<T>> = self.head.as_ref();
        while let Some(node) = cur {
            counter += 1;
            cur = node.next.as_deref();
        }
        counter
    }
}
