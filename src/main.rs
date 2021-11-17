use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("repro", 800, 600)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();
    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    canvas.clear();
    canvas.set_viewport(Rect::new(0, 0, 800, 600)); // <--

    canvas.set_draw_color(Color {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    });
    let _ = canvas.fill_rect(Rect::new(0, 0, 100, 100));

    canvas.present();

    ::std::thread::sleep(Duration::new(0, 3_000_000_000_u32));
}
