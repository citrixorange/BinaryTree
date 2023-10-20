use std::fmt::Display;
use std::cmp::{PartialEq, PartialOrd};

pub type Callback<T> = fn(Option<T>) -> Option<T>;

pub trait IBinaryTree<T> 
where
    T: Display + Copy + PartialEq + PartialOrd
{
    fn set_callback(&mut self, callback: Callback<T>);
    fn search(&mut self, item: T) -> Option<T>;
    fn insert(&mut self, item: T);
    fn remove(&mut self, item: T);
    fn pre_order(&self);
    fn in_order(&self);
    fn post_order(&self);
}