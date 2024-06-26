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


    particles: Vec<Particle>,
}

impl Renderer {
    pub fn new(screen_area: Rect, clear_color: Color, particles: Vec<Particle>) -> Renderer {
        Renderer {
            screen_area,
            clear_color,

            particles,
        }
    }
    pub fn update(&mut self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.clear_color);
        canvas.fill_rect(self.screen_area).ok().unwrap_or_default(); // clear the screen


        canvas.set_draw_color(Color::RGB(200, 200, 200));

        for particle in self.particles.iter_mut() {
            particle.update();
            canvas.aa_circle(particle.position.get_x() as i16 , particle.position.get_y() as i16, 5, Color::RGB(200, 200, 200)).ok().unwrap_or_default();
        }
    }
    
    
}