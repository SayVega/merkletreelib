use crate::hash::sha256;
#[derive(Clone)]
pub struct MerkleNode {
    pub hash: [u8; 32],
    pub left: Option<Box<MerkleNode>>,
    pub right: Option<Box<MerkleNode>>,
}
pub struct MerkleTree {
    pub root: Option<MerkleNode>,
}

pub fn build_leaves_array<T: AsRef<[u8]>>(values: &[T]) -> Vec<MerkleNode>
{
    values
        .iter()
        .map(|value| {
            let hash = sha256(value.as_ref());

            MerkleNode {
                hash,
                left: None,
                right: None,
            }
        })
        .collect()
}

pub fn build_merkle_tree_recursively(nodes: &[MerkleNode]) -> MerkleNode {
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

    build_merkle_tree_recursively(&parents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_leaves_creates_one_leaf_per_input() {
        let data = vec!["a", "b", "c"];
        let leaves = build_leaves_array(&data);
        assert_eq!(leaves.len(), 3);
    }

    #[test]
    fn leaves_have_no_children() {
        let data = vec!["hello"];
        let leaves = build_leaves_array(&data);
        let leaf = &leaves[0];
        assert!(leaf.left.is_none());
        assert!(leaf.right.is_none());
    }

    #[test]
    fn single_node_returns_itself_as_root() {
    let data = vec!["only"];
    let leaves = build_leaves_array(&data);
    let root = build_merkle_tree_recursively(&leaves);
    assert_eq!(root.hash, leaves[0].hash);
    assert!(root.left.is_none());
    assert!(root.right.is_none());
    }

    #[test]
    fn tree_with_two_leaves_has_children() {
    let data = vec!["a", "b"];
    let leaves = build_leaves_array(&data);
    let root = build_merkle_tree_recursively(&leaves);
    assert!(root.left.is_some());
    assert!(root.right.is_some());
    }

    #[test]
    fn odd_number_of_leaves_duplicates_last() {
    let data = vec!["a", "b", "c"];
    let leaves = build_leaves_array(&data);
    let root = build_merkle_tree_recursively(&leaves);
    assert!(root.left.is_some());
    assert!(root.right.is_some());
    }

    #[test]
    fn merkle_tree_is_order_sensitive() {
    let data1 = vec!["a", "b"];
    let data2 = vec!["b", "a"];
    let leaves1 = build_leaves_array(&data1);
    let leaves2 = build_leaves_array(&data2);
    let root1 = build_merkle_tree_recursively(&leaves1);
    let root2 = build_merkle_tree_recursively(&leaves2);
    assert_ne!(root1.hash, root2.hash);
    }

#[test]
fn merkle_tree_is_deterministic() {
    let data = vec!["a", "b", "c", "d"];
    let leaves1 = build_leaves_array(&data);
    let leaves2 = build_leaves_array(&data);

    let root1 = build_merkle_tree_recursively(&leaves1);
    let root2 = build_merkle_tree_recursively(&leaves2);

    assert_eq!(root1.hash, root2.hash);
    }
}
