#[test]
fn test_less() {
    let v = vec![1, 2];
    assert_eq!(algorithm_sort_rust::less(v, 0, 1), true);
}

#[test]
fn test_exch() {
    let mut v = vec![1, 2];
    algorithm_sort_rust::exch(&mut v, 0, 1);
    assert_eq!(v[0], 2);
    assert_eq!(v[1], 1);
    assert_eq!(v.len(), 2);
}
