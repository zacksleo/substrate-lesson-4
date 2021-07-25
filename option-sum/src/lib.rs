pub fn sum(arr: &[u32]) -> Option<u32> {
    arr.iter().fold(Some(0), |acc, x| match acc {
        None => None,
        Some(val) => val.checked_add(*x),
    })
}
