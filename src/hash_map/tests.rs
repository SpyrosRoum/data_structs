use super::HashMap;

#[test]
fn create_new() {
    let map = HashMap::<i32, i32>::new();
    assert_eq!(map.len(), 0);
}

#[test]
fn insert_get() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    assert_eq!(map.len(), 1);
    assert_eq!(map.insert("b", 2), None);
    assert_eq!(map.len(), 2);

    assert_eq!(map.get(&"a"), Some(&1));
    assert_eq!(map.get(&"b"), Some(&2));
    assert_eq!(map.get(&"c"), None);

    assert_eq!(map.insert("a", 2), Some(1));
    assert_eq!(map.get(&"a"), Some(&2));
}

#[test]
fn remove() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);

    assert_eq!(map.remove(&"a"), Some(1));
    assert_eq!(map.remove(&"c"), None);
}

#[test]
fn get_mut() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);

    let mut v = map.get_mut(&"a").unwrap();
    *v = 2;
    assert_eq!(map.get(&"a"), Some(&2));
}
