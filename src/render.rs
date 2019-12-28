mod world;
mod peep;

use sfml::graphics::*;

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

}
