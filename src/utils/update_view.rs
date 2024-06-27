use sdl2::{
    pixels::Color,
    rect::Rect,
    render::Canvas,
    video::Window,
    gfx::primitives::DrawRenderer,
};

use super::particle::Particle;

pub struct Renderer {
    pub screen_area: Rect,
    pub clear_color: Color,

    pub screen_dimension: (u32, u32),

    particles: Vec<Particle>,
}

impl Renderer {
    pub fn new(screen_area: Rect, clear_color: Color, screen_dimension: (u32, u32), particles: Vec<Particle>) -> Renderer {
        Renderer {
            screen_area,
            clear_color,
            screen_dimension,

            particles,
        }
    }
    pub fn update(&mut self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.clear_color);
        canvas.fill_rect(self.screen_area).ok().unwrap_or_default(); // clear the screen


        canvas.set_draw_color(Color::RGB(200, 200, 200));

        for particle in self.particles.iter_mut() {
            particle.update();
            canvas.aa_circle(particle.position.get_x() as i16 , particle.position.get_y() as i16, particle.radius as i16, Color::RGB(200, 200, 200)).ok().unwrap_or_default();

            //edge detection and edge bouncing
            if (particle.position.get_x() - particle.radius as f64) <= 0.0 {
                particle.position.set_x(particle.radius as f64);
                particle.velocity.set_x(particle.velocity.get_x()* particle.bounce as f64);
            }
            if (particle.position.get_x() + particle.radius as f64) >= self.screen_dimension.0 as f64 {
                particle.position.set_x((self.screen_dimension.0 - particle.radius) as f64);
                particle.velocity.set_x(particle.velocity.get_x()* particle.bounce as f64);
            }
            if (particle.position.get_y() - particle.radius as f64) <= 0.0 {
                particle.position.set_y(particle.radius as f64);
                particle.velocity.set_y(particle.velocity.get_y()* particle.bounce as f64);
            }
            if (particle.position.get_y() + particle.radius as f64) >= self.screen_dimension.1 as f64 {
                particle.position.set_y((self.screen_dimension.1 - particle.radius) as f64);
                particle.velocity.set_y(particle.velocity.get_y()* particle.bounce as f64);
            }
        }
    }
    
    
}