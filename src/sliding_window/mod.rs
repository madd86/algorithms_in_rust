use std::collections::HashMap;
use std::cmp;

fn fruits_basket(arr: Vec<String>) -> usize {
    let mut start = 0;
    let mut max_length = 0;
    let mut frequency: HashMap<&str, u16> = HashMap::new();

    for end in 0..arr.len() {
        *frequency.entry(&*arr[end]).or_insert(0) += 1;
        while frequency.len() > 2 {
            *frequency.get_mut(&*arr[start]).unwrap() -= 1;
            if frequency[&*arr[start]] == 0 {
                frequency.remove(&*arr[start]);
            }
            start += 1;
        }
        max_length = cmp::max(max_length, end - start + 1)
    }

    return max_length;
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