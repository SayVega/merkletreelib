use crate::hash::sha256;
#[derive(Clone)]
struct MerkleNode {
    hash: [u8; 32],
    left: Option<Box<MerkleNode>>,
    right: Option<Box<MerkleNode>>,
}

pub struct MerkleTree {
    root: Option<MerkleNode>,
    leaves: Vec<[u8; 32]>,
}
pub enum Direction {
    Left,
    Right,
}

impl MerkleTree {
    pub fn get_root(&self) -> Option<&[u8; 32]> {
        return self.root.as_ref().map(|n| &n.hash);
    }
    pub fn from_bytes<T: AsRef<[u8]>>(values: &[T]) -> Self {
        let leaves = build_leaves_array(values);
        let root = if !leaves.is_empty() {
            Some(build_merkle_tree_recursively(&leaves))
        } else {
            None
        };
        return MerkleTree {
            root,
            leaves: leaves.iter().map(|n| n.hash).collect(),
        };
    }
    pub fn push(&mut self, value: &[u8]) {
        let leaf_hashed = sha256(value);
        self.leaves.push(leaf_hashed);
        let nodes: Vec<MerkleNode> = self
            .leaves
            .iter()
            .map(|h| MerkleNode {
                hash: *h,
                left: None,
                right: None,
            })
            .collect();
        let new_tree = if nodes.is_empty() {
            None
        } else {
            Some(build_merkle_tree_recursively(&nodes))
        };
        return self.root = new_tree;
    }
    pub fn generate_proof(&self, target: &[u8; 32]) -> Option<Vec<([u8; 32], Direction)>> {
        let root = self.root.as_ref()?;
        let mut proof = Vec::new();
        if dfs_generate_proof(root, target, &mut proof) {
            return Some(proof);
        } else {
            return None;
        }
    }
}

pub fn verify_proof(
    leaf_hash: [u8; 32],
    proof: &[([u8; 32], Direction)],
    root_hash: [u8; 32],
) -> bool {
    let mut current = leaf_hash;
    for (sibling_hash, direction) in proof {
        let mut data = Vec::with_capacity(current.len() + sibling_hash.len());
        match direction {
            Direction::Left => {
                data.extend_from_slice(sibling_hash);
                data.extend_from_slice(&current);
            }
            Direction::Right => {
                data.extend_from_slice(&current);
                data.extend_from_slice(sibling_hash);
            }
        }
        current = sha256(&data);
    }
    return current == root_hash;
}

fn build_leaves_array<T: AsRef<[u8]>>(values: &[T]) -> Vec<MerkleNode> {
    return values
        .iter()
        .map(|value| {
            let hash = sha256(value.as_ref());
            MerkleNode {
                hash,
                left: None,
                right: None,
            }
        })
        .collect();
}

fn build_merkle_tree_recursively(nodes: &[MerkleNode]) -> MerkleNode {
    if nodes.len() == 1 {
        return nodes[0].clone();
    }
    let mut parents = Vec::new();
    let mut i: usize = 0;

    while i < nodes.len() {
        let left = nodes[i].clone();
        let right = if i + 1 < nodes.len() {
            nodes[i + 1].clone()
        } else {
            nodes[i].clone()
        };

        let mut data = Vec::with_capacity(left.hash.len() + right.hash.len());
        data.extend_from_slice(&left.hash);
        data.extend_from_slice(&right.hash);

        let hash = sha256(&data);
        parents.push(MerkleNode {
            hash,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        });
        i += 2;
    }
    return build_merkle_tree_recursively(&parents);
}

