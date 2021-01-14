use std::rc::Rc;

struct Node<T> {
    value: T,
    left_child: Option<Rc<Node<T>>>,
    right_child: Option<Rc<Node<T>>>
}

impl Node<i64> {
    fn new(value: i64) -> Node<i64> {
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
