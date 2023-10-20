use std::fmt::Display;
use std::cmp::{PartialEq, PartialOrd};

use crate::binary_tree::interface::{Callback, IBinaryTree};

struct Node<T> 
where
    T: Display + Copy + PartialEq + PartialOrd
{
    item: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>
}

impl<T> Node<T> 
where
    T: Display + Copy + PartialEq + PartialOrd
{
    pub fn new(item: T) -> Self {
        Self {
            item: item,
            left: None,
            right: None
        }
    }

    fn inner_search(&mut self, item: T) -> Option<&mut Node<T>> {
        
        if self.item == item {
            return Some(self);
        }

        if item < self.item {
            if let Some(left) = &mut self.left {
                return left.inner_search(item);
            } else {
                return None;
            }
        } else {
            if let Some(right) = &mut self.right {
                return right.inner_search(item);
            } else {
                return None;
            }
        }
    }

    fn inner_insert(&mut self, item: T) {
        
        if self.item == item {
            return;
        }

        if item < self.item {
            if let Some(left) = &mut self.left {
                return left.inner_insert(item);
            } else {
                self.left = Some(Box::new(Node::<T>::new(item)));
                return;
            }
        } else {
            if let Some(right) = &mut self.right {
                return right.inner_insert(item);
            } else {
                self.right = Some(Box::new(Node::<T>::new(item)));
                return;
            }
        }
    }

    fn inner_remove(&mut self, item: T) -> Option<Box<Node<T>>> {

        if &item < &self.item {
            if let Some(ref mut left) = self.left {
                self.left = left.inner_remove(item);
            }
        } else if &item > &self.item {
            if let Some(ref mut right) = self.right {
                self.right = right.inner_remove(item);
            }
        } else {
            if self.left.is_none() {
                return self.right.take();
            } else if self.right.is_none() {
                return self.left.take();
            } else {
                let mut right_min = self.right.take();
                let mut current = right_min.as_mut().unwrap();
                while let Some(ref mut left) = current.left {
                    current = left;
                }
                self.item = current.item;
                right_min = right_min.unwrap().inner_remove(self.item);
                self.right = right_min;
            }
        }

        Some(Box::new(Node {
            item: self.item,
            left: self.left.take(),
            right: self.right.take(),
        }))

    }

    fn inner_pre_order(&self, callback: &Option<Callback<T>>) {

        if let Some(call) = &callback {
            call(Some(self.item));
        }

        if let Some(left) = &self.left {
            left.inner_pre_order(&callback);
        }

        if let Some(right) = &self.right {
            right.inner_pre_order(&callback);
        }
    }

    fn inner_in_order(&self, callback: &Option<Callback<T>>) {
        if let Some(left) = &self.left {
            left.inner_in_order(&callback);
        }

        if let Some(call) = &callback {
            call(Some(self.item));
        }

        if let Some(right) = &self.right {
            right.inner_in_order(&callback);
        }
    }

    fn inner_post_order(&self, callback: &Option<Callback<T>>) {
        if let Some(left) = &self.left {
            left.inner_post_order(&callback);
        }

        if let Some(right) = &self.right {
            right.inner_post_order(&callback);
        }

        if let Some(call) = &callback {
            call(Some(self.item));
        }
    }
}

pub struct ConcreteBinaryTree<T> 
where
    T: Display + Copy + PartialEq + PartialOrd
{
    callback: Option<Callback<T>>,
    root: Option<Box<Node<T>>>
}

impl<T> ConcreteBinaryTree<T>
where
    T: Display + Copy + PartialEq + PartialOrd
{
    pub fn new() -> Self {
        Self {
            callback: None,
            root: None
        }
    }
}

impl<T> IBinaryTree<T> for ConcreteBinaryTree<T>
where
    T: Display + Copy + PartialEq + PartialOrd
{

    fn set_callback(&mut self, callback: Callback<T>) {
        self.callback = Some(callback);
    }

    fn search(&mut self, item: T) -> Option<T> {
        if let Some(root) = &mut self.root {
            if let Some(node) = root.inner_search(item) {
                return Some(node.item);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    fn insert(&mut self, item: T) {
        if let Some(root) = &mut self.root {
            return root.inner_insert(item);
        } else {
            self.root = Some(Box::new(Node::<T>::new(item)));
            return;
        }
    }

    fn remove(&mut self, item: T){
        if let Some(ref mut root) = &mut self.root {
            self.root = root.inner_remove(item);
        }
    }

    fn pre_order(&self) {
        if let Some(root) = &self.root {
            root.inner_pre_order(&self.callback);
        }
    }

    fn in_order(&self) {
        if let Some(root) = &self.root {
            root.inner_in_order(&self.callback);
        }
    }

    fn post_order(&self) {
        if let Some(root) = &self.root {
            root.inner_post_order(&self.callback);
        }
    }
}