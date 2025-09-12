//! # SegTreeNode Module
//!
//! This module provides `SegTreeNode`, a node representation for power-of-two layout segment trees.
//! The layout uses 1-based indexing where:
//! - Root is at index 1
//! - For node i: left child = 2*i, right child = 2*i+1, parent = i/2
//!
//! ## Tree Structure
//! ```text
//! Level 0:     1
//! Level 1:   2   3
//! Level 2:  4 5 6 7
//! Level 3: 8 9 ...
//! ```
//!
//! Each node represents a range [left, right) in the underlying array.

/// A node in a power-of-two layout segment tree.
///
/// This struct wraps a `usize` index representing a node's position in the tree.
/// The tree uses 1-based indexing for efficient parent/child calculations.
///
/// # Examples
///
/// ```rust
/// use array_range_query::SegTreeNode;
///
/// let root = SegTreeNode(1);
/// let left_child = root.left_child();
/// let right_child = root.right_child();
///
/// assert_eq!(left_child.0, 2);
/// assert_eq!(right_child.0, 3);
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct SegTreeNode(pub usize);

impl SegTreeNode {
    // ===== NAVIGATION =====

    /// Returns the left child of this node.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use array_range_query::SegTreeNode;
    ///
    /// let node = SegTreeNode(1);
    /// assert_eq!(node.left_child().0, 2);
    /// ```
    #[inline]
    pub fn left_child(&self) -> SegTreeNode {
        SegTreeNode(self.0 * 2)
    }

    /// Returns the right child of this node.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use array_range_query::SegTreeNode;
    ///
    /// let node = SegTreeNode(1);
    /// assert_eq!(node.right_child().0, 3);
    /// ```
    #[inline]
    pub fn right_child(&self) -> SegTreeNode {
        SegTreeNode(self.0 * 2 + 1)
    }

    /// Returns the parent of this node.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use array_range_query::SegTreeNode;
    ///
    /// let node = SegTreeNode(4);
    /// assert_eq!(node.parent().0, 2);
    /// ```
    #[inline]
    pub fn parent(&self) -> SegTreeNode {
        if self.is_root() {
            panic!("Root node has no parent")
        } else {
            SegTreeNode(self.0 / 2)
        }
    }

    /// Returns the sibling of this node.
    /// Assumes that the node is not root.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use array_range_query::SegTreeNode;
    ///
    /// let left = SegTreeNode(4);
    /// let right = SegTreeNode(5);
    /// assert_eq!(left.sibling().0, 5);
    /// assert_eq!(right.sibling().0, 4);
    /// ```
    #[inline]
    pub fn sibling(&self) -> SegTreeNode {
        SegTreeNode(self.0 ^ 1)
    }

    /// Returns the sibling of this node.
    ///
    /// For a non-root node, the sibling is obtained by flipping the least-significant
    /// bit of the node index (i.e. sibling_index = self.0 ^ 1). The tree uses
    /// 1-based indexing, therefore the root node (index 1) has no sibling and
    /// calling this method on the root will panic with the message
    /// "Root node has no sibling".
    ///
    /// If you know the node is not root, you can use the `sibling()` method instead.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use array_range_query::SegTreeNode;
    ///
    /// let left = SegTreeNode(2);
    /// let right = left.sibling_safe();
    /// assert_eq!(right.0, 3);
    ///
    /// let root = SegTreeNode(1);
    /// assert!(root.is_root());
    /// // root.sibling_safe(); // would panic: "Root node has no sibling"
    /// ```
    #[inline]
    pub fn sibling_safe(&self) -> SegTreeNode {
        if self.is_root() {
            panic!("Root node has no sibling")
        }
        SegTreeNode(self.0 ^ 1)
    }

    // ===== PROPERTIES =====

    /// Checks if the current node is root
    ///
    /// # Examples
    ///
    /// ```rust
    /// use array_range_query::SegTreeNode;
    ///
    /// let node = SegTreeNode(1);
    /// assert!(node.is_root());
    /// ```
    #[inline]
    pub fn is_root(&self) -> bool {
        self.0 == 1
    }

