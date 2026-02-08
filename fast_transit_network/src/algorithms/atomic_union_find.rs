use std::sync::atomic::{AtomicUsize, Ordering};

/// Thread-safe Union-Find for parallel WCC using atomics (no locks).
pub struct AtomicUnionFind {
    parent: Vec<AtomicUsize>,
}

impl AtomicUnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).map(|i| AtomicUsize::new(i)).collect(),
        }
    }

    /// Thread-safe find with path compression (try to skip to grandparent).
    pub fn find(&self, mut x: usize) -> usize {
        loop {
            let parent = self.parent[x].load(Ordering::Relaxed);
            if parent == x {
                return x;
            }

            let grandparent = self.parent[parent].load(Ordering::Relaxed);
            if grandparent == parent {
                return parent;
            }

            self.parent[x]
                .compare_exchange(parent, grandparent, Ordering::Relaxed, Ordering::Relaxed)
                .ok();

            x = parent;
        }
    }

    /// Thread-safe union: always link smaller root to larger for consistency.
    pub fn union(&self, x: usize, y: usize) {
        loop {
            let root_x = self.find(x);
            let root_y = self.find(y);

            if root_x == root_y {
                return;
            }

            let (small, large) = if root_x < root_y {
                (root_x, root_y)
            } else {
                (root_y, root_x)
            };

            match self.parent[small].compare_exchange(
                small,
                large,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => return,
                Err(_) => continue,
            }
        }
    }

    /// Returns the final component id (root) for each element.
    pub fn get_components(&self) -> Vec<usize> {
        (0..self.parent.len()).map(|i| self.find(i)).collect()
    }
}
