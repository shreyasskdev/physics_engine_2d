use sdl2::{
    Sdl,
    event::Event,
    gfx::framerate::FPSManager
};


mod utils;
use utils::update_view::Renderer;

fn main() -> Result<(), String> {
    let width :u32 = 800;
    let height :u32 = 600;

    let sdl_context: Sdl = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("2d physics engine", width, height)
        .build()
        .unwrap();
    let mut canvas = window.into_canvas()
        .build()
        .unwrap();

    let screen_area = sdl2::rect::Rect::new(0, 0, width, height);
    let clear_color = sdl2::pixels::Color::RGB(10, 10, 10, );
    let x: f64 = 200.0;
    let y: f64 = 10.0;


    let mut update_view: Renderer = Renderer::new(x, y, screen_area, clear_color);


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