use advent_utils::math::{Equations, Rat, Vec2, Vec3};
use itertools::Itertools;
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
struct Object {
    position: Vec3,
    velocity: Vec3,
}

fn solve_xy(
    objects: Vec<Object>,
    x_range: RangeInclusive<i128>,
    y_range: RangeInclusive<i128>,
) -> usize {
    let mut total = 0usize;
    for i in 0..objects.len() {
        let a = &objects[i];
        let a_pos = a.position.xy();
        let a_vel = a.velocity.xy();
        for j in (i + 1)..objects.len() {
            let b = &objects[j];
            let b_pos = b.position.xy();
            let b_vel = b.velocity.xy();
            if let Some(intersection) = get_intersection_xy(&a_pos, &a_vel, &b_pos, &b_vel) {
                match intersection {
                    Intersection2D::Point(p, t1, t2) => {
                        if t1.is_non_negative()
                            && t2.is_non_negative()
                            && p.x.in_range(&x_range)
                            && p.y.in_range(&y_range)
                        {
                            total += 1;
                        }
                    }
                    Intersection2D::Line(_, _) => {
                        continue;
                    }
                }
            }
        }
    }
    total
}

enum Intersection2D {
    Point(Vec2, Rat, Rat),
    Line(Vec2, Vec2),
}

fn get_intersection_xy(
    a_pos: &Vec2,
    a_vel: &Vec2,
    b_pos: &Vec2,
    b_vel: &Vec2,
) -> Option<Intersection2D> {
    if a_vel.x.is_zero() && b_vel.x.is_zero() {
        if a_pos.x == b_pos.x {
            return Some(Intersection2D::Line(a_pos.clone(), a_vel.clone()));
        }
        return None;
    }
    if b_vel.x.is_zero() {
        let t = (b_pos.x - a_pos.x) / a_vel.x;
        let y = a_pos.y + t * a_vel.y;
        let t2 = (y - b_pos.y) / b_vel.y;
        return Some(Intersection2D::Point(Vec2::new(b_pos.x, y), t, t2));
    }
    if a_vel.x.is_zero() {
        return get_intersection_xy(b_pos, b_vel, a_pos, a_vel);
    }
    if a_vel.y.is_zero() || b_vel.y.is_zero() {
        return match get_intersection_xy(
            &Vec2::new(a_pos.y, a_pos.x),
            &Vec2::new(a_vel.y, a_vel.x),
            &Vec2::new(b_pos.y, b_pos.x),
            &Vec2::new(b_vel.y, b_vel.x),
        ) {
            Some(Intersection2D::Point(Vec2 { x, y }, t1, t2)) => {
                Some(Intersection2D::Point(Vec2::new(y, x), t1, t2))
            }
            Some(Intersection2D::Line(Vec2 { x: x1, y: y1 }, Vec2 { x: x2, y: y2 })) => {
                Some(Intersection2D::Line(Vec2::new(y1, x1), Vec2::new(y2, x2)))
            }
            None => None,
        };
    }
    if a_vel.x * b_vel.y == a_vel.y * b_vel.x {
        let t1 = (a_pos.x - b_pos.x) / b_vel.x;
        let t2 = (a_pos.y - b_pos.y) / b_vel.y;

        return (t1 == t2).then_some(Intersection2D::Line(a_pos.clone(), a_vel.clone()));
    }

    let b_k = b_vel.y / b_vel.x;
    let d_x = a_pos.x - b_pos.x;
    let d_y = a_pos.y - b_pos.y;
    let bottom = a_vel.x * b_k - a_vel.y;
    let top = d_y - d_x * b_k;
    let m = top / bottom;
    let x = a_pos.x + a_vel.x * m;
    let y = a_pos.y + a_vel.y * m;
    let t = (x - b_pos.x) / b_vel.x;

    Some(Intersection2D::Point(Vec2::new(x, y), m, t))
}

