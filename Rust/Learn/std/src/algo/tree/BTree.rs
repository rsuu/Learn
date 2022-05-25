/// 前序遍历 :: 根 ➜ 左 ➜ 右

/// 中序遍历 :: 左 ➜ 右 ➜ 根

/// 后序遍历 :: 左 ➜ 右 ➜ 根

/// 广度遍历

/// 深度遍历

/// 如果以搜索引擎为例的话
/// 广度遍历是指 获取最多的相关的搜索结果
/// 深度遍历是指 获取最精确的搜索结果
use std::{
    cmp::{PartialEq, PartialOrd},
    string::String,
};

#[derive(Debug)]
struct BinTree<T> {
    n: T,
    l: Option<Box<BinTree<T>>>,
    r: Option<Box<BinTree<T>>>,
}

impl<T: PartialEq + PartialOrd> BinTree<T> {
    pub fn new(val: T) -> BinTree<T> {
        BinTree {
            n: val,
            l: None,
            r: None,
        }
    }

    pub fn add(&mut self, val: T) {
        let update = if self.n == val {
            return;
        }
        // have val
        else if val > self.n {
            &mut self.r
        }
        // val is big
        else {
            &mut self.l
        }; // val is small
        match update {
            Some(update) => update.add(val),                     // dig deeper
            None => *update = Some(Box::new(BinTree::new(val))), // add a leaf
        }
    }

    pub fn search(&self, val: T) -> Option<T> {
        if val == self.n {
            Some(val)
        // found
        } else if val < self.n {
            self.l.as_ref()?.search(val)
        } else if val > self.n {
            self.r.as_ref()?.search(val)
        } else {
            None
            // not found
        }
    }

    pub fn get_max_depth_l(&self) -> usize {
        match &self.l {
            Some(node) => (&*node).get_max_depth_l() + 1,
            None => 0,
        }
        // 1. (&*node).get_max_depth_l() + 1
        // 2. (&*node).get_max_depth_l() + 1
        // 3. (&*node).get_max_depth_l() + 1
        // 4. (&*node).get_max_depth_l() + 1
        // 5. (&*node).get_max_depth_l() + 1
        // when None => 0
        // 5. 0+1
        // 4. 1+1
        // 3. 2+1
        // 2. 3+1
        // 1. 4+1
        // return usize
    }
    pub fn get_max_depth_r(&self) -> usize {
        match &self.r {
            Some(node) => (&*node).get_max_depth_r() + 1,
            None => 0,
        }
    }

    pub fn get_max_depth(&self) -> usize {
        std::cmp::max(self.get_max_depth_l(), self.get_max_depth_r())
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_usize() {
        let mut root = BinTree::new(9);
        root.add(9);
        root.add(8);
        root.add(7);
        root.add(6);
        root.add(5);
        root.add(10);

        assert_eq!(root.get_max_depth_l(), 4);
        assert_eq!(root.get_max_depth_r(), 1);
        assert_eq!(root.get_max_depth(), 4);
    }

    #[test]
    fn test_type_isize() {
        let mut root = BinTree::new(9);
        root.add(9);
        root.add(8);
        root.add(-7);
        root.add(6);
        root.add(-5);
        root.add(-10);

        assert_eq!(root.get_max_depth_l(), 3);
        assert_eq!(root.get_max_depth_r(), 0);
        assert_eq!(root.get_max_depth(), 3);
    }

    #[test]
    fn test_type_string() {
        let mut s = BinTree::new(String::from("angle"));
        s.add(String::from("all"));
        s.add(String::from("bell"));

        let res = s.search(String::from("bell"));

        assert_eq!(s.get_max_depth_l(), 1);
        assert_eq!(s.get_max_depth_r(), 1);
        assert_eq!(s.get_max_depth(), 1);
        assert_eq!(res, Some("bell".to_string()));
    }
}
