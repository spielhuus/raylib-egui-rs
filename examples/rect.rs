use mint::Vector2;
use raylib_egui_rs::*;

pub fn main() {
    InitWindow(800, 600, "hello saylor");
    SetTargetFPS(60);

    const SPEED: f32 = 200.0;

    let mut position = Vector2 { x: 0.0, y: 0.0 };
    let mut force = Vector2 { x: SPEED, y: SPEED };
    let size = Vector2 { x: 20.0, y: 20.0 };

    while !WindowShouldClose() {
        BeginDrawing();
        let dt = GetFrameTime();
        ClearBackground(raylib_egui_rs::color::Color::BLANK);

        position = Vector2Add(position, Vector2Multiply(force, Vector2 { x: dt, y: dt }));
        if position.x < 0.0 || position.x + size.x >= GetScreenWidth() as f32 {
            force.x = force.x * -1.0;
        }
        if position.y < 0.0 || position.y + size.y >= GetScreenHeight() as f32 {
            force.y = force.y * -1.0;
        }
        DrawRectangleV(position, size, color::Color::LIME);
        EndDrawing();
    }
    CloseWindow();
}
