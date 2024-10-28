use std::{io, thread, time::Duration};

use particle::{Gradient, ParticleSystem, SpawnStrategy};
use term::Screen;
use vector::Vector2;

mod particle;
mod term;
mod vector;

const SCREEN_DIMENSIONS: Vector2 = Vector2 { x: 100.0, y: 31.0 };

fn main() {
    let stdout = io::stdout();
    let mut screen = Screen::new(
        stdout,
        SCREEN_DIMENSIONS.x as usize,
        SCREEN_DIMENSIONS.y as usize,
    );
    let char_map = Gradient::with_equal_spacing(vec!['.', '\'', '°', '*', '~', '#']);
    let color_map = Gradient::with_equal_spacing(vec![
        "18", "20", "23", "27", "29", "30", "31", "33", "37", "44", "67", "72", "84", "86",
    ]);
    let mut particle_systems: Vec<ParticleSystem> = vec![ParticleSystem::new(
        0.35,
        400,
        Vector2::new(0.0, -40.0),
        Vector2::new(0.0, -6.0),
        SpawnStrategy::Box(
            Vector2::new(0.0, SCREEN_DIMENSIONS.y - 1.0),
            Vector2::new(SCREEN_DIMENSIONS.x, 1.0),
        ),
        &color_map,
        &char_map,
    )];

    let dt = 1.0 / 30.0;

    loop {
        screen.clear();

        for ps in &mut particle_systems {
            ps.tick(dt);
            ps.draw(&mut screen);
        }

        screen.flush();

        thread::sleep(Duration::from_secs_f64(dt));
    }
}
