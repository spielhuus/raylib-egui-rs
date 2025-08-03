// #include "raylib.h"

use raylib_egui_rs::raylib::*;
use raylib_egui_rs::color::Color;

const  NUM_TEXTURES: usize =    9;      // Currently we have 8 generation algorithms but some have multiple purposes (Linear and Square Gradients)
const screenWidth: i32 = 800;
const screenHeight: i32 = 450;

//------------------------------------------------------------------------------------
// Program main entry point
//------------------------------------------------------------------------------------
pub fn main() {

    // Initialization
    //--------------------------------------------------------------------------------------

    InitWindow(screenWidth, screenHeight, "raylib [textures] example - procedural images generation");

     let verticalGradient = GenImageGradientLinear(screenWidth, screenHeight, 0, Color::RED, Color::BLUE);
     let horizontalGradient = GenImageGradientLinear(screenWidth, screenHeight, 90, Color::RED, Color::BLUE);
     let diagonalGradient = GenImageGradientLinear(screenWidth, screenHeight, 45, Color::RED, Color::BLUE);
     let radialGradient = GenImageGradientRadial(screenWidth, screenHeight, 0.0, Color::WHITE, Color::BLACK);
     let squareGradient = GenImageGradientSquare(screenWidth, screenHeight, 0.0, Color::WHITE, Color::BLACK);
     let checked = GenImageChecked(screenWidth, screenHeight, 32, 32, Color::RED, Color::BLUE);
     let whiteNoise = GenImageWhiteNoise(screenWidth, screenHeight, 0.5);
     let perlinNoise = GenImagePerlinNoise(screenWidth, screenHeight, 50, 50, 4.0);
     let cellular = GenImageCellular(screenWidth, screenHeight, 32);

let mut textures: [Texture2D; NUM_TEXTURES] = std::array::from_fn(|_| Texture2D::default()); 

    textures[0] = LoadTextureFromImage(verticalGradient);
    textures[1] = LoadTextureFromImage(horizontalGradient);
    textures[2] = LoadTextureFromImage(diagonalGradient);
    textures[3] = LoadTextureFromImage(radialGradient);
    textures[4] = LoadTextureFromImage(squareGradient);
    textures[5] = LoadTextureFromImage(checked);
    textures[6] = LoadTextureFromImage(whiteNoise);
    textures[7] = LoadTextureFromImage(perlinNoise);
    textures[8] = LoadTextureFromImage(cellular);

    // Unload image data (CPU RAM)
    UnloadImage(verticalGradient);
    UnloadImage(horizontalGradient);
    UnloadImage(diagonalGradient);
    UnloadImage(radialGradient);
    UnloadImage(squareGradient);
    UnloadImage(checked);
    UnloadImage(whiteNoise);
    UnloadImage(perlinNoise);
    UnloadImage(cellular);

let mut currentTexture = 0;

    SetTargetFPS(60);
    //---------------------------------------------------------------------------------------

    // Main game loop
    while (!WindowShouldClose())
    {
        // Update
        //----------------------------------------------------------------------------------
        if (IsMouseButtonPressed(MouseButton::MOUSE_BUTTON_LEFT as i32) || IsKeyPressed(KeyboardKey::KEY_RIGHT))
        {
            currentTexture = (currentTexture + 1)%NUM_TEXTURES; // Cycle between the textures
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        BeginDrawing();

            ClearBackground(Color::RAYWHITE);

            DrawTexture(textures[currentTexture], 0, 0, Color::WHITE);

            DrawRectangle(30, 400, 325, 30, Fade(Color::SKYBLUE, 0.5));
            DrawRectangleLines(30, 400, 325, 30, Fade(Color::WHITE, 0.5));
            DrawText("MOUSE LEFT BUTTON to CYCLE PROCEDURAL TEXTURES", 40, 410, 10, Color::WHITE);

            match currentTexture
            {
                 0 => DrawText("VERTICAL GRADIENT", 560, 10, 20, Color::RAYWHITE),
                 1 => DrawText("HORIZONTAL GRADIENT", 540, 10, 20, Color::RAYWHITE),
                 2 => DrawText("DIAGONAL GRADIENT", 540, 10, 20, Color::RAYWHITE),
                 3 => DrawText("RADIAL GRADIENT", 580, 10, 20, Color::LIGHTGRAY),
                 4 => DrawText("SQUARE GRADIENT", 580, 10, 20, Color::LIGHTGRAY),
                 5 => DrawText("CHECKED", 680, 10, 20, Color::RAYWHITE),
                 6 => DrawText("WHITE NOISE", 640, 10, 20, Color::RED),
                 7 => DrawText("PERLIN NOISE", 640, 10, 20, Color::RED),
                 8 => DrawText("CELLULAR", 670, 10, 20, Color::RAYWHITE),
                 _ => unreachable!()
            }

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------

    // Unload textures data (GPU VRAM)
    for i in 0..NUM_TEXTURES { UnloadTexture(textures[i]) };

    CloseWindow();                // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

}
