use std::cmp::PartialOrd;

struct Node<T> {
    value: T,
    left_child: Option<Box<Node<T>>>,
    right_child: Option<Box<Node<T>>>
    // Box is used because Node<T> is a recursive type.
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
    root: Box<Node<T>>
}

impl<T> BinarySearchTree<T>
where T: PartialOrd
{
    fn new(value: T) -> BinarySearchTree<T> {
        BinarySearchTree {
            root: Box::new(Node::new(value))
        }
    }

    fn insert(self: &Self, value: T) {
        let new_node: Node<T> = Node::new(value);

        let mut current_node= Box::new(&self.root);

        if new_node.value < current_node.value {
            if let Some(left_child) = &current_node.left_child {
                todo!()
                // current_node = left_child;
            } else {
                todo!()
            }
        } else {
            if let Some(right_child) = &current_node.right_child {
                todo!()
            } else {
                todo!()
            }
        }
    }
}

fn main() {
    let bst: BinarySearchTree<i64> = BinarySearchTree::new(256);

    println!("{:?}", bst.root.value);
}
