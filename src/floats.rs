pub fn is_close(f1: f64, f2: f64) -> bool {
    (f1 - f2).abs() < 1e-5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_close() {
        assert!(is_close(1.234, 1.234));
        assert!(is_close(1.234, 1.2340001));
        assert!(!is_close(1.234, 1.235));
    }
}
