use mint::Vector2;
use raylib_egui_rs::raylib::*;
use raylib_egui_rs::*;

#[cfg(target_arch = "wasm32")]
use std::ffi::{c_int, c_void};

#[allow(static_mut_refs)]
#[cfg(target_arch = "wasm32")]
type EmArgCallbackFunc = unsafe extern "C" fn(arg: *mut c_void);

#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn emscripten_set_main_loop_arg(
        func: EmArgCallbackFunc,
        arg: *mut c_void,
        fps: c_int,
        simulate_infinite_loop: c_int,
    );
}

#[cfg(target_arch = "wasm32")]
unsafe extern "C" fn main_loop_wrapper(arg: *mut c_void) {
    let game_state = &mut *(arg as *mut GameState);
    update(game_state);
}

struct GameState {
    position: Vector2<f32>,
    force: Vector2<f32>,
    size: Vector2<f32>,
}

pub fn main() {
    InitWindow(800, 600, "hello saylor");
    SetTargetFPS(60);

    const SPEED: f32 = 200.0;

    let mut state = GameState {
        position: Vector2 { x: 0.0, y: 0.0 },
        force: Vector2 { x: SPEED, y: SPEED },
        size: Vector2 { x: 20.0, y: 20.0 },
    };

    // Main game loop
    #[cfg(not(target_arch = "wasm32"))]
    {
        while !WindowShouldClose() {
            update(&mut state);
        }
        CloseWindow();
    }
    #[cfg(target_arch = "wasm32")]
    {
        let boxed_state = Box::new(state);
        let state_ptr = Box::into_raw(boxed_state) as *mut c_void;
        unsafe {
            emscripten_set_main_loop_arg(main_loop_wrapper, state_ptr, 0, 1);
        }
    }

    while !WindowShouldClose() {}
    CloseWindow();
}

fn update(state: &mut GameState) {
    BeginDrawing();
    let dt = GetFrameTime();
    ClearBackground(raylib_egui_rs::color::Color::BLANK);

    state.position = Vector2Add(
        state.position,
        Vector2Multiply(state.force, Vector2 { x: dt, y: dt }),
    );
    if state.position.x < 0.0 || state.position.x + state.size.x >= GetScreenWidth() as f32 {
        state.force.x = state.force.x * -1.0;
    }
    if state.position.y < 0.0 || state.position.y + state.size.y >= GetScreenHeight() as f32 {
        state.force.y = state.force.y * -1.0;
    }
    DrawRectangleV(state.position, state.size, color::Color::LIME);
    EndDrawing();
}
