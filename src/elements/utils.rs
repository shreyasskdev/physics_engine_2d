use libm::sqrt;
use super::particle::Particle;

pub struct Utils{}
#[allow(dead_code)]
impl Utils{
    pub fn distance(v1: Particle, v2: Particle) -> f64{
        let dx = v1.position.get_x() - v2.position.get_x();
        let dy = v1.position.get_y() - v2.position.get_y();

        sqrt(dx * dx + dy* dy)
    }
    pub fn distance_xy(x1: f64, y1: f64, x2: f64, y2: f64) -> f64{
        let dx = x1 - x2;
        let dy = y1 - y2;

        sqrt(dx * dx + dy* dy)
    }

    pub fn circle_collide(c1: Particle, c2: Particle) -> bool {
        Utils::distance(c1, c2) <= (c1.radius + c2.radius) as f64 
    }
}