fn dfs_generate_proof(
    node: &MerkleNode,
    target: &[u8; 32],
    proof: &mut Vec<([u8; 32], Direction)>,
) -> bool {
    if node.left.is_none() && node.right.is_none() {
        return &node.hash == target;
    }
    if let Some(left) = &node.left {
        if dfs_generate_proof(left, target, proof) {
            if let Some(right) = &node.right {
                proof.push((right.hash, Direction::Right));
            }
            return true;
        }
    }
    if let Some(right) = &node.right {
        if dfs_generate_proof(right, target, proof) {
            if let Some(left) = &node.left {
                proof.push((left.hash, Direction::Left));
            }
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;
    mod tree_construction {
        use super::*;
        #[test]
        fn merkle_tree_is_deterministic() {
            let data = vec!["a", "b", "c", "d"];
            let t1 = MerkleTree::from_bytes(&data);
            let t2 = MerkleTree::from_bytes(&data);
            assert_eq!(t1.get_root(), t2.get_root());
        }
        #[test]
        fn empty_input_has_no_root() {
            let data: Vec<&[u8]> = vec![];
            let tree = MerkleTree::from_bytes(&data);
            assert!(tree.root.is_none());
        }
        #[test]
        fn single_element_tree() {
            let data: Vec<&[u8]> = vec![&[42u8]];
            let tree = MerkleTree::from_bytes(&data);
            let expected = sha256(&[42u8]);
            assert_eq!(tree.get_root(), Some(&expected));
        }
        #[test]
        fn multiple_elements_tree_has_root() {
            let data: Vec<&[u8]> = vec![&[1], &[2], &[3], &[4]];
            let tree = MerkleTree::from_bytes(&data);
            assert!(tree.root.is_some());
        }
        #[test]
        fn odd_number_of_elements_is_supported() {
            let data: Vec<&[u8]> = vec![&[1], &[2], &[3]];
            let tree = MerkleTree::from_bytes(&data);
            assert!(tree.root.is_some());
        }
    }
    mod proof_generation_and_verification {
        use super::*;
        #[test]
        fn changing_inputs_changes_root() {
            let base = MerkleTree::from_bytes(&["a", "b", "c"]);
            assert_ne!(
                base.get_root(),
                MerkleTree::from_bytes(&["c", "b", "a"]).get_root()
            );
            assert_ne!(
                base.get_root(),
                MerkleTree::from_bytes(&["a", "b", "d"]).get_root()
            );
            assert_ne!(
                base.get_root(),
                MerkleTree::from_bytes(&["a", "b"]).get_root()
            );
        }
        #[test]
        fn generate_and_verify_proof_for_each_leaf() {
            let data = vec!["a", "b", "c", "d"];
            let tree = MerkleTree::from_bytes(&data);
            let root = *tree.get_root().unwrap();
            for value in &data {
                let leaf_hash = sha256(value.as_bytes());
                let proof = tree.generate_proof(&leaf_hash).unwrap();
                assert!(verify_proof(leaf_hash, &proof, root));
            }
        }
        #[test]
        fn proof_for_non_existing_leaf_fails() {
            let values: Vec<&str> = vec!["a", "b", "c"];
            let tree = MerkleTree::from_bytes(&values);
            let fake_hash = sha256("z".as_bytes());
            let proof = tree.generate_proof(&fake_hash);
            assert!(proof.is_none());
        }
        #[test]
        fn odd_number_of_leaves_padding_case() {
            let values: Vec<&str> = vec!["a", "b", "c"];
            let tree = MerkleTree::from_bytes(&values);
            let leaf_hash = sha256("c".as_bytes());
            let proof = tree.generate_proof(&leaf_hash).unwrap();
            let root = *tree.get_root().unwrap();
            assert!(verify_proof(leaf_hash, &proof, root));
        }
        #[test]
        fn single_element_has_empty_proof() {
            let values: Vec<&str> = vec!["only"];
            let tree = MerkleTree::from_bytes(&values);
            let leaf_hash = sha256("only".as_bytes());
            let proof = tree.generate_proof(&leaf_hash).unwrap();
            let root = *tree.get_root().unwrap();
            assert!(proof.is_empty());
            assert!(verify_proof(leaf_hash, &proof, root));
        }
        #[test]
        fn empty_tree_has_no_root_and_no_proofs() {
            let tree = MerkleTree::from_bytes::<&[u8]>(&[]);
            assert!(tree.get_root().is_none());
            assert!(tree.generate_proof(&[0u8; 32]).is_none());
        }
        #[test]
        fn verify_proof_accepts_valid_single_step_proof() {
            let leaf = sha256("a".as_bytes());
            let sibling = sha256("b".as_bytes());
            let mut data = Vec::new();
            data.extend_from_slice(&leaf);
            data.extend_from_slice(&sibling);
            let root = sha256(&data);
            let proof = vec![(sibling, Direction::Right)];
            assert!(verify_proof(leaf, &proof, root));
        }
        #[test]
        fn verify_proof_fails_if_sibling_hash_is_wrong() {
            let leaf = sha256("a".as_bytes());
            let correct_sibling = sha256("b".as_bytes());
            let wrong_sibling = sha256("x".as_bytes());
            let mut data = Vec::new();
            data.extend_from_slice(&leaf);
            data.extend_from_slice(&correct_sibling);
            let root = sha256(&data);
            let proof = vec![(wrong_sibling, Direction::Right)];
            assert!(!verify_proof(leaf, &proof, root));
        }
        #[test]
        fn verify_proof_fails_if_direction_is_wrong() {
            let leaf = sha256("a".as_bytes());
            let sibling = sha256("b".as_bytes());
            let mut data = Vec::new();
            data.extend_from_slice(&leaf);
            data.extend_from_slice(&sibling);
            let root = sha256(&data);
            let proof = vec![(sibling, Direction::Left)];
            assert!(!verify_proof(leaf, &proof, root));
        }
        #[test]
        fn verify_proof_empty_is_valid_only_when_leaf_equals_root() {
            let leaf = sha256("a".as_bytes());
            let other = sha256("x".as_bytes());
            assert!(verify_proof(leaf, &[], leaf));
            assert!(!verify_proof(leaf, &[], other));
        }
    }
    mod tree_dinamic_update {
        use super::*;
        #[test]
        fn push_produces_same_root_as_from_bytes() {
            let values = vec!["a", "b", "c", "d"];
            let direct_tree = MerkleTree::from_bytes(&values);
            let mut pushed_tree = MerkleTree::from_bytes::<&[u8]>(&[]);
            for v in &values {
                pushed_tree.push(v.as_bytes());
            }
            assert_eq!(direct_tree.get_root(), pushed_tree.get_root());
        }
        #[test]
        fn push_changes_root_and_invalidates_old_proof() {
            let mut tree = MerkleTree::from_bytes(&["a", "b"]);
            let leaf_hash = sha256("a".as_bytes());
            let proof = tree.generate_proof(&leaf_hash).unwrap();
            let old_root: [u8; 32] = *tree.get_root().unwrap();
            tree.push("c".as_bytes());
            let new_root: [u8; 32] = *tree.get_root().unwrap();
            assert_ne!(old_root, new_root);
            assert!(!verify_proof(leaf_hash, &proof, new_root));
        }
    }
}
