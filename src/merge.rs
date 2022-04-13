use std::mem;
use crate::less;

pub fn sort<T>(arr: &mut Vec<T>)
    where T: PartialOrd + Default + std::fmt::Debug {
    let mut aux: Vec<T> = Vec::new();
    for _ in 0..arr.len() {
        aux.push(T::default());
    }
    sort_util(arr, &mut aux, 0, arr.len() - 1);
}

fn sort_util<T>(arr: &mut Vec<T>, aux: &mut Vec<T>, hi: usize, lo: usize)
    where T: PartialOrd + Default + std::fmt::Debug {
    if hi >= lo {
        return;
    }
    let mid = hi + (lo - hi) / 2;
    sort_util(arr, aux, hi, mid);
    sort_util(arr, aux, mid + 1, lo);
    merge(arr, aux, hi, lo, mid);
}

fn merge<T>(arr: &mut Vec<T>, aux: &mut Vec<T>, hi: usize, lo: usize, mid: usize)
    where T: PartialOrd + Default + std::fmt::Debug {
    for i in hi..lo + 1 {
        *aux.get_mut(i).expect("") = move_form_vec(arr, i);
    }
    let mut lt = hi;
    let mut rt = mid + 1;
    for i in hi..lo + 1 {
        if lt > mid {
            *arr.get_mut(i).expect("") = move_form_vec(aux, rt);
            rt += 1;
        } else if rt > lo {
            *arr.get_mut(i).expect("") = move_form_vec(aux, lt);
            lt += 1;
        } else if less(aux, rt, lt) {
            *arr.get_mut(i).expect("") = move_form_vec(aux, rt);
            rt += 1;
        } else {
            *arr.get_mut(i).expect("") = move_form_vec(aux, lt);
            lt += 1;
        }
    }
}

fn move_form_vec<T>(arr: &mut Vec<T>, index: usize) -> T
    where T: Default {
    arr.get_mut(index).map(|tmp| mem::take(tmp)).expect("")
}

