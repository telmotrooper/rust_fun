use std::rc::Rc;

struct Node<T> {
    value: T,
    left_child: Option<Rc<Node<T>>>,
    right_child: Option<Rc<Node<T>>>
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            value,
            left_child: None,
            right_child: None
        }
    }
}

struct BinarySearchTree<T> {
    root: Node<T>
}

fn main() {
    let bst: BinarySearchTree<i64> = BinarySearchTree {
        root: Node::new(256)
    };

    println!("Hello, world!");
}
