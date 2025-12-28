use std::collections::VecDeque;

// Cópia da estrutura Node para os testes de integração
// Em um projeto real, você exportaria isso de um módulo em src/
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
            }
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

    fn find_min(node: &Box<Node>) -> i32 {
        match &node.left {
            None => node.value,
            Some(left) => Node::find_min(left),
        }
    }

    fn level_order_traversal(root: &Option<Box<Node>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut queue = VecDeque::new();

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

// Helper para criar uma árvore de teste
fn create_test_tree() -> Option<Box<Node>> {
    let mut tree: Option<Box<Node>> = None;
    Node::insert(&mut tree, 10);
    Node::insert(&mut tree, 5);
    Node::insert(&mut tree, 15);
    Node::insert(&mut tree, 3);
    Node::insert(&mut tree, 7);
    Node::insert(&mut tree, 12);
    Node::insert(&mut tree, 20);
    //        10
    //       /  \
    //      5    15
    //     / \   / \
    //    3   7 12  20
    tree
}

// ============ Testes para search ============

#[test]
fn test_search_encontra_raiz() {
    let tree = create_test_tree();
    assert!(Node::search(&tree, 10));
}

#[test]
fn test_search_encontra_folha_esquerda() {
    let tree = create_test_tree();
    assert!(Node::search(&tree, 3));
}

#[test]
fn test_search_encontra_folha_direita() {
    let tree = create_test_tree();
    assert!(Node::search(&tree, 20));
}

#[test]
fn test_search_nao_encontra_valor() {
    let tree = create_test_tree();
    assert!(!Node::search(&tree, 100));
    assert!(!Node::search(&tree, 0));
    assert!(!Node::search(&tree, 8));
}

#[test]
fn test_search_arvore_vazia() {
    let tree: Option<Box<Node>> = None;
    assert!(!Node::search(&tree, 10));
}

// ============ Testes para find_min ============

#[test]
fn test_find_min_arvore_completa() {
    let tree = create_test_tree();
    let min = Node::find_min(tree.as_ref().unwrap());
    assert_eq!(min, 3);
}

#[test]
fn test_find_min_no_unico() {
    let mut tree: Option<Box<Node>> = None;
    Node::insert(&mut tree, 42);
    let min = Node::find_min(tree.as_ref().unwrap());
    assert_eq!(min, 42);
}

#[test]
fn test_find_min_apenas_filhos_direita() {
    let mut tree: Option<Box<Node>> = None;
    Node::insert(&mut tree, 10);
    Node::insert(&mut tree, 20);
    Node::insert(&mut tree, 30);
    let min = Node::find_min(tree.as_ref().unwrap());
    assert_eq!(min, 10); // raiz é o mínimo
}

// ============ Testes para level_order_traversal ============

#[test]
fn test_level_order_arvore_completa() {
    let tree = create_test_tree();
    let result = Node::level_order_traversal(&tree);
    assert_eq!(result, vec![10, 5, 15, 3, 7, 12, 20]);
}

#[test]
fn test_level_order_arvore_vazia() {
    let tree: Option<Box<Node>> = None;
    let result = Node::level_order_traversal(&tree);
    assert!(result.is_empty());
}

#[test]
fn test_level_order_no_unico() {
    let mut tree: Option<Box<Node>> = None;
    Node::insert(&mut tree, 42);
    let result = Node::level_order_traversal(&tree);
    assert_eq!(result, vec![42]);
}

#[test]
fn test_level_order_apenas_esquerda() {
    let mut tree: Option<Box<Node>> = None;
    Node::insert(&mut tree, 30);
    Node::insert(&mut tree, 20);
    Node::insert(&mut tree, 10);
    let result = Node::level_order_traversal(&tree);
    assert_eq!(result, vec![30, 20, 10]);
}

#[test]
fn test_level_order_apenas_direita() {
    let mut tree: Option<Box<Node>> = None;
    Node::insert(&mut tree, 10);
    Node::insert(&mut tree, 20);
    Node::insert(&mut tree, 30);
    let result = Node::level_order_traversal(&tree);
    assert_eq!(result, vec![10, 20, 30]);
}
