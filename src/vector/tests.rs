use super::Vector;

#[test]
fn create_new() {
    let vec = Vector::<i32>::new();

    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 0);
}

#[test]
fn add_values() {
    let mut vec = Vector::new();
    vec.push(1);
    assert_eq!(vec.len(), 1);
    assert_eq!(vec.capacity(), 4);

    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);

    assert_eq!(vec.len(), 5);
    assert_eq!(vec.capacity(), 8);
}

#[test]
fn remove_values() {
    let mut vec = Vector::new();
    vec.push(1);
    assert_eq!(vec.len(), 1);
    assert_eq!(vec.capacity(), 4);

    assert_eq!(vec.pop(), Some(1));
    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 4);
    assert_eq!(vec.pop(), None);

    vec.push(2);
    vec.push(3);

    assert_eq!(vec.len(), 2);
    assert_eq!(vec.pop(), Some(3));
}

#[test]
fn index() {
    let mut vec = Vector::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    assert_eq!(vec[0], 1);
    assert_eq!(vec[1], 2);
    assert_eq!(vec[1..], *vec![2, 3, 4].as_slice());
    vec[3] = 2;
    assert_eq!(vec[3], 2);
    assert_eq!(vec.pop(), Some(2))
}
