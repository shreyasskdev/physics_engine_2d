use super::vector::Vector;

pub struct Particle{
    pub position: Vector,
    pub velocity: Vector,
    pub gravity: Option<Vector>,
    // pub mass: f32
}impl Particle{
    pub fn new(position: Vector, speed: f64, direction: f64) -> Particle{
        let mut velocity = Vector::new(0.0, 0.0);
        velocity.set_length(speed);
        velocity.set_angle(direction);

        Particle{
            position,
            velocity,
            gravity: None,
        }
    }
    pub fn new_with_gravity(position: Vector, speed: f64, direction: f64, gravity: Vector) -> Particle{
        let mut velocity = Vector::new(0.0, 0.0);
        velocity.set_length(speed);
        velocity.set_angle(direction);

        Particle{
            position,
            velocity,
            gravity: Some(gravity),
        }
    }
    pub fn accelerate(&mut self, acceleration: Vector){
        self.velocity += acceleration
    }
    pub fn update(&mut self) {
        self.position += self.velocity;
        // self.velocity += self.gravity
        match self.gravity {
            Some(gravity) => {self.velocity += gravity}
            None => return
        }
        // if let Some(gravity) = self.gravity {
        //     self.velocity += gravity;
        // }
    }
}