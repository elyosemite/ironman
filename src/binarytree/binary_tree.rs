#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
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
                    Node::search(&node.left, value)
                } else {
                    Node::search(&node.right, value)
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
                    Node::insert(&mut node.left, value);
                } else if value > node.value {
                    Node::insert(&mut node.right, value);
                }
                // valores iguais são ignorados (opção de projeto)
            }
        }
    }

    fn find_min(node: &Box<Node>) -> i32 {
        match &node.left {
            None => node.value,
            Some(left) => Node::find_min(left),
        }
    }


    fn remove(root: &mut Option<Box<Node>>, value: i32) {
        if let Some(node) = root {
            if value < node.value {
                Node::remove(&mut node.left, value);
            } else if value > node.value {
                Node::remove(&mut node.right, value);
            } else {
                if node.left.is_none() {
                    *root = node.right.take();
                } else if node.right.is_none() {
                    *root = node.left.take();
                } else {
                    let min_value = Node::find_min(node.right.as_ref().unwrap());
                    node.value = min_value;
                    Node::remove(&mut node.right, min_value)
                }
            }
        }
    }

    fn level_order_traversal(root: &Option<Box<Node>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut queue = std::collections::VecDeque::new();

        if let Some(node) = root {
            queue.push_back(node);
        }

        while let Some(node) = queue.pop_front() {
            result.push(node.value);

            if let Some(left) = &node.left {
                queue.push_back(left);
            }
            if let Some(right) = &node.right {
                queue.push_back(right);
            }
        }

        result
    }
}