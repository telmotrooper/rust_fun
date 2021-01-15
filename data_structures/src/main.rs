use std::rc::Rc;
use std::cmp::PartialOrd;

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

impl<T> BinarySearchTree<T>
where T: PartialOrd
{
    fn new(value: T) -> BinarySearchTree<T> {
        BinarySearchTree {
            root: Node::new(value)
        }
    }

    fn insert(self: &Self, value: T) {
        let new_node: Node<T> = Node::new(value);

        let current_node= &self.root;

        if new_node.value > current_node.value {
            if let Some(left_child) = &current_node.left_child {
                // TODO
            }
        }
    }
}

fn main() {
    let bst: BinarySearchTree<i64> = BinarySearchTree::new(256);

    println!("{:?}", bst.root.value);
}
