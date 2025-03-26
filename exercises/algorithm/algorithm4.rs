/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
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
    T: Ord +Copy,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }
    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        if let Some(mut root) = self.root.as_mut() {
            Self::insert_into_node(root, value);
        } else {
            self.root = Some(Box::new(TreeNode::new(value)));
        }
    }

    // 醒槐
    // 递归实现二分查找树，太美丽了
    // 这里的Ordering实现了比较
    // 而且有个语法技巧：if let 我原来想要匹配不到返回none，使用if let 美观很多

    // Helper function to insert into a node
    fn insert_into_node(node: &mut Box<TreeNode<T>>, value: T) {
        match value.cmp(&node.value) {
            Ordering::Less => {
                if let Some(left) = &mut node.left {
                    Self::insert_into_node(left, value);
                } else {
                    node.left = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Greater => {
                if let Some(right) = &mut node.right {
                    Self::insert_into_node(right, value);
                } else {
                    node.right = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Equal => {} // Ignore duplicate values
        }
    }

    // 醒槐
    // 啊???这就实现了
    // 这个as_ref和map_or的组合使用
    // Search for a value in the BST
    fn search(&self, value: T) -> bool
    where
        T: PartialEq,
    {
        let value = &value;
        self.root.as_ref().map_or(false, |node| Self::search_in_node(node, value))
    }
    // 递归
    // Helper function to search in a node
    fn search_in_node(node: &Box<TreeNode<T>>, value: &T) -> bool {
        match value.cmp(&node.value) {
            Ordering::Equal => true,
            Ordering::Less => node.left.as_ref().map_or(false, |left| Self::search_in_node(left, value)),
            Ordering::Greater => node.right.as_ref().map_or(false, |right| Self::search_in_node(right, value)),
        }
    }
}
//     // Insert a value into the BST
//     fn insert(&mut self, value: T) {
//         if self.root.is_none(){
//             self.root = Some(Box::new(TreeNode::new(value)));
//         }else{
//             let mut tree_node = self.root.as_mut().unwrap();
//             loop{
//                 if value < tree_node.value {
//                     if tree_node.left.is_none() {
//                         tree_node.left = Some(Box::new(TreeNode::new(value)));
//                         break;
//                     }
//                     tree_node = tree_node.left.as_mut().unwrap();
//                 }else if value > tree_node.value{
//                     if tree_node.right.is_none() {
//                         tree_node.right = Some(Box::new(TreeNode::new(value)));
//                         break;
//                     }
//                     tree_node = tree_node.right.as_mut().unwrap();
//                 }else{
//                     break;
//                 }
//         }}

//     }

//     // Search for a value in the BST
//     fn search(&self, value: T) -> bool {
//         //TODO
//         if self.root.is_none(){
//             false
//         }else{
//             let mut node = self.root.as_ref().unwrap();
//             loop{
//                 if value == node.value{
//                     return true;
//                 }else if value<= node.value{
//                     match &node.left {
//                         Some(left) => node = left, // 直接使用引用
//                         None => return false,
//                     }
//                 }else{
//                     match &node.right {
//                         Some(right) => node = right, // 直接使用引用
//                         None => return false,
//                     }
//                 }
//             }
//         }
        

//     }
// }

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


