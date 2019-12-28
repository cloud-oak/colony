extern crate rand;
extern crate sfml;

mod game;

use game::{Renderable, World, Peep};
use rand::{thread_rng, Rng};
use sfml::graphics::{Color, RenderTarget, RenderWindow, View};
use sfml::graphics::{Vertex, VertexArray, PrimitiveType};
use sfml::system::{Clock, Vector2f};
use sfml::window::{ContextSettings, Event, Key, Style};
use std::f32::consts::PI;

fn main() {
    let rng = thread_rng();

    let CAM_R = 18.0;

    // Define some constants
    let game_width = 800;
    let game_height = 600;

    // Create the window of the application
    let context_settings = ContextSettings {
        antialiasing_level: 8,
        ..Default::default()
    };
    let mut window = RenderWindow::new(
        (game_width, game_height),
        "Tiny Colony",
        Style::CLOSE,
        &context_settings,
    );
    let mut view = View::new(Vector2f::new(0.0, -CAM_R), Vector2f::new(40.0, 30.0));
    window.set_vertical_sync_enabled(true);

    window.set_view(&view);

    let world = World::new();
    let mut peeps : Vec<Peep> = vec![];

    for i in 0..18 {
        peeps.push(Peep::new(world.surface[10*i]));
    }

    let mut clock = Clock::start();

    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return,
                Event::KeyPressed { code: Key::A, .. } => {
                    view.rotate(5.0);
                    let theta = view.rotation().to_radians();
                    view.set_center((CAM_R * theta.sin(), -CAM_R * theta.cos()));
                    window.set_view(&view);
                },
                Event::KeyPressed { code: Key::D, .. } => {
                    view.rotate(-5.0);
                    let theta = view.rotation().to_radians();
                    view.set_center((CAM_R * theta.sin(), -CAM_R * theta.cos()));
                    window.set_view(&view);
                },
                _ => {}
            }
        }

        let delta_time = clock.restart().as_seconds();

        // Clear the window
        window.clear(Color::rgb(50, 50, 100));

        world.render(&mut window);
        for peep in peeps.iter() {
            peep.render(&mut window);
        }

        // Display things on screen
        window.display()
    }
}
