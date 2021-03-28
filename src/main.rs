mod binary_search;
mod bitwise_xor;
mod cyclic_sort;
mod sliding_window;

fn main() {
    assert_eq!(binary_search::binary_search(vec![4, 6, 10], 10), Ok(2));
}