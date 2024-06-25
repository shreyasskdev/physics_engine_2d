use sdl2::{
    pixels::Color,
    rect::Rect,
    render::Canvas,
    video::Window,
    gfx::primitives::DrawRenderer,
};

use super::vector::Vector;

pub struct Renderer {
    pub screen_area: Rect,
    pub clear_color: Color,

    pub x: f64,
    pub y: f64,

    position: Vector
}

impl Renderer {
    pub fn new(x: f64, y: f64, screen_area: Rect, clear_color: Color) -> Renderer {
        Renderer {
            screen_area,
            clear_color,
            x,
            y,

            position: Vector::new(100.0, 100.0),
        }
    }
    pub fn update(&mut self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.clear_color);
        canvas.fill_rect(self.screen_area).ok().unwrap_or_default(); // clear the screen


        canvas.set_draw_color(Color::RGB(200, 200, 200));

        canvas.aa_circle(self.x as i16 , self.y as i16, 50, Color::RGB(200, 200, 200)).ok().unwrap_or_default();

        

        self.y = self.y + 1.0;
    }
    
    
}