use super::BSTree;

#[test]
fn create_new() {
    let tree = BSTree::new(24);

    assert_eq!(tree.len(), 1);
    assert_eq!(tree.get(24), Some(&24));
}
