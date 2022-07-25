#[derive(Debug)]
pub struct Linklist<T> {
    top: Option<Box<Node<T>>>,
}

#[derive(Debug, Clone)]
pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Node { data, next: None }
    }

    pub fn get_data(&self) -> &T {
        &self.data
    }
    pub fn get_next(&self) -> &Option<Box<Node<T>>> {
        &self.next
    }
}

impl<T> Linklist<T> {
    pub fn new(data: T) -> Self {
        let node = Node::new(data);
        Linklist {
            top:Some(Box::new(node)),
        }
    }

    pub fn push(&mut self, data: T) {
        let next = self.top.take().unwrap();
        let node = Node {
            data,
            next: Some(next),
        };
        self.top = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        if let None = self.top {
            return None;
        }

        let mut node = self.top.take().unwrap();
        self.top = node.next.take();

        Some(node.data)
    }

    pub fn get_top(&self) -> &Option<Box<Node<T>>> {
        &self.top
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fun() {
        let mut linklist = Linklist::new(0);
        linklist.push(1);
        linklist.push(2);
        linklist.push(3);
        linklist.push(4);

        assert_eq!(linklist.pop(), Some(4));
        assert_eq!(linklist.pop(), Some(3));
        assert_eq!(linklist.pop(), Some(2));
        assert_eq!(linklist.pop(), Some(1));
        assert_eq!(linklist.pop(), Some(0));
        assert_eq!(linklist.pop(), None);
    }
}