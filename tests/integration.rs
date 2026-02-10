use merkletreelib::MerkleTree;
use merkletreelib::hash::sha256;
use merkletreelib::tree::verify_proof;

#[test]
fn merkle_tree_end_to_end_flow() {
    let data = vec!["a", "b", "c"];
    let mut tree = MerkleTree::from_bytes(&data);
    let leaf_hash = sha256("b".as_bytes());
    let proof = tree.generate_proof(&leaf_hash).unwrap();
    let root_before = *tree.get_root().unwrap();
    assert!(verify_proof(leaf_hash, &proof, root_before));
    tree.push("d".as_bytes());
    let root_after = *tree.get_root().unwrap();
    assert_ne!(root_before, root_after);
    assert!(!verify_proof(leaf_hash, &proof, root_after));
}
