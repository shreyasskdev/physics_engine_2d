use super::vector::Vector;

pub struct Particle{
    pub position: Vector,
    pub velocity: Vector,
    // pub mass: f32
}
impl Particle{
    pub fn new(position: Vector, speed: f64, direction: f64) -> Particle{
        let mut velocity = Vector::new(0.0, 0.0);
        velocity.set_length(speed);
        velocity.set_angle(direction);
        Particle{
            position,
            velocity,
        }
    }
    pub fn update(&mut self) {
        self.position += self.velocity;
    }
}