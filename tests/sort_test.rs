#[test]
fn insert_test() {
    let mut arr = vec![4, 3, 5, 4, 2, 1];
    let res = vec![1, 2, 3, 4, 4, 5];
    algorithm_sort_rust::insert::sort(&mut arr);

    assert_eq!(arr, res);
}

#[test]
fn selection_test(){
    let mut arr = vec![4, 3, 5, 4, 2, 1];
    let res = vec![1, 2, 3, 4, 4, 5];
    algorithm_sort_rust::selection::sort(&mut arr);

    assert_eq!(arr, res);
}