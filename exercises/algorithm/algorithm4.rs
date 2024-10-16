/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

//I AM NOT DONE
use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug, PartialEq)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        let new_node = Box::new(TreeNode::<T>::new(value));
        if let Some(ref mut node) = self.root {
            Self::insert_into(node, new_node);
        } else {
            self.root = Some(new_node);
        }
    }

    fn insert_into(node: &mut Box<TreeNode<T>>, new_node: Box<TreeNode<T>>) {
        match new_node.value.cmp(&node.value) {
            Ordering::Less => {
                if let Some(ref mut left) = node.left {
                    Self::insert_into(left, new_node);
                } else {
                    node.left = Some(new_node);
                }
            }
            Ordering::Greater => {
                if let Some(ref mut right) = node.right {
                    Self::insert_into(right, new_node);
                } else {
                    node.right = Some(new_node);
                }
            }
            Ordering::Equal => {
                // 如果值相等，可以选择不插入，或者允许重复元素
                // 这里选择不插入
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        Self::search_in(&self.root, &value)
    }

    fn search_in(node: &Option<Box<TreeNode<T>>>, value: &T) -> bool {
        match node {
            Some(ref node) => match value.cmp(&node.value) {
                Ordering::Less => Self::search_in(&node.left, value),
                Ordering::Greater => Self::search_in(&node.right, value),
                Ordering::Equal => true,
            },
            None => false,
        }
    }
}

// impl<T> TreeNode<T>
// where
//     T: Ord,
// {
//     // Insert a node into the tree
//     fn insert(&mut self, value: T) {
//         //TODO
//     }
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


