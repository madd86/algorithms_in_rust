

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_fruits() {
        assert_eq!(fruits_basket(vec!["A", "B", "C", "A", "C"]), 3)
    }
}
