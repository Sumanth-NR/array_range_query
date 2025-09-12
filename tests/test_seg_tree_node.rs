use array_range_query::SegTreeNode;

#[cfg(test)]
mod comprehensive_tests {
    use super::*;

    // ===== BASIC NAVIGATION TESTS =====

    #[test]
    fn test_tree_navigation_various_depths() {
        // Test navigation at different levels
        let root = SegTreeNode(1);

        // Level 1
        let left_1 = root.left_child();
        let right_1 = root.right_child();
        assert_eq!(left_1.0, 2);
        assert_eq!(right_1.0, 3);

        // Level 2
        let left_2_left = left_1.left_child();
        let left_2_right = left_1.right_child();
        let right_2_left = right_1.left_child();
        let right_2_right = right_1.right_child();

        assert_eq!(left_2_left.0, 4);
        assert_eq!(left_2_right.0, 5);
        assert_eq!(right_2_left.0, 6);
        assert_eq!(right_2_right.0, 7);

        // Test parent relationships
        assert_eq!(left_2_left.parent().0, 2);
        assert_eq!(left_2_right.parent().0, 2);
        assert_eq!(right_2_left.parent().0, 3);
        assert_eq!(right_2_right.parent().0, 3);
    }

    #[test]
    fn test_sibling_relationships() {
        // Test siblings across multiple levels
        let pairs = [
            (2, 3), // Level 1
            (4, 5), // Level 2 left subtree
            (6, 7), // Level 2 right subtree
            (8, 9), // Level 3
            (10, 11),
            (12, 13),
            (14, 15),
        ];

        for (left_idx, right_idx) in pairs {
            let left = SegTreeNode(left_idx);
            let right = SegTreeNode(right_idx);

            assert_eq!(left.sibling().0, right_idx);
            assert_eq!(right.sibling().0, left_idx);
            assert!(left.is_left_child());
            assert!(right.is_right_child());
        }
    }

    // ===== LEVEL AND BOUNDS TESTS =====

    #[test]
    fn test_level_calculation_comprehensive() {
        let test_cases = [
            (1, 0), // Root
            (2, 1),
            (3, 1), // Level 1
            (4, 2),
            (5, 2),
            (6, 2),
            (7, 2), // Level 2
            (8, 3),
            (9, 3),
            (10, 3),
            (11, 3),
            (12, 3),
            (13, 3),
            (14, 3),
            (15, 3), // Level 3
            (16, 4),
            (31, 4), // Level 4
            (32, 5),
            (63, 5), // Level 5
        ];

        for (node_idx, expected_level) in test_cases {
            let node = SegTreeNode(node_idx);
            assert_eq!(
                node.level(),
                expected_level,
                "Node {} should be at level {}",
                node_idx,
                expected_level
            );
        }
    }

    #[test]
    fn test_node_bounds_various_depths() {
        // Test for max_depth = 3 (array size 8)
        let max_depth = 3;

        let test_cases = [
            // (node_idx, expected_left, expected_right, expected_size)
            (1, 0, 8, 8), // Root covers [0, 8)
            (2, 0, 4, 4), // Left child covers [0, 4)
            (3, 4, 8, 4), // Right child covers [4, 8)
            (4, 0, 2, 2), // Level 2 nodes
            (5, 2, 4, 2),
            (6, 4, 6, 2),
            (7, 6, 8, 2),
            (8, 0, 1, 1), // Level 3 nodes (leaves)
            (9, 1, 2, 1),
            (10, 2, 3, 1),
            (11, 3, 4, 1),
            (12, 4, 5, 1),
            (13, 5, 6, 1),
            (14, 6, 7, 1),
            (15, 7, 8, 1),
        ];

        for (node_idx, expected_left, expected_right, expected_size) in test_cases {
            let node = SegTreeNode(node_idx);

            assert_eq!(
                node.node_left_bound(max_depth),
                expected_left,
                "Node {} left bound",
                node_idx
            );
            assert_eq!(
                node.node_right_bound(max_depth),
                expected_right,
                "Node {} right bound",
                node_idx
            );
            assert_eq!(
                node.node_bounds(max_depth),
                (expected_left, expected_right),
                "Node {} bounds",
                node_idx
            );
            assert_eq!(
                node.node_size(max_depth),
                expected_size,
                "Node {} size",
                node_idx
            );
        }
    }

    #[test]
    fn test_node_bounds_different_max_depths() {
        let root = SegTreeNode(1);

        // Test different tree depths
        let depth_tests = [
            (1, 0, 2, 2),   // max_depth = 1, array size 2
            (2, 0, 4, 4),   // max_depth = 2, array size 4
            (3, 0, 8, 8),   // max_depth = 3, array size 8
            (4, 0, 16, 16), // max_depth = 4, array size 16
        ];

        for (max_depth, expected_left, expected_right, expected_size) in depth_tests {
            assert_eq!(root.node_left_bound(max_depth), expected_left);
            assert_eq!(root.node_right_bound(max_depth), expected_right);
            assert_eq!(root.node_size(max_depth), expected_size);
        }
    }

