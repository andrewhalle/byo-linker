#[inline]
pub fn next_aligned_value(value: usize, align: usize) -> usize {
    if align == 0 || align == 1 {
        value
    } else {
        (value & (!align + 1)) + align
    }
}
