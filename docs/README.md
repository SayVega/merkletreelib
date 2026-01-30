## Design Overview
This library implements a simple and explicit Merkle Tree in Rust.
The design prioritizes clarity and correctness over performance, rebuilding the tree when new elements are added.

### MerkleTree::from_bytes
Builds a Merkle Tree from a collection of byte-representable values, if `values` is empty will return a Merkle Tree with no root.

Internally it starts hashing all the leaf nodes (initial values) using SHA-256, then builds the tree by recursively combining pairs of nodes until a single root node is produced. 
If the numbers of nodes at any level are odd it will duplicate the last one.

**Example:**
 ```
use merkletreelib::MerkleTree;

let data = vec!["a", "b", "c"];
let tree = MerkleTree::from_bytes(&data);
```
Its important to remind that this method only accepts data that can be represented as raw bytes (`AsRef<[u8]>`). Numeric and structurd type must be explicitly converted to bytes before hashing like this:
```
use merkletreelib::MerkleTree;

let value: u32 = 42;
let bytes = value.to_be_bytes();
let tree = MerkleTree::from_bytes(&[bytes]);
```
### MerkleTre::get_root
Returns the hash value of the root.

### MerkleTree::generate_proof
Generates a Merkle inclusion proof for a given leaf (Hashed value). It returns a vector with the sibling hashes and direction (left/right) required to reconstruct the Merkle tree (starting from leaf to root). If `target` was not found in the tree returns None.
The leaf is not included in the proof, or none if the tree is empty.

### verify_proof
Given a leaf (Already hashed), a Merkle proof, and a root hash, this function reconstructs the Merkle root.
Returns `true` if the proof is valid and correctly reconstructs `root_hash`, otherwise returns `false`.
An empty proof is only valid when `leaf_hash == root_hash`.

**Example**
```
use merkletreelib::MerkleTree;
use merkletreelib::hash::sha256;
use merkletreelib::tree::verify_proof;

let data = vec!["a", "b", "c"];
let tree = MerkleTree::from_bytes(&data);

let leaf_hash = sha256("b".as_bytes());
let proof = tree.generate_proof(&leaf_hash).unwrap();
let root = *tree.get_root().unwrap();
let is_valid = verify_proof(leaf_hash, &proof, root);
```

### MerkleTree::push
Adds a new element to the Merkle tree. It receives a non-hashed value, who its hashed internally.
This operation mutates the tree. Previously generated proofs must not be reused.

**Example**
```
use merkletreelib::MerkleTree;

let mut tree = MerkleTree::from_bytes(&["a", "b"]);
tree.push("c".as_bytes());
let root = tree.get_root().unwrap();
```