    /// Returns true if this node is a left child of its parent.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use array_range_query::SegTreeNode;
    ///
    /// let left = SegTreeNode(4);
    /// let right = SegTreeNode(5);
    /// assert!(left.is_left_child());
    /// assert!(!right.is_left_child());
    /// ```
    #[inline]
    pub fn is_left_child(&self) -> bool {
        !self.is_root() && self.0 & 1 == 0
    }

    /// Given that this node is not root,
    /// checks if it is the left child of its parent.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use array_range_query::SegTreeNode;
    ///
    /// let left = SegTreeNode(4);
    /// let right = SegTreeNode(5);
    /// assert!(left.is_left_child_if_not_root());
    /// assert!(!right.is_left_child_if_not_root());
    /// ```
    #[inline]
    pub fn is_left_child_if_not_root(&self) -> bool {
        self.0 & 1 == 0
    }

    /// Returns true if this node is a right child of its parent.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use array_range_query::SegTreeNode;
    ///
    /// let left = SegTreeNode(4);
    /// let right = SegTreeNode(5);
    /// assert!(!left.is_right_child());
    /// assert!(right.is_right_child());
    /// ```
    #[inline]
    pub fn is_right_child(&self) -> bool {
        !self.is_root() && self.0 & 1 == 1
    }

    /// Given that this node is not root,
    /// checks if it is the right child of its parent.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use array_range_query::SegTreeNode;
    ///
    /// let left = SegTreeNode(4);
    /// let right = SegTreeNode(5);
    /// assert!(!left.is_right_child_if_not_root());
    /// assert!(right.is_right_child_if_not_root());
    /// ```
    #[inline]
    pub fn is_right_child_if_not_root(&self) -> bool {
        self.0 & 1 == 1
    }

    /// Returns the depth of this node in the tree.
    /// Root is at depth 0.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use array_range_query::SegTreeNode;
    ///
    /// let root = SegTreeNode(1);
    /// let depth1 = SegTreeNode(2);
    /// let depth2 = SegTreeNode(4);
    /// assert_eq!(root.depth(), 0);
    /// assert_eq!(depth1.depth(), 1);
    /// assert_eq!(depth2.depth(), 2);
    /// ```
    #[inline]
    pub fn depth(&self) -> u32 {
        self.0.ilog2()
    }

    /// Returns true if this node is a leaf (at maximum depth).
    ///
    /// # Parameters
    /// - `max_depth`: The maximum depth of the tree
    ///
    /// # Examples
    ///
    /// ```rust
    /// use array_range_query::SegTreeNode;
    ///
    /// let root = SegTreeNode(1);
    /// let leaf = SegTreeNode(8);
    /// assert!(!root.is_leaf(3));
    /// assert!(leaf.is_leaf(3));
    /// ```
    #[inline]
    pub fn is_leaf(&self, max_depth: u32) -> bool {
        self.depth() == max_depth
    }

    // ===== BOUNDS / RANGE HELPERS =====

    /// Returns the size of the range this node represents.
    ///
    /// # Parameters
    /// - `max_depth`: The maximum depth of the tree (depth of leaf nodes)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use array_range_query::SegTreeNode;
    ///
    /// let root = SegTreeNode(1);
    /// let leaf = SegTreeNode(8);
    /// assert_eq!(root.size(3), 8);  // Root covers entire array of size 8
    /// assert_eq!(leaf.size(3), 1);  // Leaf covers single element
    /// ```
    #[inline]
    pub fn size(&self, max_depth: u32) -> usize {
        1 << (max_depth - self.depth())
    }

    /// Returns the left boundary of the range this node represents.
    ///
    /// # Parameters
    /// - `max_depth`: The maximum depth of the tree (depth of leaf nodes)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use array_range_query::SegTreeNode;
    ///
    /// let node = SegTreeNode(2);  // Left child of root
    /// assert_eq!(node.left_bound(3), 0);  // Covers [0, 4)
    /// ```
    #[inline]
    pub fn left_bound(&self, max_depth: u32) -> usize {
        let depth = self.depth();
        let pos = self.0 - (1 << depth);
        pos * (1 << (max_depth - depth))
    }

