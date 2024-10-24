pub fn is_close(f1: f64, f2: f64) -> bool {
    (f1 - f2).abs() < 1e-6
}
