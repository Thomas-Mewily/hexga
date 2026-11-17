use hexga_math::prelude::*;

fn main() {
    let start = vec2(2.0, 0.0);
    let end = vec2(0.0, 2.0);

    let midpoint = start.map_with(end, |a, b| (a + b) / 2.0);
    assert_eq!(midpoint, Vec2::ONE);
    let trippled = midpoint.map_into(|value| value * 3.0);

    let rotate_90_degrees = Mat2::from_row_array([[0.0, -1.0], [1.0, 0.0]]);
    let result = rotate_90_degrees * trippled;

    assert_eq!(result, vec2(-3.0, 3.0));
    println!("transformed vector: {result:?}");
}
