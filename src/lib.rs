use std::mem;

pub mod insert;
pub mod merge;
pub mod selection;
pub mod shell;

pub fn less<T>(arr: &Vec<T>, lo: usize, hi: usize) -> bool
    where
        T: PartialOrd,
{
    return arr.get(lo).expect("") < arr.get(hi).expect("");
}

pub fn exch<T>(arr: &mut Vec<T>, lo: usize, hi: usize)
    where
        T: PartialOrd + Default,
{
    let tmp_lo = arr.get_mut(lo).map(|tmp| mem::take(tmp)).expect("");
    let tmp_hi = arr.get_mut(hi).map(|tmp| mem::take(tmp)).expect("");
    *arr.get_mut(lo).expect("") = tmp_hi;
    *arr.get_mut(hi).expect("") = tmp_lo;
}
