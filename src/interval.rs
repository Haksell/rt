struct Interval {
    min: f32,
    max: f32,
}

impl Interval {
    fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }
}
