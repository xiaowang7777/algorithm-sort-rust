pub fn sort<T>(arr: &mut Vec<T>)
    where T: PartialOrd + Default {
    for i in 0..arr.len() {
        let mut min_idx = i;
        for j in i..arr.len() {
            if crate::less(arr, j, min_idx) {
                min_idx = j;
            }
        }
        if min_idx != i {
            crate::exch(arr, min_idx, i);
        }
    }
}