    /// Returns the right boundary of the range this node represents.
    ///
    /// # Parameters
    /// - `max_depth`: The maximum depth of the tree (depth of leaf nodes)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use array_range_query::SegTreeNode;
    ///
    /// let node = SegTreeNode(2);  // Left child of root
    /// assert_eq!(node.right_bound(3), 4);  // Covers [0, 4)
    /// ```
    #[inline]
    pub fn right_bound(&self, max_depth: u32) -> usize {
        let depth = self.depth();
        let pos = self.0 - (1 << depth);
        (pos + 1) * (1 << (max_depth - depth))
    }

    #[inline]
    pub fn mid(&self, max_depth: u32) -> usize {
        let depth = self.depth();
        let pos = self.0 - (1 << depth);
        let range = 1 << (max_depth - depth);
        pos * range + range / 2
    }

    /// Returns the range [left, right) that this node represents.
    ///
    /// # Parameters
    /// - `max_depth`: The maximum depth of the tree (depth of leaf nodes)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use array_range_query::SegTreeNode;
    ///
    /// let root = SegTreeNode(1);
    /// assert_eq!(root.node_bounds(3), (0, 8));  // Root covers [0, 8)
    /// ```
    #[inline]
    pub fn node_bounds(&self, max_depth: u32) -> (usize, usize) {
        let depth = self.depth();
        let pos = self.0 - (1 << depth);
        let range = 1 << (max_depth - depth);
        (pos * range, (pos + 1) * range)
    }

    // ===== LCA HELPERS =====

    /// Finds the Lowest Common Ancestor (LCA) of two nodes at the same depth.
    ///
    /// # Parameters
    /// - `left`: First node
    /// - `right`: Second node (must be at same depth as `left`)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use array_range_query::SegTreeNode;
    ///
    /// let node4 = SegTreeNode(4);
    /// let node5 = SegTreeNode(5);
    /// let lca = SegTreeNode::get_lca_from_same_depth(node4, node5);
    /// assert_eq!(lca.0, 2);  // Parent of both 4 and 5
    /// ```
    pub fn get_lca_from_same_depth(left: SegTreeNode, right: SegTreeNode) -> SegTreeNode {
        // Fast path: if nodes already equal, return immediately
        let xor = left.0 ^ right.0;
        if xor == 0 {
            return left;
        }
        // Find the highest differing bit (0-based from LSB) and shift both nodes
        // right by (highest_diff_bit + 1). This directly yields their LCA.
        //
        // Example: left=4 (100), right=5 (101) => xor=1 (highest=0), shift=1 => 4>>1 = 2 (LCA)
        let highest = usize::BITS - 1 - xor.leading_zeros();
        let shift = (highest + 1) as usize;
        SegTreeNode(left.0 >> shift)
    }

    /// Finds the Lowest Common Ancestor (LCA) of two nodes at potentially different depths.
    ///
    /// # Parameters
    /// - `left`: First node
    /// - `right`: Second node
    ///
    /// # Examples
    ///
    /// ```rust
    /// use array_range_query::SegTreeNode;
    ///
    /// let node2 = SegTreeNode(2);  // Level 1
    /// let node4 = SegTreeNode(4);  // Level 2, child of 2
    /// let lca = SegTreeNode::get_lca_from_different_depth(node2, node4);
    /// assert_eq!(lca.0, 2);  // LCA is node 2 itself
    /// ```
    pub fn get_lca_from_different_depth(
        mut left: SegTreeNode,
        mut right: SegTreeNode,
    ) -> SegTreeNode {
        if left.depth() >= right.depth() {
            left.0 >>= left.depth() - right.depth();
        } else {
            right.0 >>= right.depth() - left.depth();
        }
        Self::get_lca_from_same_depth(left, right)
    }

