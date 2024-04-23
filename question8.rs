pub struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

fn max_depth<T>(node: &Option<Box<TreeNode<T>>>) -> i32 {
    match node {
        Some(node) => 1 + std::cmp::max(max_depth(&node.left), max_depth(&node.right)),
        None => 0,
    }
}

fn main() {
    let leaf1 = TreeNode {
        value: 4,
        left: None,
        right: None,
    };
    let leaf2 = TreeNode {
        value: 5,
        left: None,
        right: None,
    };
    let node = TreeNode {
        value: 3,
        left: Some(Box::new(leaf1)),
        right: Some(Box::new(leaf2)),
    };
    let root = TreeNode {
        value: 1,
        left: Some(Box::new(node)),
        right: None,
    };

    let depth = max_depth(&Some(Box::new(root)));
    println!("The maximum depth of the tree is: {}", depth);
}
