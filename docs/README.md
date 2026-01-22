## Design Overview
This library implements a simple and explicit Merkle Tree in Rust.
The design prioritizes clarity and correctness over performance, rebuilding the tree when new elements are added.

### build_leaves_array
The leaf nodes are created by hashing the provided data and storing the result in a **MerkleNode** array with no children.

### build_merkle_tree_recursively
Builds a Merkle Tree from a vector of Merkle nodes by recursively combining pairs of nodes until a single root node is produced. It assumes that all input nodes are valid leaves or intermediate Merkle nodes. If the number of nodes is odd at any level, the last node is duplicated.

The function returns the root `MerkleNode` of the tree, or `None` if the input vector is empty.

### sha-256
Takes a sequence of bytes and returns its hash computed using the SHA-256 algorithm.