pub fn find_missing_number(arr: Vec<i32>) -> i32 {
    // Adding two here helps because it covers '0' index plus the missing number
    let len = arr.len() + 2;
    let mut s1 = 0;
    for i in 0..len {
        s1 += i as i32;
    }

    for e in &arr {
        s1 -= e;
    }

    return s1 as i32;
}

pub fn find_non_duplicate(arr: Vec<i32>) -> i32 {
    let mut num = 0;
    for i in 0..arr.len() {
        num ^= arr[i]
    }

    return num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_number() {
        assert_eq!(find_missing_number(vec![1,5,2,6,4]), 3);
        assert_eq!(find_missing_number(vec![1,2,3,6,4,5,7,8,9,10,12]), 11);
    }

    #[test]
    fn find_non_duplicate_number() {
        assert_eq!(find_non_duplicate(vec![1,1,6,9,6,7,3,7,9]), 3)
    }
}
