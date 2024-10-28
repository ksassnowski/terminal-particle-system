use std::f64::consts::PI;

use rand::Rng;

use crate::{term::Screen, vector::Vector2};

#[derive(Clone, Copy, Debug)]
pub struct GradientEntry<T> {
    t: f64,
    value: T,
}

impl<T> GradientEntry<T> {
    fn new(t: f64, value: T) -> GradientEntry<T> {
        GradientEntry { t, value }
    }
}

#[derive(Debug)]
pub struct Gradient<T> {
    values: Vec<GradientEntry<T>>,
}

impl<T> Gradient<T> {
    pub fn new(mut values: Vec<GradientEntry<T>>) -> Gradient<T> {
        values.sort_by(|a, b| a.t.total_cmp(&b.t));

        Gradient { values }
    }

    pub fn with_equal_spacing(values: Vec<T>) -> Gradient<T> {
        let mut entries = Vec::with_capacity(values.len());
        let step = 1.0 / values.len() as f64;

        for (i, v) in values.into_iter().enumerate() {
            entries.push(GradientEntry::new(step * (i + 1) as f64, v));
        }

        Gradient::new(entries)
    }

    pub fn get_value(&self, t: f64) -> &T {
        for entry in &self.values {
            if t <= entry.t {
                return &entry.value;
            }
        }
        return &self.values.last().unwrap().value;
    }
}

#[derive(Debug)]
pub enum SpawnStrategy {
    Circle(Vector2, f64),
    Point(Vector2),
    Box(Vector2, Vector2),
}

#[derive(Clone, Copy, Debug)]
struct Particle {
    time_left: f64,
    lifetime: f64,
    velocity: Vector2,
    position: Vector2,
    force: Vector2,
}

impl Particle {
    fn new(ttl: f64, initial_velocity: Vector2, initial_position: Vector2) -> Particle {
        Particle {
            time_left: ttl,
            lifetime: ttl,
            velocity: initial_velocity,
            position: initial_position,
            force: Vector2::zero(),
        }
    }

    fn apply_force(&mut self, force: Vector2) {
        self.force += force;
    }

    fn tick(&mut self, dt: f64) {
        self.time_left -= dt;
        self.velocity += self.force * dt;
        self.position += self.velocity * dt;

        self.force.x = 0.0;
        self.force.y = 0.0;
    }
}

#[derive(Debug)]
pub struct ParticleSystem<'a> {
    particles: Vec<Particle>,
    gravity: Vector2,
    particle_lifetime: f64,
    number_of_particles: usize,
    initial_velocity: Vector2,
    spawn_strategy: SpawnStrategy,
    color_map: &'a Gradient<&'a str>,
    char_map: &'a Gradient<char>,
}

impl<'a> ParticleSystem<'a> {
    pub fn new(
        particle_lifetime: f64,
        number_of_particles: usize,
        gravity: Vector2,
        initial_velocity: Vector2,
        spawn_strategy: SpawnStrategy,
        color_map: &'a Gradient<&'a str>,
        char_map: &'a Gradient<char>,
    ) -> ParticleSystem<'a> {
        let mut system = ParticleSystem {
            particles: Vec::with_capacity(number_of_particles),
            gravity,
            particle_lifetime,
            number_of_particles,
            initial_velocity,
            spawn_strategy,
            color_map,
            char_map,
        };

        for _ in 0..number_of_particles {
            system.spawn_particle();
        }

        system
    }

    pub fn tick(&mut self, dt: f64) {
        for p in &mut self.particles {
            p.apply_force(self.gravity);
            p.tick(dt);
        }
        // TODO: Is there a way of doing this in one loop instead of two?
        self.particles.retain(|&p| p.time_left > 0.0);

        let particles_to_spawn = self.number_of_particles - self.particles.len();
        for _ in 0..particles_to_spawn {
            self.spawn_particle();
        }
    }

    pub fn draw(&self, screen: &mut Screen) {
        for p in &self.particles {
            let time_left = p.time_left / p.lifetime;
            let char = self.char_map.get_value(time_left);
            let color = self.color_map.get_value(time_left);

            screen.write_char(
                *char,
                color.to_string(),
                p.position.y.round() as i32,
                p.position.x.round() as i32,
            );
        }
    }

    pub fn spawn_particle(&mut self) {
        let mut rng = rand::thread_rng();
        let ttl = self.particle_lifetime * rng.gen_range(0.1..1.9);
        let initial_velocity = self.initial_velocity * rng.gen_range(0.1..1.9);

        let initial_position = match self.spawn_strategy {
            SpawnStrategy::Box(position, dimensions) => Vector2::new(
                rng.gen_range(position.x..=(position.x + dimensions.x)),
                rng.gen_range(position.y..=(position.y + dimensions.y)),
            ),
            SpawnStrategy::Circle(position, radius) => {
                let angle = rng.gen_range(0.0..(2.0 * PI));
                let magnitude = rng.gen_range(0.0..=radius);
                let offset = Vector2::from_polar(angle, magnitude);
                position + offset
            }
            SpawnStrategy::Point(position) => position,
        };

        let particle = Particle::new(ttl, initial_velocity, initial_position);

        self.particles.push(particle);
    }
}
