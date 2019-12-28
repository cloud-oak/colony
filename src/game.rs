pub mod world;
pub mod peep;

pub use world::World;
pub use peep::Peep;

use rand::{thread_rng, Rng};
use sfml::graphics::*;
use sfml::system::Vector2f;

pub trait Renderable {
    fn render(&self, window: &mut RenderWindow);
}

impl Renderable for World {
    fn render(&self, window: &mut RenderWindow) {
        let mut shape = ConvexShape::new(self.surface.len() as u32);
        for (i, x) in self.surface.iter().enumerate() {
            shape.set_point(i as u32, x.clone());
        }
        shape.set_fill_color(Color::GREEN);
        window.draw(&shape);
    }
}

impl Renderable for Peep {
    fn render(&self, window: &mut RenderWindow) {
        let theta = -self.position.x.atan2(self.position.y);

        let mut transform = Transform::default();
        transform.rotate(theta.to_degrees());
        transform.translate(-self.position.x, -self.position.y);

        let mut head = CircleShape::new(0.5, 20);
        head.set_outline_color(Color::BLACK);
        head.set_fill_color(Color::WHITE);
        head.set_outline_thickness(0.1);

        head.set_origin((0.5, -0.5));
        head.rotate(theta.to_degrees());
        head.move_(self.position.clone());

        let mut body = RectangleShape::with_size(Vector2f::new(0.5, 1.0));
        body.set_outline_color(Color::BLACK);
        body.set_fill_color(Color::WHITE);
        body.set_outline_thickness(0.1);

        body.set_origin((0.25, 0.0));
        body.rotate(theta.to_degrees());
        body.move_(self.position.clone());

        let mut rand = rand::thread_rng();

        window.draw(&body);
        window.draw(&head);
    }
}
