use sfml::system::Vector2f;

// The laws of physics dictate that for every action there is an opposite and equal reaction.
// i.e. If one body type pushes, the other has to push as well
// This makes it so that there can only be 1 universal body type meaning that only 1 at a time can have absolute reign
// Here it is chosen to be WEIRD. This makes it so that even NEUTRAL is repelled by WEIRD
#[derive(PartialEq, Eq)]
pub enum BodyType {
    POSITIVE,
    NEGATIVE,
    NEUTRAL, // Attracted to everything except WEIRD
    WEIRD // Repelled by everything
}

pub struct Body {
    pub velocity: Vector2f,
    pub position: Vector2f,
    pub mass: f32,
    pub body_type: BodyType
}

impl Body {
    pub fn new(starting_velocity: Vector2f,
                starting_position: Vector2f,
                starting_mass: f32,
                starting_body_type: BodyType) -> Self {
        Body{velocity: starting_velocity, position: starting_position, mass: starting_mass, body_type: starting_body_type}
    }
}
