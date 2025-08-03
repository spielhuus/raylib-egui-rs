use egui_demo_lib::DemoWindows;
use raylib_egui_rs::egui::EguiRaylib;
use raylib_egui_rs::raylib;

const SCREEN_WIDTH: usize = 1280;
const SCREEN_HEIGHT: usize = 720;
const TITLE: &str = "Template";

fn main() {
    env_logger::init();

    let mut egui_raylib = EguiRaylib::new();
    let mut demo = DemoWindows::default();

    // initialize raylib
    raylib::InitWindow(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, TITLE);
    raylib::SetTargetFPS(240);

    // Main game loop
    #[cfg(not(target_arch = "wasm32"))]
    {
        while !raylib::WindowShouldClose() {
            use raylib_egui_rs::color::Color;

            raylib::BeginDrawing();
            raylib::ClearBackground(Color::WHITE);

            egui_raylib.run(|egui_ctx| {
                demo.ui(&egui_ctx);
            });

            egui_raylib.paint();
            raylib::EndDrawing();
        }
        raylib::CloseWindow();
    }
    #[cfg(target_arch = "wasm32")]
    {
        let boxed_state = Box::new(game_state);
        let state_ptr = Box::into_raw(boxed_state) as *mut c_void;
        emscripten_set_main_loop_arg(main_loop_wrapper, state_ptr, 0, 1);
    }
}
