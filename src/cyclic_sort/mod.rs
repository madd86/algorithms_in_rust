pub fn cyclic_sort(nums: &mut Vec<usize>) -> Vec<usize> {
    let mut i = 0;
    while i < nums.len() {
        let j = nums[i] - 1;
        if nums[i] != nums[j] {
            nums.swap(i, j);
        } else {
            i += 1;
        }
    }
    return nums.to_vec();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_numbers() {
        assert_eq!(cyclic_sort(&mut vec![6, 5, 1, 2, 4, 3]), [1,2,3,4,5,6])
    }
}
