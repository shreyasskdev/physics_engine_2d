use std::{f64::consts::PI, time::{Duration, Instant}};

use rand::Rng;
use sdl2::{
    event::Event, gfx::framerate::FPSManager,
    pixels::Color,
    rect::Rect, 
    render::{
        CanvasBuilder, 
        TextureQuery
    }, 
    ttf, 
    Sdl
};

mod elements;
use elements::{particle::Particle, universe::World, vector::Vector};

fn main() -> Result<(), String> {
    let width :u32 = 1600;
    let height :u32 = 900;

    let sdl_context: Sdl = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = ttf::init().map_err(|e| e.to_string())?;
    let window = video_subsystem.window("2d physics engine", width, height)
        .opengl()
        .build()
        .unwrap();
    let mut canvas = CanvasBuilder::new(window)
        .accelerated()
        // .present_vsync()
        .build()
        .unwrap();
    let texture_creator = canvas.texture_creator();

    let screen_area = sdl2::rect::Rect::new(0, 0, width, height);
    let clear_color = sdl2::pixels::Color::RGB(10, 10, 10, );
    let mut particles: Vec<Particle> = Vec::new();

    for _i in 0..100{
        particles.push(Particle::new_with_gravity(Vector::new(rand::thread_rng().gen_range(0.1..width as f64), rand::thread_rng().gen_range(0.1..height as f64)),
        rand::thread_rng().gen_range(0.1..50.0),
    rand::thread_rng().gen_range(0.0..PI*2.0),
        20,
        -0.9,
        10.0,
      Vector::new(0.0, 0.1)
            ));
    }

    let mut system: World = World::new(screen_area, clear_color, (width, height), particles);


    let mut running = true;
    let mut event_queue = sdl_context.event_pump().unwrap();
    let mut fps_manager = FPSManager::new();
    fps_manager.set_framerate(60).unwrap();


    let font = ttf_context.load_font("src/Roboto/Roboto-Regular.ttf", 24)?;   
    
    // For FPS calculation
    let mut frame_count = 0;
    let mut last_time = Instant::now();
    let mut fps = 60.00;

    let mut mouse_pressed: bool = false;

    while running{

        for event in event_queue.poll_iter(){
            match event{

                Event::Quit {..} => {
                    running = false;
                }
                Event::MouseButtonDown { x, y, .. } => {
                    mouse_pressed = true;
                    system.handle_click(x, y, mouse_pressed);
                }
                Event::MouseButtonUp { x, y, .. } => {
                    mouse_pressed = false;
                    system.handle_click(x, y, mouse_pressed);
                }
                Event::MouseMotion { x, y, .. } => {
                    if mouse_pressed {
                        system.handle_click(x, y, mouse_pressed);
                    }              
                }
                
                _ => {}
            }
        }

        system.update(&mut canvas);

        // get fps
        frame_count += 1;
        let current_time = Instant::now();
        let elapsed = current_time.duration_since(last_time);

        if elapsed >= Duration::from_secs(1) {
            fps = frame_count as f64 / elapsed.as_secs_f64();
            frame_count = 0;
            last_time = current_time;
        }

        // render font
        let surface = font.render(&format!("FPS: {:.2}", fps)).blended(Color::RGBA(255, 255, 255, 255)).map_err(|e| e.to_string())?;
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
        let TextureQuery { width, height, .. } = texture.query();
        canvas.copy(&texture, None, Some(Rect::new(0, 0, width, height)))?;

        canvas.present();
        fps_manager.delay();

    }

    Ok(())
}