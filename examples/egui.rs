use raylib_egui_rs::egui::EguiRaylib;
use raylib_egui_rs::raylib;

const SCREEN_WIDTH: usize = 1280;
const SCREEN_HEIGHT: usize = 720;
const TITLE: &str = "Template";

fn main() {
    env_logger::init();
    raylib::InitWindow(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, TITLE);
    raylib::SetTargetFPS(60);

    let mut egui_raylib = EguiRaylib::new();

    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut age = 1;
        while !raylib::WindowShouldClose() {
            use raylib_egui_rs::color::Color;

            raylib::BeginDrawing();
            raylib::ClearBackground(Color::BLACK);

            egui_raylib.run(|egui_ctx| {
                egui::Window::new("UI").show(&egui_ctx, |ui| {
                    ui.visuals_mut().dark_mode = false;
                    ui.label("XXX");
                    ui.label("a very nice gui :3");
                    if ui.button("print \"hello world\"").clicked() {
                        println!("hello world");
                    }
                    if ui.button("quit").clicked() {
                        // ctx.request_quit();
                    }
                    ui.image(egui::include_image!("ferris.png"));
                });
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

// fn update(ctx: &mut Context, gui: &mut Gui) {
//     let gui_ctx = gui.ctx();
//
//     gui_ctx.set_visuals(egui::Visuals::light());
//
//     egui::Window::new("UI").show(&gui_ctx, |ui| {
//         ui.visuals_mut().dark_mode = false;
//         ui.label("XXX");
//         ui.label("a very nice gui :3");
//         if ui.button("print \"hello world\"").clicked() {
//             println!("hello world");
//         }
//         if ui.button("quit").clicked() {
//             // ctx.request_quit();
//         }
//         ui.image(egui::include_image!("ferris.png"));
//     });
//     gui.update();
// }