fn parse(input: &str) -> Vec<Object> {
    input.lines().map(parse_object).collect_vec()
}
fn parse_object(input: &str) -> Object {
    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.trim_end_matches(','))
        .flat_map(|x| x.parse::<i128>().ok());
    let px = it.next().expect("Expected px");
    let py = it.next().expect("Expected py");
    let pz = it.next().expect("Expected pz");
    let vx = it.next().expect("Expected vx");
    let vy = it.next().expect("Expected vy");
    let vz = it.next().expect("Expected vz");
    Object {
        position: Vec3::new(px, py, pz),
        velocity: Vec3::new(vx, vy, vz),
    }
}

pub fn solve_part_1(file_content: &str) -> usize {
    solve_xy(
        parse(file_content),
        200000000000000..=400000000000000,
        200000000000000..=400000000000000,
    )
}

pub fn solve_part_2(file_content: &str) -> i64 {
    let objects = parse(file_content);
    let p0 = objects[0].position.clone();
    let p1 = objects[1].position.clone();
    let p2 = objects[2].position.clone();

    let v0 = objects[0].velocity.clone();
    let v1 = objects[1].velocity.clone();
    let v2 = objects[2].velocity.clone();

    let v0x = v0.x;
    let v0y = v0.y;
    let v0z = v0.z;

    let v1x = v1.x;
    let v1y = v1.y;
    let v1z = v1.z;

    let v2x = v2.x;
    let v2y = v2.y;
    let v2z = v2.z;

    let p0x = p0.x;
    let p0y = p0.y;
    let p0z = p0.z;

    let p1x = p1.x;
    let p1y = p1.y;
    let p1z = p1.z;

    let p2x = p2.x;
    let p2y = p2.y;
    let p2z = p2.z;

    let mut equations = Equations {
        lefts: vec![
            vec![
                v0y - v1y,
                -v0x + v1x,
                Rat::ZERO,
                -p0y + p1y,
                p0x - p1x,
                Rat::ZERO,
            ],
            vec![
                v0z - v1z,
                Rat::ZERO,
                -v0x + v1x,
                -p0z + p1z,
                Rat::ZERO,
                p0x - p1x,
            ],
            vec![
                Rat::ZERO,
                v0z - v1z,
                -v0y + v1y,
                Rat::ZERO,
                -p0z + p1z,
                p0y - p1y,
            ],
            vec![
                v0y - v2y,
                -v0x + v2x,
                Rat::ZERO,
                -p0y + p2y,
                p0x - p2x,
                Rat::ZERO,
            ],
            vec![
                v0z - v2z,
                Rat::ZERO,
                -v0x + v2x,
                -p0z + p2z,
                Rat::ZERO,
                p0x - p2x,
            ],
            vec![
                Rat::ZERO,
                v0z - v2z,
                -v0y + v2y,
                Rat::ZERO,
                -p0z + p2z,
                p0y - p2y,
            ],
        ],
        rights: vec![
            p0x * v0y - p0y * v0x - p1x * v1y + p1y * v1x,
            p0x * v0z - p0z * v0x - p1x * v1z + p1z * v1x,
            p0y * v0z - p0z * v0y - p1y * v1z + p1z * v1y,
            p0x * v0y - p0y * v0x - p2x * v2y + p2y * v2x,
            p0x * v0z - p0z * v0x - p2x * v2z + p2z * v2x,
            p0y * v0z - p0z * v0y - p2y * v2z + p2z * v2y,
        ],
    };

    equations
        .solve()
        .expect("Expected to have a solution")
        .into_iter()
        .take(3)
        .sum::<Rat>()
        .top
        .try_into()
        .expect("the result should be an integer with less than 64 bits")
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{parse, solve_part_2, solve_xy};

    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case(EXAMPLE, 7, 21, 2)]
    #[case(ACTUAL, 200000000000000, 400000000000000, 11246)]
    fn test_part1(
        #[case] input: &str,
        #[case] min_value: i128,
        #[case] max_value: i128,
        #[case] expected: usize,
    ) {
        let objects = parse(input);
        assert_eq!(
            solve_xy(objects, min_value..=max_value, min_value..=max_value),
            expected
        );
    }

    #[rstest]
    #[case(EXAMPLE, 47)]
    #[case(ACTUAL, 716_599_937_560_103)]
    fn test_part2(#[case] input: &str, #[case] expected: i64) {
        let res = solve_part_2(input);
        assert_eq!(res, expected);
    }
}
