use std::ops::{AddAssign, SubAssign};

use crate::utils::*;

pub struct BinaryIndexedTree<T> {
    tree: Vec<T>,
    len: usize,
}

impl<T: AddAssign + SubAssign + Copy> BinaryIndexedTree<T> {
    pub fn from(mut tree: Vec<T>) -> Self {
        let len = tree.len();
        for x in (1..len).rev() {
            let begin = x - lowbit(x) + 1;
            for i in begin..x {
                let tmp = tree[i];
                tree[x] += tmp;
            }
        }
        BinaryIndexedTree { tree, len }
    }

    /// # Examples
    ///
    /// ```
    /// # use algorithm::data_structures::BinaryIndexedTree;
    ///
    /// let vec = vec![2; 10];
    /// let mut t = BinaryIndexedTree::from(vec);
    ///
    /// t.edit(0, 2);
    /// assert!(t.query_one(0) == 4);
    ///
    /// t.edit(0, -2);
    /// assert!(t.query_one(9) == 20);
    /// ```
    pub fn edit(&mut self, mut index: usize, delta: T) {
        while index < self.len {
            self.tree[index] += delta;
            if index == 0 {
                index += 1;
            } else {
                index += lowbit(index);
            }
        }
    }

    /// # Examples
    ///
    /// ```
    /// # use algorithm::data_structures::BinaryIndexedTree;
    ///
    /// let vec = vec![2; 10];
    /// let t = BinaryIndexedTree::from(vec);
    ///
    /// assert!(t.query_one(0) == 2);
    /// assert!(t.query_one(9) == 20);
    /// ```
    pub fn query_one(&self, mut index: usize) -> T {
        let mut ans = self.tree[index];
        while index > 0 {
            index -= lowbit(index);
            ans += self.tree[index];
        }
        ans
    }

    /// # Examples
    ///
    /// ```
    /// # use algorithm::data_structures::BinaryIndexedTree;
    ///
    /// let vec = vec![2; 10];
    /// let t = BinaryIndexedTree::from(vec);
    ///
    /// assert!(t.query_range(0, 0) == 0);
    /// assert!(t.query_range(0, 5) == 10);
    /// ```
    pub fn query_range(&self, index_left: usize, index_right: usize) -> T {
        let mut ans = self.query_one(index_right);
        ans -= self.query_one(index_left);
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::data_structures::BinaryIndexedTree;

    #[test]
    fn query_one() {
        let vec = vec![2; 20];
        let t = BinaryIndexedTree::from(vec);
        assert!(t.query_one(0) == 2);
        assert!(t.query_one(9) == 20);
        assert!(t.query_one(14) == 30);
        assert!(t.query_one(19) == 40);
    }

    #[test]
    fn query_range() {
        let vec = vec![2; 20];
        let t = BinaryIndexedTree::from(vec);
        assert!(t.query_range(0, 0) == 0);
        assert!(t.query_range(0, 5) == 10);
        assert!(t.query_range(5, 10) == 10);
        assert!(t.query_range(10, 19) == 18);
    }

    #[test]
    fn query_edit() {
        let vec = vec![2; 20];
        let mut t = BinaryIndexedTree::from(vec);
        t.edit(0, 2);
        assert!(t.query_one(0) == 4);
        t.edit(0, -2);
        assert!(t.query_one(9) == 20);
        t.edit(5, 3);
        assert!(t.query_one(14) == 33);
        t.edit(10, 10);
        assert!(t.query_one(19) == 53);
    }
}
