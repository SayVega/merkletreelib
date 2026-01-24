## Design Overview
This library implements a simple and explicit Merkle Tree in Rust.
The design prioritizes clarity and correctness over performance, rebuilding the tree when new elements are added.

### MerkleTree::from_bytes()
Builds a Merkle Tree from a collection of byte-representable values, if `values` are empty will return a Merkle Tree with no root.

Internally it starts hashing all the leaf nodes (initial values) using SHA-256, then builds the tree by recursively combining pairs of nodes until a single root node is produced. If the numbers of nodes at any level are odd it will duplicate the last one.

**Example:**
 ```
use merkletreelib::MerkleTree;
let data = vec!["a", "b", "c"];
let tree = MerkleTree::from_bytes(&data);
```
Its important to remind that this method only accepts data that can be represented as raw bytes (`AsRef<[u8]>`). Numeric and structurd type must be explicitly converted to bytes before hashing like this:
```
let value: u32 = 42;
let bytes = value.to_be_bytes();
let tree = MerkleTree::from_bytes(&[bytes]);
```