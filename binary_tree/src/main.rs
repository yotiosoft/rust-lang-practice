/// ジェネリック列挙型を用いたバイナリツリー
/// pp.214- （第2版）

// Tの順序付きコレクション
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

// BinaryTreeの一部
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

use self::BinaryTree::*;

fn main() {
    let jupiter_tree = NonEmpty(Box::new(TreeNode {
        element: "Jupiter",
        left: Empty,
        right: Empty,
    }));

    let mars_tree = NonEmpty(Box::new(TreeNode {
        element: "Mars",
        left: jupiter_tree,
        right: Empty,
    }));

    let mut tree = BinaryTree::Empty;
    let planets = ["Earth"];
    for planet in planets {
        tree.add(planet);
    }
}
