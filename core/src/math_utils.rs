//! Various math tools that are usefull for several modules.

use std::cmp;
use std::f32::consts::PI;
use math::*;

pub fn fuzzy_eq_f32(a: f32, b: f32) -> bool {
    let epsilon = 0.000001;
    return (a - b).abs() <= epsilon;
}

pub fn fuzzy_eq(a: Vec2, b: Vec2) -> bool { fuzzy_eq_f32(a.x, b.x) && fuzzy_eq_f32(a.y, b.y) }

/// Defines an ordering between two points
pub fn is_below(a: Vec2, b: Vec2) -> bool {
    a.y > b.y || (a.y == b.y && a.x > b.x)
}

/// Defines an ordering between two points
pub fn is_below_int(a: IntVec2, b: IntVec2) -> bool {
    a.y > b.y || (a.y == b.y && a.x > b.x)
}

// Compute the vector from ce center of an ellipse on of its points
pub fn ellipse_center_to_point(center: Vec2, ellipse_point: Vec2, radii: Vec2) -> Vec2{
    vec2(
        (ellipse_point.x - center.x) / radii.x,
        (ellipse_point.y - center.y) / radii.y,
    )
}

pub fn ellipse_point_from_angle(center: Vec2, radii: Vec2, angle: f32) -> Vec2{
    vec2(
        center.x + radii.x * angle.cos(),
        center.y + radii.y * angle.sin()
    )
}


/// Angle between vectors v1 and v2 (oriented clockwise assyming y points downwards).
/// The result is a number between 0 and 2*PI.
///
/// ex: directed_angle([0,1], [1,0]) = 3/2 Pi rad
///     x       __
///   0-->     /  \
///  y|       |  x--> v2
///   v        \ |v1
///              v
///
/// Or, assuming y points upwards:
/// directed_angle([0,-1], [1,0]) = 1/2 Pi rad
///
///   ^           v2
///  y|          x-->
///   0-->    v1 | /
///     x        v-
///
pub fn directed_angle(a:Vec2, b: Vec2) -> f32 {
    let angle = (b.y).atan2(b.x) - (a.y).atan2(a.x);
    return if angle < 0.0 { angle + 2.0 * PI } else { angle };
}

pub fn directed_angle2(center: Vec2,  a: Vec2, b: Vec2) -> f32 {
    directed_angle(a - center, b - center)
}

pub fn angle_between(start_vector : Vec2, end_vector : Vec2) -> f32 {
    let mut result = ((start_vector.x * end_vector.x + start_vector.y * end_vector.y) /
                 (start_vector.length() * end_vector.length())).acos() ;

    if (start_vector.x*end_vector.y - start_vector.y*end_vector.x) < 0.0{
        result = - result;
    }
    result
}


pub fn tangent(v: Vec2) -> Vec2 {
    let l = v.length();
    return vec2(-v.y / l, v.x / l);
}

pub fn line_intersection(
    a1: Vec2,
    a2: Vec2,
    b1: Vec2,
    b2: Vec2
) -> Option<Vec2> {
    let det = (a1.x - a2.x) * (b1.y - b2.y) - (a1.y - a2.y) * (b1.x - b2.x);
    if det.abs() <= 0.000001 {
        // The lines are very close to parallel
        return None;
    }
    let inv_det = 1.0 / det;
    let a = a1.x * a2.y - a1.y * a2.x;
    let b = b1.x * b2.y - b1.y * b2.x;
    return Some(vec2(
        (a * (b1.x - b2.x) - b * (a1.x - a2.x)) * inv_det,
        (a * (b1.y - b2.y) - b * (a1.y - a2.y)) * inv_det
    ));
}

