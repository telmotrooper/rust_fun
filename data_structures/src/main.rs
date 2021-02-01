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

        let current_node;

        match root {
            None => {
                self.root = Some(Box::new(new_node));
                return
            },
            Some(node) => {
                current_node = node;
            }
        }

        if new_node.value < current_node.value { // Less than
            println!("Less than")

        } else { // Greater than
            println!("Greater than")
        }

        // if new_node.value < current_node.as_ref().unwrap().value { // Less than
        //     if current_node.as_ref().unwrap().left_child.is_some() {
        //         current_node = &current_node.as_ref().unwrap().left_child;
        //     } else {
        //         todo!()
        //     }
        // } else { // Greater than
        //     if current_node.as_ref().unwrap().right_child.is_some() {
        //         current_node = &current_node.as_ref().unwrap().right_child;
        //     } else {
        //         todo!()
        //     }
        // }
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
