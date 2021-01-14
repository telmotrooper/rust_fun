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

impl<T> BinarySearchTree<T> {
    fn new(value: T) -> BinarySearchTree<T> {
        BinarySearchTree {
            root: Node::new(value)
        }
    }
}

fn main() {
    let bst: BinarySearchTree<i64> = BinarySearchTree::new(256);

    println!("{:?}", bst.root.value);
}
