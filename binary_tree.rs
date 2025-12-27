#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn New(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn search(root: &Option<Box<Node>>, value: i32) -> bool {
        match root {
            None => false,
            Some(node) => {
                if value == node.value {
                    true
                } else if value < node.value {
                    search(&node.left, value)
                } else {
                    search(&node.right, value)
                }
            }
        }
    }

    fn insert(root: &mut Option<Box<Node>>, value: i32) {
        match root {
            None => {
                *root = Some(Box::new(Node::new(value)));
            }
            Some(node) => {
                if value < node.value {
                    insert(&mut node.left, value);
                } else if value > node.value {
                    insert(&mut node.right, value);
                }
                // valores iguais são ignorados (opção de projeto)
            }
        }
    }

    fn find_min(node: &Box<Node>) -> i32 {
        match &node.left {
            None => node.value,
            Some(left) => find_min(left),
        }
    }


    fn remove(root: &mut Option<Box<Node>>, value: i32) {
        if let Some(node) = root {
            if value < node.value {
                remove(&mut node.left, value);
            } else if value > node.value {
                remove(&mut node.right, value);
            } else {
                if node.left.is_none() {
                    *root = node.right.take();
                } else if node.right.is_none() {
                    *root = node.left.take();
                } else {
                    let min_value = find_min(node.right.as_ref().unwrap());
                    node.value = min_value;
                    remove(&mut node.right, min_value)
                }
            }
        }
    }
}