use std::collections::HashMap;
use std::cmp;

/*
Given an array of characters where each character represents a fruit tree, you are given two baskets,
and your goal is to put maximum number of fruits in each basket. The only restriction is that each
basket can have only one type of fruit.

You can start with any tree, but you can’t skip a tree once you have started. You will pick one fruit
from each tree until you cannot, i.e., you will stop when you have to pick from a third fruit type.
*/
fn fruits_basket(arr: Vec<String>) -> usize {
    let mut start = 0;
    let mut max_length = 0;
    let mut frequency: HashMap<&str, u16> = HashMap::new();

    for end in 0..arr.len() {
        *frequency
            .entry(&*arr[end])
            .or_insert(0) += 1;

        while frequency.len() > 2 {
            *frequency
                .get_mut(&*arr[start])
                .unwrap() -= 1;

            if frequency[&*arr[start]] == 0 {
                frequency.remove(&*arr[start]);
            }
            start += 1;
        }
        max_length = cmp::max(max_length, end - start + 1)
    }

    return max_length;
}

fn find_max_sum_window(k: i32, arr: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut current_sum = 0;
    let mut start = 0;

    for (i, _num) in arr.iter().enumerate() {
        current_sum += _num;

        if i >= (k - 1) as usize {
            max = cmp::max(max, current_sum);
            current_sum -= arr[start];
            start += 1;
        }
    }

    return max;
}

fn find_smallest_sum_window(target_sum: i32, arr: Vec<i32>) -> i32 {
    let mut min = i32::max_value();
    let mut current_sum = 0;
    let mut start = 0;

    for (i, _num) in arr.iter().enumerate() {
        current_sum += _num;

        while current_sum >= target_sum {
            min = cmp::min(min, (i - start + 1) as i32);
            current_sum -= arr[start];
            start += 1;
        }
    }

    if min == i32::max_value() {
        return 0;
    }

    return min;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_fruits() {
        assert_eq!(fruits_basket(vec!["A", "B", "C", "A", "C"]
            .into_iter()
            .map(|s| s.to_owned()).collect()), 3)
    }

    #[test]
    fn get_max_window_size() {
        assert_eq!(find_max_sum_window(3, vec![2,1,5,1,3,2]), 9)
    }

    #[test]
    fn get_smallest_window_size() {
        assert_eq!(find_smallest_sum_window(7, vec![2,1,5,2,3,2]), 2)
    }
}

/*
max size = 10 * 10^3
80% of it could be the same value
= 8 * 10^3

i16 max size = 32 * 10^3
16 bit = 2 bytes

Do we need negative int?
Nope.
So we can use u16 which goes up to ~64 * 10^3
*/