#[test]
fn insert_test() {
    let mut arr = vec![4, 3, 5, 4, 2, 1];
    let res = vec![1, 2, 3, 4, 4, 5];
    algorithm_sort_rust::insert::sort(&mut arr);

    assert_eq!(arr, res);
}

#[test]
fn selection_test() {
    let mut arr = vec![4, 3, 5, 4, 2, 1];
    let res = vec![1, 2, 3, 4, 4, 5];
    algorithm_sort_rust::selection::sort(&mut arr);

    assert_eq!(arr, res);
}

#[test]
fn shell_test() {
    let mut arr = vec![4, 3, 5, 4, 2, 1];
    let res = vec![1, 2, 3, 4, 4, 5];
    algorithm_sort_rust::shell::sort(&mut arr, 2);

    assert_eq!(arr, res);
}

#[test]
fn merge_test() {
    let mut arr = vec![4, 3, 5, 4, 2, 1];
    let res = vec![1, 2, 3, 4, 4, 5];
    algorithm_sort_rust::merge::sort(&mut arr);

    assert_eq!(arr, res);
}

#[test]
fn quick_test() {
    let mut arr = vec![4, 3, 5, 4, 2, 1];
    let res = vec![1, 2, 3, 4, 4, 5];
    algorithm_sort_rust::quick::sort(&mut arr);

    assert_eq!(arr, res);
}
