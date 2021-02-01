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

    fn insert(&mut self, value: T) {
        let new_node: Node<T> = Node::new(value);

        let mut root: &Option<Box<Node<T>>> = &self.root;

        let mut current_node;

        match root {
            Some(node) => {
                current_node = node;
            },
            None => {
                self.root = Some(Box::new(new_node));
                return;
            }
        }

        if new_node.value < current_node.value { // Less than
            match &current_node.left_child {
                Some(left_child) => {
                    current_node = &left_child;
                },
                None => {
                    // current_node.left_child = Some(Box::new(new_node));
                    println!("#1");
                }
            }

        } else { // Greater than
            match &current_node.right_child {
                Some(right_child) => {
                    current_node = &right_child;
                },
                None => {
                    // current_node.right_child = Some(Box::new(new_node));
                    println!("#2");
                }
            }
        }
    }
}

fn main() {
    let mut bst: BinarySearchTree<i64> = BinarySearchTree::new();

    bst.insert(256);
    bst.insert(512);

    match bst.root {
        Some(node) => println!("Root: {:?}", node.value),
        None => println!("Root: None")
    };
}
