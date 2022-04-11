use crate::{exch, less};

pub fn sort<T>(arr: &mut Vec<T>, step: usize)
    where T: PartialOrd + Default {
    let mut step_tmp = step;

    while step_tmp > 0 {
        for i in step_tmp..arr.len() {
            let mut j = i;
            while j >= step_tmp {
                if less(arr, j, j - step_tmp) {
                    exch(arr, j, j - step_tmp);
                } else {
                    break;
                }
                j -= step_tmp;
            }
        }
        step_tmp -= 1;
    }
}