    #[test]
    fn test_leaf_detection() {
        let max_depth = 3;

        // Non-leaf nodes
        let non_leaves = [1, 2, 3, 4, 5, 6, 7];
        for node_idx in non_leaves {
            let node = SegTreeNode(node_idx);
            assert!(
                !node.is_leaf(max_depth),
                "Node {} should not be a leaf",
                node_idx
            );
        }

        // Leaf nodes
        let leaves = [8, 9, 10, 11, 12, 13, 14, 15];
        for node_idx in leaves {
            let node = SegTreeNode(node_idx);
            assert!(
                node.is_leaf(max_depth),
                "Node {} should be a leaf",
                node_idx
            );
        }
    }

    // ===== LCA (Lowest Common Ancestor) TESTS =====

    #[test]
    fn test_lca_same_depth_various_cases() {
        let test_cases = [
            // (left_node, right_node, expected_lca)
            (2, 3, 1),   // Adjacent siblings at level 1
            (4, 5, 2),   // Adjacent siblings at level 2
            (6, 7, 3),   // Adjacent siblings at level 2
            (4, 6, 1),   // Different subtrees at level 2
            (4, 7, 1),   // Different subtrees at level 2
            (5, 6, 1),   // Different subtrees at level 2
            (8, 9, 4),   // Adjacent siblings at level 3
            (10, 11, 5), // Adjacent siblings at level 3
            (8, 10, 2),  // Same grandparent at level 3
            (8, 12, 1),  // Different subtrees at level 3
            (9, 14, 1),  // Different subtrees at level 3
        ];

        for (left_idx, right_idx, expected_lca) in test_cases {
            let left = SegTreeNode(left_idx);
            let right = SegTreeNode(right_idx);
            let lca = SegTreeNode::get_lca_from_same_depth(left, right);

            assert_eq!(
                lca.0, expected_lca,
                "LCA of nodes {} and {} should be {}",
                left_idx, right_idx, expected_lca
            );
        }
    }

    #[test]
    fn test_lca_different_depth() {
        let test_cases = [
            // (left_node, right_node, expected_lca)
            (1, 8, 1),  // Root and deep leaf
            (2, 8, 2),  // Node and its descendant
            (2, 9, 2),  // Node and its descendant
            (2, 12, 1), // Node and leaf in different subtree
            (3, 8, 1),  // Node and leaf in different subtree
            (4, 9, 4),  // Node and its descendant
            (4, 10, 2), // Node and cousin
            (5, 8, 2),  // Cousins with different depths
        ];

        for (left_idx, right_idx, expected_lca) in test_cases {
            let left = SegTreeNode(left_idx);
            let right = SegTreeNode(right_idx);
            let lca = SegTreeNode::get_lca_from_different_depth(left, right);

            assert_eq!(
                lca.0, expected_lca,
                "LCA of nodes {} and {} should be {}",
                left_idx, right_idx, expected_lca
            );
        }
    }

    #[test]
    fn test_lca_edge_cases() {
        // Same node
        let node = SegTreeNode(5);
        let lca = SegTreeNode::get_lca_from_same_depth(node.clone(), node.clone());
        assert_eq!(lca.0, 5);

        // Root with itself
        let root = SegTreeNode(1);
        let lca = SegTreeNode::get_lca_from_same_depth(root.clone(), root.clone());
        assert_eq!(lca.0, 1);
    }

    // ===== BINDING NODES TESTS =====

    #[test]
    fn test_left_binding_node() {
        let test_cases = [
            // (node_idx, expected_binding)
            (1, 1),   // Root (already not a left child since it has no parent)
            (2, 1),   // Left child -> go up until not a left child
            (3, 3),   // Right child -> stop
            (4, 1),   // Left child of left child -> go up to root
            (5, 5),   // Right child -> stop
            (6, 3),   // Left child -> go up to 3 (which is a right child)
            (7, 7),   // Right child -> stop
            (8, 1),   // Deep left path
            (9, 9),   // Right child -> stop
            (10, 5),  // Left child -> go up to 5 (which is a right child)
            (11, 11), // Right child -> stop
        ];

        for (node_idx, expected_binding) in test_cases {
            let node = SegTreeNode(node_idx);
            let binding = node.get_left_binding_node();
            assert_eq!(
                binding.0, expected_binding,
                "Left binding of node {} should be {}",
                node_idx, expected_binding
            );
        }
    }

    #[test]
    fn test_right_binding_node() {
        let test_cases = [
            // (node_idx, expected_binding)
            (1, 1),   // Root -> stop at root
            (2, 2),   // Left child -> stop
            (3, 1),   // Right child -> go up to 1 (root)
            (4, 4),   // Left child -> stop
            (5, 2),   // Right child -> go up to 2 (left child)
            (6, 6),   // Left child -> stop
            (7, 1),   // Right child -> go up through 3 to 1 (root)
            (8, 8),   // Left child -> stop
            (9, 4),   // Right child -> go up to 4 (left child)
            (10, 10), // Left child -> stop
            (11, 2),  // Right child -> go up through 5 to 2 (left child)
        ];

        for (node_idx, expected_binding) in test_cases {
            let node = SegTreeNode(node_idx);
            let binding = node.get_right_binding_node();
            assert_eq!(
                binding.0, expected_binding,
                "Right binding of node {} should be {}",
                node_idx, expected_binding
            );
        }
    }

