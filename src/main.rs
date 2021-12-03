use sdl2::rect::Rect;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("repro", 800, 600)
        .build()
        .map_err(|e| e.to_string())
        .unwrap();
    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let before = canvas.viewport();
    canvas.set_viewport(Rect::new(0, 0, 800, 600)); // <--
    let after = canvas.viewport();

    assert_eq!(before, after);
}
