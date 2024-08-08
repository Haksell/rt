mod tuple;

pub use tuple::Tuple;

type Float = f32; // TODO: try f64

fn is_close(f1: Float, f2: Float) -> bool {
    const EPSILON: Float = 1e-6;
    (f1 - f2).abs() < EPSILON
}