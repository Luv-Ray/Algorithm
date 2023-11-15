use std::ops::{AddAssign, SubAssign};

fn lowbit(x: usize) -> usize {
    x & (!x + 1)
}

pub struct BinaryIndexedTree<T> {
    tree: Vec<T>,
    len: usize,
}

impl<T: AddAssign + SubAssign + Copy> BinaryIndexedTree<T> {
    pub fn new(mut tree: Vec<T>) -> Self {
        let len = tree.len();
        for x in (1..len).rev() {
            for i in x - lowbit(x) + 1..x {
                let tmp = tree[i];
                tree[x] += tmp;
            }
        }
        BinaryIndexedTree { tree, len }
    }

    /// # Examples
    /// 
    /// ```
    /// use algorithm::data_structures::BinaryIndexedTree;
    /// 
    /// let vec = vec![2; 10];
    /// let mut t = BinaryIndexedTree::new(vec);
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
    /// use algorithm::data_structures::BinaryIndexedTree;
    /// 
    /// let vec = vec![2; 10];
    /// let t = BinaryIndexedTree::new(vec);
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
    /// use algorithm::data_structures::BinaryIndexedTree;
    /// 
    /// let vec = vec![2; 10];
    /// let t = BinaryIndexedTree::new(vec);
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
