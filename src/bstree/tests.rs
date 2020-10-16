use super::BSTree;

#[test]
fn create_new() {
    let tree = BSTree::new(24);

    assert_eq!(tree.len(), 1);
    assert_eq!(tree.get(24), Some(&24));
}

#[test]
fn add_values() {
    let mut tree = BSTree::new(24);
    tree.add(40);
    tree.add(32);
    tree.add(2);

    assert_eq!(tree.len(), 4);
    assert_eq!(tree.get(32), Some(&32));
    assert_eq!(tree.get(202), None);
}

#[test]
fn get_max() {
    let mut tree = BSTree::new(24);
    tree.add(40);
    tree.add(32);
    tree.add(2);

    assert_eq!(tree.max(), &40);
}

#[test]
fn get_min() {
    let mut tree = BSTree::new(24);
    tree.add(40);
    tree.add(32);
    tree.add(2);

    assert_eq!(tree.min(), &2);
}

#[test]
fn get_ordered_vec() {
    let mut tree = BSTree::new(24);
    tree.add(40);
    tree.add(32);
    tree.add(2);

    assert_eq!(tree.ordered_traversal(), vec![&2, &24, &32, &40]);
}