pub fn segment_intersection(
    a1: Vec2,
    b1: Vec2,
    a2: Vec2,
    b2: Vec2
) -> Option<Vec2> {
    let v1 = b1 - a1;
    let v2 = b2 - a2;
    if fuzzy_eq(v2, vec2(0.0, 0.0)) {
        return None;
    }

    let v1_cross_v2 = v1.cross(v2);
    let a2_a1_cross_v1 = (a2 - a1).cross(v1);

    if v1_cross_v2 == 0.0 {
        if a2_a1_cross_v1 == 0.0 {

            let v1_sqr_len = v1.square_length();
            // check if a2 is between a1 and b1
            let v1_dot_a2a1 = v1.dot(a2 - a1);
            if v1_dot_a2a1 > 0.0 && v1_dot_a2a1 < v1_sqr_len { return Some(a2); }

            // check if b2 is between a1 and b1
            let v1_dot_b2a1 = v1.dot(b2 - a1);
            if v1_dot_b2a1 > 0.0 && v1_dot_b2a1 < v1_sqr_len { return Some(b2); }

            let v2_sqr_len = v2.square_length();
            // check if a1 is between a2 and b2
            let v2_dot_a1a2 = v2.dot(a1 - a2);
            if v2_dot_a1a2 > 0.0 && v2_dot_a1a2 < v2_sqr_len { return Some(a1); }

            // check if b1 is between a2 and b2
            let v2_dot_b1a2 = v2.dot(b1 - a2);
            if v2_dot_b1a2 > 0.0 && v2_dot_b1a2 < v2_sqr_len { return Some(b1); }

            return None;
        }

        return None;
    }

    let t = (a2 - a1).cross(v2) / v1_cross_v2;
    let u = a2_a1_cross_v1 / v1_cross_v2;

    // TODO :(
    if t > 0.00001 && t < 0.9999 && u > 0.00001 && u < 0.9999 {
        return Some(a1 + (v1 * t));
    }

    return None;
}

pub fn segment_intersection_int(
    _a1: IntVec2,
    _b1: IntVec2,
    _a2: IntVec2,
    _b2: IntVec2
) -> Option<IntVec2> {
    if _a1 == _a2 || _a1 == _b1 || _b1 == _a2 || _b1 == _b2 {
        return None;
    }
    let a1 = vec2(_a1.x as f32, _a1.y as f32);
    let a2 = vec2(_a2.x as f32, _a2.y as f32);
    let b1 = vec2(_b1.x as f32, _b1.y as f32);
    let b2 = vec2(_b2.x as f32, _b2.y as f32);

    let v1 = b1 - a1;
    let v2 = b2 - a2;
    if v2 == vec2(0.0, 0.0) {
        return None;
    }

    let v1_cross_v2 = v1.cross(v2);
    let a2_a1_cross_v1 = (a2 - a1).cross(v1);

    if v1_cross_v2 == 0.0 {
        if a2_a1_cross_v1 == 0.0 {

            let v1_sqr_len = v1.x*v1.x + v1.y*v1.y;
            // check if a2 is between a1 and b1
            let v1_dot_a2a1 = v1.dot(a2 - a1);
            if v1_dot_a2a1 > 0.0 && v1_dot_a2a1 < v1_sqr_len { return Some(int_vec2(a2.x as i32, a2.y as i32)); }

            // check if b2 is between a1 and b1
            let v1_dot_b2a1 = v1.dot(b2 - a1);
            if v1_dot_b2a1 > 0.0 && v1_dot_b2a1 < v1_sqr_len { return Some(int_vec2(b2.x as i32, b2.y as i32)); }

            let v2_sqr_len = v2.x*v2.x + v2.y*v2.y;
            // check if a1 is between a2 and b2
            let v2_dot_a1a2 = v2.dot(a1 - a2);
            if v2_dot_a1a2 > 0.0 && v2_dot_a1a2 < v2_sqr_len { return Some(int_vec2(a1.x as i32, a1.y as i32)); }

            // check if b1 is between a2 and b2
            let v2_dot_b1a2 = v2.dot(b1 - a2);
            if v2_dot_b1a2 > 0.0 && v2_dot_b1a2 < v2_sqr_len { return Some(int_vec2(b1.x as i32, b1.y as i32)); }

            return None;
        }

        return None;
    }

    let t = (a2 - a1).cross(v2) / v1_cross_v2;
    let u = a2_a1_cross_v1 / v1_cross_v2;

    if t > 0.0 && t < 1.0 && u > 0.0 && u < 1.0 {
        let res = a1 + (v1 * t);
        return Some(int_vec2(res.x as i32, res.y as i32));
    }

    return None;
}

