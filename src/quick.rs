use std::fmt::Debug;
use crate::{exch, less};

pub fn sort<T>(arr: &mut Vec<T>)
    where T: PartialOrd + Default {
    sort_util(arr, 0, arr.len() - 1)
}

fn sort_util<T>(arr: &mut Vec<T>, lo: usize, hi: usize)
    where T: PartialOrd + Default {
    if lo >= hi {
        return;
    }
    let p = partition(arr, lo, hi);
    if p > 0 {
        sort_util(arr, lo, p - 1);
    }
    sort_util(arr, p + 1, hi);
}

fn partition<T>(arr: &mut Vec<T>, lo: usize, hi: usize) -> usize
    where T: PartialOrd + Default {
    let p = lo;
    let mut i = lo;
    let mut j = hi + 1;

    loop {
        i += 1;
        j -= 1;
        while less(arr, i, p) {
            if i == p {
                break;
            }
            i += 1;
        }
        while less(arr, p, j) {
            if p == j {
                break;
            }
            j -= 1;
        }
        if i >= j {
            break;
        }
        exch(arr, i, j);
    }
    exch(arr, p, j);
    j
}