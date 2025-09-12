# Seg Tree Representation Theory

## Using 2^k number of elements (`Range: [0, ..., 2^k)`)

- Advantage: Easy to implement and understand
- Disadvantage: Requires more memory
- Root Node: `1`
- We store `2^{k+1}` elements
- `data[2^k..2^{k+1})` represent the range `[0, ..., 2^k)`.
- The 2 power number of bits in `2^{k+1} / (idx + 1)` gives us the length of the range represented by the node.

### ToDo:

- Formula to find exact range covered by the node `idx`.
- For every range, find the 2 nodes and the corresponding lca.

### Iteration design:

- Consider the range `[left, right)`.
- There's an lca with range, say `[lca_left, lca_right)`.
- There are nodes with range `[left, left_node_right)` and `[right_node_left, right)`.
- `left_node_left` and `right_node_right` just correspond to `left` and `right` respectively.
- For simiplicity, let's call them `left_idx`, `right_idx` and `lca_idx`.

### Common operations:

- **Consume**: Operation to determine how the updates behave with the nodes.
  `consume(update, (mutable) data, size)`

- **Propagate**: Here, we are just adding some update to the already existing tag in our node.

    _Note: Propagate is only defined for non-leaf nodes._

    The pseudocode for `propagate(idx, update_val)`:
    1. If the children are leaves, just let them `consume(.., &.., 1)` the update.
    2. if `update_left` is the update at left node, set it to `update_left + update_val`
    3. if `update_right` is the update at right node, set it to `update_right + update_val`

- **Push node**: Here, we are consuming any range update to the node and propagating it to its children.

    The pseudocode for `push(idx)`:
    1. If `idx` is a leaf node, return.
    2. Store the update at the node `idx` in a variable, say `existing_update`
    3. Consume the update at `idx`
    4. Propagate `propagate(idx_child, existing_update)` to it's children.

- **Push node with update**: Here, we have another update to push to the children.

    The pseudocode for `push_with_update(idx, update_val)`:
    1. If `idx` is a leaf node, return.
    2. Store the update at the node `idx` in a variable, say `existing_update`
    3. Consume the update at `idx`
    4. Propagate the update `existing_update` to it's children.
    5. Propagate the update `update_val` to it's children.

- **Push nodes**: Here, we are pushing the node along some path.

    The pseudocode for `push_nodes(idx_from, idx_to)`:
    1. `push(idx_from)`
    2. if `idx_from` is `idx_to`, return.
    3. We want to determine if `idx_to` belongs to the left_subtree or the right.
    5. if `idx_to` lies in the left child of `idx_from`, `push_nodes(idx_from * 2)`
    6. if `idx_to` lies in the right child of `idx_from`, `push_nodes(idx_from * 2 + 1)`

- **Push nodes with update**: Here, we are pushing the node along some path with an update.

    The pseudocode for `push_nodes_with_update(idx_from, idx_to, update_val)`:
    1. `push_with_update(idx_from, update_val)`
    2. if `idx_from` is `idx_to`, return.
    3. We want to determine if `idx_to` belongs to the left_subtree or the right.
    5. if `idx_to` lies in the left child of `idx_from`, `push_with_update(idx_from * 2, update_val)`
    6. if `idx_to` lies in the right child of `idx_from`, `push_with_update(idx_from * 2 + 1, update_val)`

- **Pull node**: Here, we want to make sure the data in the children is up-to-date.

    The pseudocode for `pull(idx)` (Make sure the node is pushed)
    1. If it's a leaf node, ignore.
    2. Otherwise, `push(idx_child)` the children.
    3. Recompute data at this node.

- **Pull nodes**: Here, we want to make sure the data in the children is up-to-date.

    The pseudocode for `pull_nodes(idx_from, idx_to)`: (We assume the nodes have been pushed)
    1. `pull(idx_from)` (if not done already)
    2. start a while loop here with `idx = idx_from`
    3. `push(idx ^ 1)` (the sibling of `idx`)
    4. Recompute data at this node
    5. if `idx_from` is `idx_to`, break.
    6. Set `idx = idx / 2` and continue

- **Query**: Query the range `[left, right)`.

    The pseudocode for `query(left, right)`:
    1. Find the `idx_left`, `idx_right`, `idx_lca`.
    2. `push_nodes(idx_root, idx_lca)`
    3. `push_nodes(idx_lca, idx_left)`
    4. `push_nodes(idx_lca, idx_right)`
    5. `pull_nodes(idx_left, idx_lca)`
    6. `pull_nodes(idx_right, idx_lca)`
    7. `pull_nodes(idx_lca, idx_root)`

- **Update**: Update the range `[left, right)`.

    The pseudocode for `update(left, right, update)`
    1. Find the `idx_left`, `idx_right`, `idx_lca`.
    2. `push_nodes_with_update(idx_root, idx_lca, update)`
    3. `push_nodes_with_update(idx_lca, idx_left, update)`
    4. `push_nodes_with_update(idx_lca, idx_right, update)`
    5. `pull_nodes_update(idx_lca, idx_left, update)`
    6. `pull_nodes_update(idx_lca, idx_right, update)`
    7. `pull_nodes_update(idx_lca, idx_root, update)`
