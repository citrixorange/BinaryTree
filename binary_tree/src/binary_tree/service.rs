use std::fmt::Display;
use std::cmp::{PartialEq, PartialOrd};

use crate::binary_tree::interface::{Callback, IBinaryTree};
use crate::binary_tree::implementation::binary_tree::ConcreteBinaryTree;

pub struct BinaryTree<T>
where
    T: Display + Copy + PartialEq + PartialOrd
{
    tree: Box<dyn IBinaryTree<T>>
}

impl<T> BinaryTree<T> 
where
    T: Display + Copy + PartialEq + PartialOrd + 'static 
{
    pub fn new() -> Self {

        let tree = ConcreteBinaryTree::<T>::new();

        Self {
            tree: Box::new(tree)
        }
    }
}

impl<T> IBinaryTree<T> for BinaryTree<T> 
where
    T: Display + Copy + PartialEq + PartialOrd
{
    fn set_callback(&mut self, callback: Callback<T>) {
        self.tree.set_callback(callback);
    }

    fn search(&mut self, item: T) -> Option<T> {
        return self.tree.search(item);
    }

    fn insert(&mut self, item: T) {
        self.tree.insert(item);
    }

    fn remove(&mut self, item: T) {
        self.tree.remove(item);
    }

    fn pre_order(&self) {
        self.tree.pre_order();
    }

    fn in_order(&self) {
        self.tree.in_order();
    }

    fn post_order(&self) {
        self.tree.post_order();
    }

}