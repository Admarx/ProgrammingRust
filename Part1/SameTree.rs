// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        
        match(p, q)
        {
            (None, None) => true,
            (Some(lewy_node),Some(prawy_node)) => {
                let lewy = lewy_node.borrow();
                let prawy = prawy_node.borrow();

                let testLewy = Solution::is_same_tree(lewy.left.clone(), prawy.left.clone());
                let mut testWart = false;
                if lewy.val == prawy.val
                {
                    testWart = true;
                }
                let testPrawy = Solution::is_same_tree(lewy.right.clone(), prawy.right.clone());
                
                if testLewy && testWart && testPrawy
                {
                    return true;
                }
                else
                {
                    return false;
                }

            },
            _ => false
        }
    }
}
