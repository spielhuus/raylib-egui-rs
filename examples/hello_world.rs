use raylib_egui_rs::*;

pub fn main() {
    InitWindow(800, 600, "hello saylor");
    SetTargetFPS(60);

    while !WindowShouldClose() {
        BeginDrawing();
        ClearBackground(raylib_egui_rs::color::Color::BLACK);

        // DrawRectangleRec(state.board.walla, raylib::WHITE);
        DrawText("Hello Saylor", 200, 280, 64, color::Color::BISQUE);
        //TODO load external font
        EndDrawing();
    }
    CloseWindow();
}
