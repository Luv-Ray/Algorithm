/// ```
/// # use algorithm::utils::lowbit;
/// assert_eq!(lowbit(0b0), 0b0);
/// assert_eq!(lowbit(0b100100), 0b100);
/// ```
pub fn lowbit(x: usize) -> usize {
    if x == 0 {
        0
    } else {
        x & (!x + 1)
    }
}
