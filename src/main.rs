use rt::Tuple;

fn main() {
    println!("{:?}", Tuple::new_point(4.0, -4.0, 3.0));
    println!("{:?}", Tuple::new_vector(-4.0, 4.0, -3.0));
}
