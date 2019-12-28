use sfml::system::{Vector2f};

pub struct Peep {
    pub position: Vector2f
}

impl Peep {
    pub fn new(position: Vector2f) -> Self {
        return Peep {position: position};
    }
}
