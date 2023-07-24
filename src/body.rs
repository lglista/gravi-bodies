use sfml::system::Vector2f;
use sfml::graphics::{CircleShape, Color, Transformable, Shape};
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

static G: f32 = 6.674 * 0.001; // 6.6764E-3 (6.6764E-11 is normal, will be added to config file)
static CIRCLE_POINT_COUNT: usize = 10;

// The laws of physics dictate that for every action there is an opposite and equal reaction.
// i.e. If one body type pushes, the other has to push as well
// This makes it so that there can only be 1 universal body type meaning that only 1 at a time can have absolute reign
// It is chosen in lib::body_type_multiplier() to be WEIRD. This makes it so that even NEUTRAL is repelled by WEIRD
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum BodyType {
    POSITIVE,
    NEGATIVE,
    NEUTRAL, // Attracted to everything except WEIRD
    WEIRD // Repelled by everything
}

pub struct Body {
    pub velocity: Vector2f,
    pub mass: f32,
    pub body_type: BodyType
}

impl Body {
    pub fn new(starting_velocity: Vector2f, starting_mass: f32,starting_body_type: BodyType) -> Self {
        Body{velocity: starting_velocity, mass: starting_mass, body_type: starting_body_type}
    }
}

impl FromStr for BodyType {

    type Err = ();

    fn from_str(input: &str) -> Result<BodyType, Self::Err> {
        match input {
            "NEUTRAL"=>Ok(BodyType::NEUTRAL),
            "POSITIVE"=>Ok(BodyType::POSITIVE),
            "NEGATIVE"=>Ok(BodyType::NEGATIVE),
            "WEIRD"=>Ok(BodyType::WEIRD),
            _=>Err(()),
        }
    }
}

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

pub fn read_file_and_make_circles(filename: &str) -> Vec<CircleShape> {
    let mut circle_vec = Vec::new();
    let open_file = File::open(filename);
    let mut contents = String::new();
    let _ = open_file.ok().expect("Circle File not found").read_to_string(&mut contents);
    let mut lines = contents.split('\n');
    let mut line = lines.next();
    while line != None {
        let mut values = line.expect("Lines not found in file").split_whitespace();
        let first_value = values.next();
        if first_value == Some("#") {
            line = lines.next();
            continue;
        }
        let radius = first_value.expect("Radius not found").parse().unwrap();
        let x: f32 = values.next().expect("X not found").parse().unwrap();
        let y: f32 = values.next().expect("Y not found").parse().unwrap();
        let r: u8 = values.next().expect("R not found").parse().unwrap();
        let g: u8 = values.next().expect("G not found").parse().unwrap();
        let b: u8 = values.next().expect("B not found").parse().unwrap();
        let mut circle = CircleShape::new(radius, CIRCLE_POINT_COUNT);
        circle.set_position(Vector2f::new(x, y));
        circle.set_fill_color(Color::rgb(r, g, b));
        circle_vec.push(circle);
        line = lines.next();
    }
    circle_vec
}

pub fn read_file_and_make_bodies(filename: &str) -> Vec<Body> {
    let mut body_vec = Vec::new();
    let open_file = File::open(filename);
    let mut contents = String::new();
    let _ = open_file.ok().expect("Circle File not found").read_to_string(&mut contents);
    let mut lines = contents.split('\n');
    let mut line = lines.next();
    while line != None {
        let mut values = line.expect("Lines not found in file").split_whitespace();
        let first_value = values.next();
        if first_value == Some("#") {
            line = lines.next();
            continue;
        }
        let body_type: BodyType = BodyType::from_str(first_value.expect("BodyType not found")).unwrap();
        let mass: f32 = values.next().expect("Mass not found").parse().unwrap();
        let x_velocity: f32 = values.next().expect("X Velocity not found").parse().unwrap();
        let y_velocity: f32 = values.next().expect("Y Velocity not found").parse().unwrap();
        let body = Body::new(Vector2f::new(x_velocity, y_velocity), mass, body_type);
        body_vec.push(body);
        line = lines.next();
    }
    body_vec
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
