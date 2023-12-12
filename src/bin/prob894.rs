use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>
}

type TreeNodePtr = Option<Rc<RefCell<TreeNode>>>;

impl TreeNode {
    #[inline]
    pub fn new() -> Self {
        TreeNode {
            val: 0,
            left: None,
            right: None
        }
    }

    pub fn new_with(left: TreeNodePtr, right: TreeNodePtr) -> Self {
        TreeNode {
            val: 0,
            left,
            right
        }
    }
}

pub struct Solution {}

impl Solution {
    pub fn all_possible_fbt(n: i32) -> Vec<TreeNodePtr> {
        // n 为偶数时，无法建立符合条件的树
        if n % 2 == 0 {
            return vec![];
        }

        // dp[n] == all_possible_fbt(n)
        let mut dp = vec![vec![]; (n + 1) as usize];

        // 设置 dp[1] 的情况
        dp[1].push(Some(Rc::new(RefCell::new(TreeNode::new()))));

        // ni 从 3 开始，遍历到 n，
        // all_possible_fbt(ni) 的可能性，由 左子树 的全部可能性，组合 右子树 的全部可能性构成，
        // 也即 all_possible_fbt(ni_left) * all_possible_fbt(ni_right)，
        // 其中，ni_left + ni_right + 1 == n，且 ni_left >= 1 && ni_right >= 1
        for ni in (3..=n).step_by(2) {
            let mut dp_ni = vec![];

            // 遍历所有左右子树的节点数量情况，从而找出所有可能
            for ni_left in (1..ni).step_by(2) {
                let ni_right = ni - 1 - ni_left;

                // 左右子树，在当前节点数量配比的情况下，各自的组合情况
                let dp_ni_left = & dp[ni_left as usize];
                let dp_ni_right = & dp[ni_right as usize];

                // 组合他们所有可能性，构成多个子树，记录到当前数量下的数组里
                for left in dp_ni_left {
                    for right in dp_ni_right {
                        let root = TreeNode::new_with(left.clone(), right.clone());

                        dp_ni.push(Some(Rc::new(RefCell::new(root))));
                    }
                }
            }

            // 新增 dp[ni]
            dp[ni as usize] = dp_ni;
        }

        return dp.swap_remove(n as usize);
    }
}

fn main() {
    println!("Hello, world!");

    println!("{:?}", Solution::all_possible_fbt(5));
}
