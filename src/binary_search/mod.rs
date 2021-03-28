pub(crate) fn binary_search(search_arr: Vec<i32>, key: i32) -> Result<usize, usize> {
    let mut start = 0;
    let mut end = search_arr.len() - 1;
    let is_ascending = search_arr[start] < search_arr[end];

    while start <= end {
        // calculate the middle of the current range
        let mid = start + (end - start) / 2;

        if key == search_arr[mid] {
            return Ok(mid);
        }

        if is_ascending {
            if key < search_arr[mid] {
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        } else {
            if key > search_arr[mid] {
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }
    }

    return Err(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn found_number() {
        assert_eq!(binary_search(vec![4, 6, 10], 10), Ok(2))
    }

    #[test]
    fn found_number_larger_array() {
        assert_eq!(binary_search(vec![2, 4, 6, 10, 11, 33, 44, 123, 125, 132, 155, 166, 215, 214], 155), Ok(10))
    }

    #[test]
    fn no_number() {
        assert_eq!(binary_search(vec![4, 6, 10], 11), Err(0))
    }
}