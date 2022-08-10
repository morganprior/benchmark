use std::cmp::Ordering;

#[inline]
pub fn partition(v: &mut [i32], l: usize, r: usize) -> usize {
    let pivot = (r + l) / 2;
    v.swap(pivot, r);

    let mut i = l;
    let mut j = l;

    while j < r {
        if v[j] < v[r] {
            v.swap(i, j);
            i += 1;
        }
        j += 1;
    }
    v.swap(i, r);

    i
}

pub fn find_kth(v: &mut [i32], l: usize, r: usize, k: usize) -> i32 {
    let partition_index = partition(v, l, r);

    match partition_index.cmp(&k) {
        Ordering::Less => find_kth(v, partition_index + 1, r, k),
        Ordering::Equal => v[partition_index],
        Ordering::Greater => find_kth(v, l, partition_index - 1, k),
    }
}