    // ===== CORNER CASES AND EDGE CASES =====

    #[test]
    fn test_large_tree_structure() {
        // Test with a larger tree (depth 6, size 64)
        let max_depth = 6;
        let root = SegTreeNode(1);

        assert_eq!(root.node_size(max_depth), 64);
        assert_eq!(root.node_bounds(max_depth), (0, 64));

        // Test deep leaf
        let deep_leaf = SegTreeNode(64); // First leaf at depth 6
        assert!(deep_leaf.is_leaf(max_depth));
        assert_eq!(deep_leaf.node_size(max_depth), 1);
        assert_eq!(deep_leaf.node_bounds(max_depth), (0, 1));

        // Test last leaf
        let last_leaf = SegTreeNode(127); // Last leaf at depth 6
        assert!(last_leaf.is_leaf(max_depth));
        assert_eq!(last_leaf.node_bounds(max_depth), (63, 64));
    }

    #[test]
    fn test_minimal_tree() {
        // Test with minimal tree (depth 1, size 2)
        let max_depth = 1;
        let root = SegTreeNode(1);

        assert_eq!(root.node_size(max_depth), 2);
        assert_eq!(root.node_bounds(max_depth), (0, 2));
        assert!(!root.is_leaf(max_depth));

        // Test leaves
        let left_leaf = SegTreeNode(2);
        let right_leaf = SegTreeNode(3);

        assert!(left_leaf.is_leaf(max_depth));
        assert!(right_leaf.is_leaf(max_depth));
        assert_eq!(left_leaf.node_bounds(max_depth), (0, 1));
        assert_eq!(right_leaf.node_bounds(max_depth), (1, 2));
    }

    #[test]
    fn test_mathematical_properties() {
        // Test that parent-child relationships are consistent
        for node_idx in 2..=31 {
            let node = SegTreeNode(node_idx);
            let parent = node.parent();

            if node.is_left_child() {
                assert_eq!(parent.left_child().0, node_idx);
            } else {
                assert_eq!(parent.right_child().0, node_idx);
            }
        }

        // Test that sibling relationships are symmetric
        for node_idx in 2..=31 {
            let node = SegTreeNode(node_idx);
            let sibling = node.sibling();
            assert_eq!(sibling.sibling().0, node_idx);
        }

        // Test that level calculation is consistent with tree structure
        for level in 0..=5 {
            let first_node_at_level = 1 << level;
            let last_node_at_level = (1 << (level + 1)) - 1;

            for node_idx in first_node_at_level..=last_node_at_level {
                let node = SegTreeNode(node_idx);
                assert_eq!(
                    node.level(),
                    level,
                    "Node {} should be at level {}",
                    node_idx,
                    level
                );
            }
        }
    }

    #[test]
    fn test_bounds_consistency() {
        let max_depth = 4;

        // Test that child bounds are subsets of parent bounds
        for node_idx in 1..=15 {
            let node = SegTreeNode(node_idx);

            if !node.is_leaf(max_depth) {
                let left_child = node.left_child();
                let right_child = node.right_child();

                let (parent_left, parent_right) = node.node_bounds(max_depth);
                let (left_child_left, left_child_right) = left_child.node_bounds(max_depth);
                let (right_child_left, right_child_right) = right_child.node_bounds(max_depth);

                // Left child should cover first half
                assert_eq!(left_child_left, parent_left);
                assert_eq!(left_child_right, (parent_left + parent_right) / 2);

                // Right child should cover second half
                assert_eq!(right_child_left, (parent_left + parent_right) / 2);
                assert_eq!(right_child_right, parent_right);

                // Children should be adjacent and cover entire parent range
                assert_eq!(left_child_right, right_child_left);
            }
        }
    }

    #[test]
    fn test_comprehensive_tree_traversal() {
        // Test a complete traversal of a small tree
        let max_depth = 3;
        let mut visited_bounds = Vec::new();

        // Collect all leaf node bounds
        for leaf_idx in 8..=15 {
            let leaf = SegTreeNode(leaf_idx);
            assert!(leaf.is_leaf(max_depth));
            visited_bounds.push(leaf.node_bounds(max_depth));
        }

        // Sort bounds by left boundary
        visited_bounds.sort();

        // Verify they form a complete, non-overlapping coverage of [0, 8)
        let mut expected_left = 0;
        for (left, right) in visited_bounds {
            assert_eq!(left, expected_left);
            assert_eq!(right - left, 1); // Each leaf covers exactly 1 element
            expected_left = right;
        }
        assert_eq!(expected_left, 8);
    }
}
