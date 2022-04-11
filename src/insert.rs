use crate::{exch, less};

pub fn sort<T>(arr: &mut Vec<T>)
    where T: PartialOrd + Default {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 {
            if less(arr, j, j - 1) {
                exch(arr, j, j - 1);
            } else {
                break;
            }
            j -= 1;
        }
    }
}