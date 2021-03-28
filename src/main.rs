mod binary_search;

fn main() {
    assert_eq!(binary_search::binary_search(vec![4, 6, 10], 10), Ok(2));
}