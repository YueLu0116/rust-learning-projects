pub mod first;

#[cfg(test)]
mod tests {
    #[test]
    fn basics() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
