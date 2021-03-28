pub fn cyclic_sort(&nums: Vec<i32>) -> Vec<i32> {
    let mut i = 0;

    while i < nums.len() {
        if nums[i] < nums.len() && nums[i] != nums[nums[i]] {
            nums.swap(i, nums[i])
        } else {
            i += 1
        }
    }

    return nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_numbers() {
        assert_eq!(cyclic_sort(vec![6,5,1,2,4,3]), [1,2,3,4,5,6])
    }
}
