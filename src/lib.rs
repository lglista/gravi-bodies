mod body;

use sfml::system::Vector2f;
use body::BodyType;

static G: f32 = 6.674 * 0.000000000001; // 6.6764E-11

pub fn length_of_vector(vec: Vector2f) -> f32 {
    let x_2: f32 = vec.x * vec.x;
    let y_2: f32 = vec.y * vec.y;
    f32::sqrt(x_2 + y_2)
}

pub fn normalize(vec: Vector2f, length: f32) -> Vector2f {
    Vector2f::new(vec.x / length, vec.y / length)
}

pub fn calculate_force_of_gravity(m1: f32, m2: f32, r: f32) -> f32 {
    let numerator: f32 = m1 * m2;
    let denominator: f32 = r.powf(2_f32);
    G * (numerator / denominator)
}

pub fn body_type_multiplier(a: BodyType, b: BodyType) -> i32 {
    if a == BodyType::WEIRD || b == BodyType::WEIRD {
        return -1;
    }
    else if a == BodyType::NEUTRAL || b == BodyType::NEUTRAL {
        return 1;
    }
    else if a == b {
        return -1;
    }
    else {
        return 1;
    }
}


/*
 * ###############################################################################
 * ################################# TESTS #######################################
 * ###############################################################################
 */
#[test]
fn length_of_vector_basic() {
    let vec = Vector2f::new(0_f32, 1_f32);
    assert_eq!(length_of_vector(vec), 1_f32);
}

#[test]
fn length_of_vector_perfect_square() {
    let vec_1 = Vector2f::new(3_f32, 4_f32);
    assert_eq!(length_of_vector(vec_1), 5_f32);

    let vec_2 = Vector2f::new(5_f32, 12_f32);
    assert_eq!(length_of_vector(vec_2), 13_f32);
}

#[test]
fn normalize_basic() {
    let vec_1 = Vector2f::new(0_f32, 1_f32);
    let normalized_1 = normalize(vec_1, 1_f32);
    assert_eq!(normalized_1.x, 0_f32);
    assert_eq!(normalized_1.y, 1_f32);

    let vec_2 = Vector2f::new(0_f32, 120_f32);
    let normalized_2 = normalize(vec_2, 120_f32);
    assert_eq!(normalized_2.x, 0_f32);
    assert_eq!(normalized_2.y, 1_f32);
}

#[test]
fn normalize_square() {
    let vec = Vector2f::new(3_f32, 4_f32);
    let normalized = normalize(vec, 5_f32);
    assert_eq!(normalized.x, 0.6_f32);
    assert_eq!(normalized.y, 0.8_f32);
}

#[test]
fn normalize_negative() {
    let vec = Vector2f::new(-3_f32, 4_f32);
    let normalized = normalize(vec, 5_f32);
    assert_eq!(normalized.x, -0.6_f32);
    assert_eq!(normalized.y, 0.8_f32);
}

#[test]
fn integrate_normalize_with_length_basic() {
    let vec = Vector2f::new(-3_f32, 4_f32);
    let normalized = normalize(vec, length_of_vector(vec));
    assert_eq!(normalized.x, -0.6_f32);
    assert_eq!(normalized.y, 0.8_f32);
}

#[test]
fn body_type_multiplier_positive() {
    assert_eq!(body_type_multiplier(BodyType::POSITIVE, BodyType::POSITIVE), -1);
    assert_eq!(body_type_multiplier(BodyType::POSITIVE, BodyType::NEGATIVE), 1);
    assert_eq!(body_type_multiplier(BodyType::POSITIVE, BodyType::NEUTRAL), 1);
    assert_eq!(body_type_multiplier(BodyType::POSITIVE, BodyType::WEIRD), -1);
}

#[test]
fn body_type_multiplier_negative() {
    assert_eq!(body_type_multiplier(BodyType::NEGATIVE, BodyType::POSITIVE), 1);
    assert_eq!(body_type_multiplier(BodyType::NEGATIVE, BodyType::NEGATIVE), -1);
    assert_eq!(body_type_multiplier(BodyType::NEGATIVE, BodyType::NEUTRAL), 1);
    assert_eq!(body_type_multiplier(BodyType::NEGATIVE, BodyType::WEIRD), -1);
}


#[test]
fn body_type_multiplier_neutral() {
    assert_eq!(body_type_multiplier(BodyType::NEUTRAL, BodyType::POSITIVE), 1);
    assert_eq!(body_type_multiplier(BodyType::NEUTRAL, BodyType::NEGATIVE), 1);
    assert_eq!(body_type_multiplier(BodyType::NEUTRAL, BodyType::NEUTRAL), 1);
    assert_eq!(body_type_multiplier(BodyType::NEUTRAL, BodyType::WEIRD), -1);
}

#[test]
fn body_type_multiplier_weird() {
    assert_eq!(body_type_multiplier(BodyType::WEIRD, BodyType::POSITIVE), -1);
    assert_eq!(body_type_multiplier(BodyType::WEIRD, BodyType::NEGATIVE), -1);
    assert_eq!(body_type_multiplier(BodyType::WEIRD, BodyType::NEUTRAL), -1);
    assert_eq!(body_type_multiplier(BodyType::WEIRD, BodyType::WEIRD), -1);
}
