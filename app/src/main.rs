extern crate binary_tree;

use binary_tree::binary_tree::interface::{IBinaryTree, Callback};
use binary_tree::binary_tree::service::BinaryTree;

fn main() {
    let callback: Callback<i32> = |x| {
        if let Some(item) = x {
            println!("Item: {}", item);
        } else {
            println!("Received None item.");
        }
        None
    };
    let mut tree = BinaryTree::<i32>::new();
    tree.set_callback(callback);
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(2);
    tree.insert(4);
    tree.insert(6);
    tree.insert(8);
    println!("In Order");
    tree.in_order();
    tree.remove(5);
    println!("In Order");
    tree.in_order();

}
