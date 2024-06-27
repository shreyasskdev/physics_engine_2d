use std::f64::consts::PI;

use rand::Rng;
use sdl2::{
    event::Event, gfx::framerate::FPSManager, render::CanvasBuilder, Sdl
};


mod utils;
use utils::{particle::Particle, update_view::Renderer, vector::Vector};

fn main() -> Result<(), String> {
    let width :u32 = 1600;
    let height :u32 = 900;

    let sdl_context: Sdl = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("2d physics engine", width, height)
        .build()
        .unwrap();
    let mut canvas = CanvasBuilder::new(window)
        .accelerated()
        // .present_vsync()
        .build()
        .unwrap();

    let screen_area = sdl2::rect::Rect::new(0, 0, width, height);
    let clear_color = sdl2::pixels::Color::RGB(10, 10, 10, );
    let mut particles: Vec<Particle> = Vec::new();

    for _i in 0..2000{
        particles.push(Particle::new_with_gravity(Vector::new((width/2) as f64, (height/3) as f64),
        rand::thread_rng().gen_range(0.1..20.0),
    rand::thread_rng().gen_range(0.0..PI*2.0),
        10,
        -0.9,
      Vector::new(0.0, 0.1)
            ));
    }

    let mut update_view: Renderer = Renderer::new(screen_area, clear_color, (width, height), particles);


    let mut running = true;
    let mut event_queue = sdl_context.event_pump().unwrap();
    let mut fps_manager = FPSManager::new();
    fps_manager.set_framerate(60).unwrap();
    

    while running{

        for event in event_queue.poll_iter(){
            match event{

                Event::Quit {..} => {
                    running = false;
                }
                _ => {}
            }
        }

        update_view.update(&mut canvas);

        canvas.present();
        fps_manager.delay();

    }

    Ok(())
}