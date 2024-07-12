use super::vector::Vector;

#[derive(Clone, Copy)]
pub struct Particle{
    pub position: Vector,
    pub velocity: Vector,
    pub gravity: Option<Vector>,
    pub bounce: f32,
    pub radius: u32,
    pub mass: f32,
    pub grabbed: bool,
}
#[allow(dead_code)]
impl Particle{
    pub fn new(position: Vector, speed: f64, direction: f64, radius: u32, bounce: f32, mass: f32) -> Particle{
        let mut velocity = Vector::new(0.0, 0.0);
        velocity.set_length(speed);
        velocity.set_angle(direction);

        Particle{
            position,
            velocity,
            bounce,
            radius,
            mass,
            gravity: None,
            grabbed: false,
        }
    }
    pub fn new_with_gravity(position: Vector, speed: f64, direction: f64, radius: u32, bounce: f32, mass: f32,  gravity: Vector) -> Particle{
        let mut velocity = Vector::new(0.0, 0.0);
        velocity.set_length(speed);
        velocity.set_angle(direction);

        Particle{
            position,
            velocity,
            bounce,
            radius,
            mass,
            gravity: Some(gravity),
            grabbed: false,
        }
    }
    pub fn accelerate(&mut self, acceleration: Vector){
        self.velocity += acceleration
    }
    pub fn update(&mut self) {
        if !self.grabbed{
            self.position += self.velocity;
        }
        match self.gravity {
            Some(gravity) => {self.velocity += gravity}
            None => return
        }
        // if let Some(gravity) = self.gravity {
        //     self.velocity += gravity;
        // }
    }
}