    // ===== BINDING HELPERS =====

    /// Returns the binding node by traversing up while this node is a left child.
    /// Used in segment tree range operations.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use array_range_query::SegTreeNode;
    ///
    /// let node = SegTreeNode(4);  // Left child
    /// let binding = node.get_left_binding_node();
    /// // Traverses up until finding a node that's not a left child
    /// ```
    pub fn get_left_binding_node(&self) -> SegTreeNode {
        SegTreeNode((self.0 >> self.0.trailing_zeros()).max(1))
    }

    /// Returns the binding node by traversing up while this node is a right child.
    /// Used in segment tree range operations.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use array_range_query::SegTreeNode;
    ///
    /// let node = SegTreeNode(5);  // Right child
    /// let binding = node.get_right_binding_node();
    /// // Traverses up until finding a node that's not a right child
    /// ```
    pub fn get_right_binding_node(&self) -> SegTreeNode {
        SegTreeNode((self.0 >> self.0.trailing_ones()).max(1))
    }
}

#[cfg(test)]
mod tests {
    use super::SegTreeNode;

    #[test]
    fn test_basic_navigation() {
        let root = SegTreeNode(1);
        let left = root.left_child();
        let right = root.right_child();

        assert_eq!(left.0, 2);
        assert_eq!(right.0, 3);
        assert_eq!(left.parent().0, 1);
        assert_eq!(right.parent().0, 1);
    }

    #[test]
    #[should_panic(expected = "Root node has no sibling")]
    fn test_root_node() {
        let root = SegTreeNode(1);
        assert!(root.is_root());
        assert!(!root.is_left_child());
        assert!(!root.is_right_child());
        root.sibling_safe();
    }

    #[test]
    fn test_sibling_and_child_properties() {
        let node2 = SegTreeNode(2);
        let node3 = SegTreeNode(3);

        assert_eq!(node2.sibling().0, 3);
        assert_eq!(node3.sibling().0, 2);
        assert!(node2.is_left_child());
        assert!(node3.is_right_child());
    }

    #[test]
    fn test_depth_calculation() {
        let root = SegTreeNode(1);
        let depth1 = SegTreeNode(2);
        let depth2 = SegTreeNode(4);

        assert_eq!(root.depth(), 0);
        assert_eq!(depth1.depth(), 1);
        assert_eq!(depth2.depth(), 2);
    }

    #[test]
    fn test_bounds_calculation() {
        let root = SegTreeNode(1);
        let max_depth = 3;

        assert_eq!(root.size(max_depth), 8);
        assert_eq!(root.left_bound(max_depth), 0);
        assert_eq!(root.right_bound(max_depth), 8);
        assert_eq!(root.node_bounds(max_depth), (0, 8));
    }

    #[test]
    fn test_leaf_detection() {
        let root = SegTreeNode(1);
        let leaf = SegTreeNode(8);
        let max_depth = 3;

        assert!(!root.is_leaf(max_depth));
        assert!(leaf.is_leaf(max_depth));
    }

    #[test]
    fn test_lca_same_depth() {
        let node4 = SegTreeNode(4);
        let node5 = SegTreeNode(5);
        let lca = SegTreeNode::get_lca_from_same_depth(node4, node5);

        assert_eq!(lca.0, 2);
    }

    #[test]
    fn test_lca_different_depth() {
        let node2 = SegTreeNode(2);
        let node4 = SegTreeNode(4);
        let lca = SegTreeNode::get_lca_from_different_depth(node2, node4);

        assert_eq!(lca.0, 2);
    }

    #[test]
    fn test_binding_nodes() {
        let node4 = SegTreeNode(4);
        let node5 = SegTreeNode(5);

        let left_binding = node4.get_left_binding_node();
        let right_binding = node5.get_right_binding_node();

        // Basic test that methods execute without panic
        assert!(left_binding.0 > 0);
        assert!(right_binding.0 > 0);
    }
}