#[test]
fn test_segment_intersection() {

    assert!(segment_intersection(
        vec2(0.0, -2.0), vec2(-5.0, 2.0),
        vec2(-5.0, 0.0), vec2(-11.0, 5.0)
    ).is_none());

    let i = segment_intersection(
        vec2(0.0, 0.0), vec2(1.0, 1.0),
        vec2(0.0, 1.0), vec2(1.0, 0.0)
    ).unwrap();
    println!(" intersection: {:?}", i);
    assert!(fuzzy_eq(i, vec2(0.5, 0.5)));

    assert!(segment_intersection(
        vec2(0.0, 0.0), vec2(0.0, 1.0),
        vec2(1.0, 0.0), vec2(1.0, 1.0)
    ).is_none());

    assert!(segment_intersection(
        vec2(0.0, 0.0), vec2(1.0, 0.0),
        vec2(2.0, 0.0), vec2(3.0, 0.0)
    ).is_none());

    assert!(segment_intersection(
        vec2(0.0, 0.0), vec2(2.0, 0.0),
        vec2(1.0, 0.0), vec2(3.0, 0.0)
    ).is_some());

    assert!(segment_intersection(
        vec2(3.0, 0.0), vec2(1.0, 0.0),
        vec2(2.0, 0.0), vec2(4.0, 0.0)
    ).is_some());

    assert!(segment_intersection(
        vec2(2.0, 0.0), vec2(4.0, 0.0),
        vec2(3.0, 0.0), vec2(1.0, 0.0)
    ).is_some());

    assert!(segment_intersection(
        vec2(1.0, 0.0), vec2(4.0, 0.0),
        vec2(2.0, 0.0), vec2(3.0, 0.0)
    ).is_some());

    assert!(segment_intersection(
        vec2(2.0, 0.0), vec2(3.0, 0.0),
        vec2(1.0, 0.0), vec2(4.0, 0.0)
    ).is_some());

    assert!(segment_intersection(
        vec2(0.0, 0.0), vec2(1.0, 0.0),
        vec2(0.0, 1.0), vec2(1.0, 1.0)
    ).is_none());
}

pub fn line_horizontal_intersection(
    a: Vec2,
    b: Vec2,
    y: f32
) -> f32 {
    let vx = b.x - a.x;
    let vy = b.y - a.y;
    if vy == 0.0 {
        // If the segment is horizontal, pick the biggest x value (the right-most point).
        // That's an arbitrary decision that serves the purpose of y-monotone decomposition
        return a.x.max(b.x);
    }
    return a.x + (y - a.y) * vx / vy;
}

pub fn line_horizontal_intersection_int(
    a: IntVec2,
    b: IntVec2,
    y: i32
) -> i32 {
    let vx = b.x - a.x;
    let vy = b.y - a.y;
    if vy == 0 {
        return cmp::max(a.x, b.x);
    }

    return (a.x as i64 + (y - a.y) as i64 * vx as i64 / vy as i64) as i32;
}


#[cfg(test)]
fn assert_almost_eq(a: f32, b:f32) {
    if (a - b).abs() < 0.0001 { return; }
    println!("expected {} and {} to be equal", a, b);
    panic!();
}

#[test]
fn test_intersect_segment_horizontal() {
    assert_almost_eq(line_horizontal_intersection(vec2(0.0, 0.0), vec2(0.0, 2.0), 1.0), 0.0);
    assert_almost_eq(line_horizontal_intersection(vec2(0.0, 2.0), vec2(2.0, 0.0), 1.0), 1.0);
    assert_almost_eq(line_horizontal_intersection(vec2(0.0, 1.0), vec2(3.0, 0.0), 0.0), 3.0);
}