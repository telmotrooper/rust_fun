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
    root: Option<Box<Node<T>>>
}

impl<T> BinarySearchTree<T>
where T: PartialOrd
{
    fn new() -> BinarySearchTree<T> {
        BinarySearchTree {
            root: None
        }
    }

    fn insert(self: &mut Self, value: T) {
        let new_node: Node<T> = Node::new(value);

        let mut current_node: &Option<Box<Node<T>>> = &self.root;

        if current_node.is_none() {
            self.root = Some(Box::new(new_node))
        }

        // if new_node.value < current_node.value {
        //     if let Some(left_child) = &current_node.left_child {
        //         todo!()
        //         // current_node = left_child;
        //     } else {
        //         todo!()
        //     }
        // } else {
        //     if let Some(right_child) = &current_node.right_child {
        //         todo!()
        //     } else {
        //         todo!()
        //     }
        // }
    }
}

fn main() {
    let mut bst: BinarySearchTree<i64> = BinarySearchTree::new();

    bst.insert(256);

    println!("{:?}", bst.root.unwrap().value);
}
