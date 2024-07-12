
use sdl2::{
    pixels::Color,
    rect::Rect,
    render::Canvas,
    video::Window,
    gfx::primitives::DrawRenderer,
};

use super::particle::Particle;
use super::utils::Utils;
use super::vector::Vector;

pub struct World {
    pub screen_area: Rect,
    pub clear_color: Color,

    pub screen_dimension: (u32, u32),

    friction: f32,
    particles: Vec<Particle>,

    // particle_index, particle gravity
    particle_tmp: (Option<usize>, Option<Vector>),
    vel_tmp: (Vector, Vector)
}

impl World {
    pub fn new(screen_area: Rect, clear_color: Color, screen_dimension: (u32, u32), particles: Vec<Particle>) -> World {
        World {
            screen_area,
            clear_color,
            screen_dimension,

            friction: 0.9,
            particles,

            particle_tmp: (None, None),
            vel_tmp: (Vector::new(0.0, 0.0), Vector::new(0.0, 0.0)),
        }
    }
    pub fn update(&mut self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.clear_color);
        canvas.fill_rect(self.screen_area).ok().unwrap_or_default(); // clear the screen


        canvas.set_draw_color(Color::RGB(200, 200, 200));


        for particle in self.particles.iter_mut() {
            particle.update();
            canvas.aa_circle(particle.position.get_x() as i16 , particle.position.get_y() as i16, particle.radius as i16, Color::RGB(200, 200, 200)).ok().unwrap_or_default();
            canvas.aa_circle(particle.position.get_x() as i16 , particle.position.get_y() as i16, particle.radius as i16, Color::RGB(200, 200, 200)).ok().unwrap_or_default();

            //edge collision detection, edge bouncing, and friction 
            if (particle.position.get_x() - particle.radius as f64) <= 0.0 {
                particle.position.set_x(particle.radius as f64);
                particle.velocity.set_x(particle.velocity.get_x()* particle.bounce as f64);
                particle.velocity *= self.friction as f64;
            }
            if (particle.position.get_x() + particle.radius as f64) >= self.screen_dimension.0 as f64 {
                particle.position.set_x((self.screen_dimension.0 - particle.radius) as f64);
                particle.velocity.set_x(particle.velocity.get_x()* particle.bounce as f64);
                particle.velocity *= self.friction as f64;
            }
            if (particle.position.get_y() - particle.radius as f64) <= 0.0 {
                particle.position.set_y(particle.radius as f64);
                particle.velocity.set_y(particle.velocity.get_y()* particle.bounce as f64);
                particle.velocity *= self.friction as f64;
            }
            if (particle.position.get_y() + particle.radius as f64) >= self.screen_dimension.1 as f64 {
                particle.position.set_y((self.screen_dimension.1 - particle.radius) as f64);
                particle.velocity.set_y(particle.velocity.get_y()* particle.bounce as f64);
                particle.velocity *= self.friction as f64;
            }
        }

        // // Penetration resolution sampling (uncomment this block for better peneration resolution)
        // for _ in 0..16 {
        //     for i in 0..self.particles.len(){
        //         for j in (i+1)..self.particles.len(){
        //             if Utils::circle_collide(self.particles[i], self.particles[j]){
        //                 penetration_resolution(& mut self.particles, i, j);
        //             }
        //         }
        //     }
        // }

        // Collision dectection between particles
        for i in 0..self.particles.len(){
            for j in (i+1)..self.particles.len(){
                if Utils::circle_collide(self.particles[i], self.particles[j]){
                    collision_response(&mut self.particles, i, j);
                }
            }
        }
    }
    
    pub fn handle_click(&mut self, x: i32, y: i32,  mouse_pressed: bool) {
        for i in 0..self.particles.len(){
            if Utils::circle_collide(self.particles[i], Particle::new(Vector::new(x as f64, y as f64), 0.0, 0.0, 1, 0.0, 0.0)){
                    
                    if mouse_pressed && self.particle_tmp.1.is_none(){
                        self.particle_tmp.1 = self.particles[i].gravity;
                    }
                    
                    self.particles[i].velocity = Vector::new(0.0, 0.0);
                    self.particles[i].position = Vector::new(x as f64, y as f64);
                    if mouse_pressed {
                        self.particles[i].gravity = Some(Vector::new(0.0, 0.0));
                        self.particle_tmp.0 = Some(i);

                        self.vel_tmp.0 = self.vel_tmp.1;
                        self.vel_tmp.1 = Vector::new(x as f64, y as f64);
                        
                    } else {
                        self.particles[i].gravity = self.particle_tmp.1;
                        self.particle_tmp.1 = None;
                        self.particle_tmp.0 = None;

                        self.particles[i].velocity = Vector::new(x as f64, y as f64) - self.vel_tmp.0;
                        self.particles[i].velocity.set_length(
                            Utils::distance_xy(self.vel_tmp.0.get_x(), self.vel_tmp.0.get_y(), x as f64, y as f64)
                        );
                        
                    }
            }

            if self.particle_tmp.0.is_some(){
                self.particles[self.particle_tmp.0.unwrap()].position = Vector::new(x as f64, y as f64);
            }
        }
    }    
}

fn collision_response(particles: &mut Vec<Particle>, i:usize, j:usize ){

    penetration_resolution(particles, i, j);
 
    let normal = (particles[i].position - particles[j].position).unit();
    let relative_velocity = particles[i].velocity - particles[j].velocity;
    let seperating_velocity = Vector::dot(relative_velocity, normal);

    let restitution = -particles[i].bounce as f64;

    let new_seperating_velocity = -seperating_velocity;
    let seperating_velocity_vector = normal * new_seperating_velocity * restitution;

    particles[i].velocity += seperating_velocity_vector;
    particles[j].velocity -= seperating_velocity_vector;
}

fn penetration_resolution(particles: &mut Vec<Particle>, i:usize, j:usize){

    let distance = particles[i].position - particles[j].position;
    let penetration_depth = (particles[i].radius + particles[j].radius) as f64 - distance.get_length();
    let penetration_resolution = distance.unit() * (penetration_depth / 2.0);
    particles[i].position += penetration_resolution;
    particles[j].position -= penetration_resolution;
}
