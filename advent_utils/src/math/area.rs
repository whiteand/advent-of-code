use glam::I64Vec2;

/**
 * Returns signed the area of the triangle multiplied by two
 */
pub fn double_triangle_area(a: I64Vec2, b: I64Vec2, c: I64Vec2) -> i64 {
    let u = b - a;
    let v = c - a;
    u.x * v.y - u.y * v.x
}
