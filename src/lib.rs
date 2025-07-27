#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::{CStr, CString, c_char};
use std::os::raw::{c_int, c_void};

pub mod color;
pub mod math;
// TODO: only then feature is set
pub mod raygui;

mod ffi {
    use crate::color::*;
    use crate::math::*;
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use ffi::{
    Camera, Camera2D, Camera3D, Font, Image, KeyboardKey, Ray, RenderTexture2D, Shader, Texture2D,
    VrDeviceInfo, VrStereoConfig, float3, float16,
};

//------------------------------------------------------------------------------------
// Window and Graphics Device Functions (Module: core)
//------------------------------------------------------------------------------------

/// Initialize window and OpenGL context
pub fn InitWindow(width: i32, height: i32, title: &str) {
    let title = CString::new(title).expect("Failed to create CString from title");
    unsafe {
        ffi::InitWindow(width, height, title.as_ptr());
    }
}

/// Close window and unload OpenGL context
pub fn CloseWindow() {
    unsafe { ffi::CloseWindow() }
}

/// Check if application should close (KEY_ESCAPE pressed or windows close icon clicked)
pub fn WindowShouldClose() -> bool {
    unsafe { ffi::WindowShouldClose() }
}

/// Check if window has been initialized successfully
pub fn IsWindowReady() -> bool {
    unsafe { ffi::IsWindowReady() }
}

/// Check if window is currently fullscreen
pub fn IsWindowFullscreen() -> bool {
    unsafe { ffi::IsWindowFullscreen() }
}

/// Check if window is currently hidden
pub fn IsWindowHidden() -> bool {
    unsafe { ffi::IsWindowHidden() }
}

/// Check if window is currently minimized
pub fn IsWindowMinimized() -> bool {
    unsafe { ffi::IsWindowMinimized() }
}

/// Check if window is currently maximized
pub fn IsWindowMaximized() -> bool {
    unsafe { ffi::IsWindowMaximized() }
}

/// Check if window is currently focused
pub fn IsWindowFocused() -> bool {
    unsafe { ffi::IsWindowFocused() }
}

/// Check if window has been resized last frame
pub fn IsWindowResized() -> bool {
    unsafe { ffi::IsWindowResized() }
}

/// Check if one specific window flag is enabled
pub fn IsWindowState(flag: u32) -> bool {
    unsafe { ffi::IsWindowState(flag) }
}

/// Set window configuration state using flags
pub fn SetWindowState(flags: u32) {
    unsafe { ffi::SetWindowState(flags) }
}

/// Clear window configuration state flags
pub fn ClearWindowState(flags: u32) {
    unsafe { ffi::ClearWindowState(flags) }
}

/// Toggle window state: fullscreen/windowed
pub fn ToggleFullscreen() {
    unsafe { ffi::ToggleFullscreen() }
}

/// Toggle window state: borderless windowed
pub fn ToggleBorderlessWindowed() {
    unsafe { ffi::ToggleBorderlessWindowed() }
}

/// Set window state: maximized, if resizable
pub fn MaximizeWindow() {
    unsafe { ffi::MaximizeWindow() }
}

/// Set window state: minimized, if resizable
pub fn MinimizeWindow() {
    unsafe { ffi::MinimizeWindow() }
}

/// Set window state: not minimized/maximized
pub fn RestoreWindow() {
    unsafe { ffi::RestoreWindow() }
}

/// Set icon for window (single image, RGBA 32bit)
pub fn SetWindowIcon(image: Image) {
    unsafe { ffi::SetWindowIcon(image) }
}

/// Set icon for window (multiple images, RGBA 32bit)
pub fn SetWindowIcons(images: &mut [Image]) {
    unsafe {
        ffi::SetWindowIcons(images.as_mut_ptr(), images.len() as c_int);
    }
}

/// Set title for window
pub fn SetWindowTitle(title: &str) {
    let title = CString::new(title).expect("Failed to create CString from title");
    unsafe { ffi::SetWindowTitle(title.as_ptr()) }
}

/// Set window position on screen
pub fn SetWindowPosition(x: i32, y: i32) {
    unsafe { ffi::SetWindowPosition(x, y) }
}

/// Set monitor for the current window
pub fn SetWindowMonitor(monitor: i32) {
    unsafe { ffi::SetWindowMonitor(monitor) }
}

/// Set window minimum dimensions
pub fn SetWindowMinSize(width: i32, height: i32) {
    unsafe { ffi::SetWindowMinSize(width, height) }
}

/// Set window maximum dimensions
pub fn SetWindowMaxSize(width: i32, height: i32) {
    unsafe { ffi::SetWindowMaxSize(width, height) }
}

/// Set window dimensions
pub fn SetWindowSize(width: i32, height: i32) {
    unsafe { ffi::SetWindowSize(width, height) }
}

/// Set window opacity [0.0f..1.0f]
pub fn SetWindowOpacity(opacity: f32) {
    unsafe { ffi::SetWindowOpacity(opacity) }
}

/// Set window focused
pub fn SetWindowFocused() {
    unsafe { ffi::SetWindowFocused() }
}

/// Get native window handle
pub fn GetWindowHandle() -> *mut c_void {
    unsafe { ffi::GetWindowHandle() }
}

/// Get current screen width
pub fn GetScreenWidth() -> i32 {
    unsafe { ffi::GetScreenWidth() }
}

/// Get current screen height
pub fn GetScreenHeight() -> i32 {
    unsafe { ffi::GetScreenHeight() }
}

/// Get current render width (it considers HiDPI)
pub fn GetRenderWidth() -> i32 {
    unsafe { ffi::GetRenderWidth() }
}

/// Get current render height (it considers HiDPI)
pub fn GetRenderHeight() -> i32 {
    unsafe { ffi::GetRenderHeight() }
}

/// Get number of connected monitors
pub fn GetMonitorCount() -> i32 {
    unsafe { ffi::GetMonitorCount() }
}

/// Get current monitor where window is placed
pub fn GetCurrentMonitor() -> i32 {
    unsafe { ffi::GetCurrentMonitor() }
}

/// Get specified monitor position
pub fn GetMonitorPosition(monitor: i32) -> math::Vector2 {
    unsafe { ffi::GetMonitorPosition(monitor) }
}

/// Get specified monitor width
pub fn GetMonitorWidth(monitor: i32) -> i32 {
    unsafe { ffi::GetMonitorWidth(monitor) }
}

/// Get specified monitor height
pub fn GetMonitorHeight(monitor: i32) -> i32 {
    unsafe { ffi::GetMonitorHeight(monitor) }
}

/// Get specified monitor physical width in millimetres
pub fn GetMonitorPhysicalWidth(monitor: i32) -> i32 {
    unsafe { ffi::GetMonitorPhysicalWidth(monitor) }
}

/// Get specified monitor physical height in millimetres
pub fn GetMonitorPhysicalHeight(monitor: i32) -> i32 {
    unsafe { ffi::GetMonitorPhysicalHeight(monitor) }
}

/// Get specified monitor refresh rate
pub fn GetMonitorRefreshRate(monitor: i32) -> i32 {
    unsafe { ffi::GetMonitorRefreshRate(monitor) }
}

/// Get window position XY on monitor
pub fn GetWindowPosition() -> math::Vector2 {
    unsafe { ffi::GetWindowPosition() }
}

/// Get window scale DPI factor
pub fn GetWindowScaleDPI() -> math::Vector2 {
    unsafe { ffi::GetWindowScaleDPI() }
}

/// Get the human-readable, UTF-8 encoded name of the specified monitor
pub fn GetMonitorName(monitor: i32) -> &'static str {
    unsafe {
        let c_str = ffi::GetMonitorName(monitor);
        CStr::from_ptr(c_str)
            .to_str()
            .expect("Failed to convert C string to &str")
    }
}

/// Set clipboard text content
pub fn SetClipboardText(text: &str) {
    let text = CString::new(text).expect("Failed to create CString from text");
    unsafe { ffi::SetClipboardText(text.as_ptr()) }
}

/// Get clipboard text content
pub fn GetClipboardText() -> &'static str {
    unsafe {
        let c_str = ffi::GetClipboardText();
        CStr::from_ptr(c_str)
            .to_str()
            .expect("Failed to convert C string to &str")
    }
}

/// Get clipboard image
pub fn GetClipboardImage() -> Image {
    unsafe { ffi::GetClipboardImage() }
}

/// Enable waiting for events on EndDrawing(), no automatic event polling
pub fn EnableEventWaiting() {
    unsafe { ffi::EnableEventWaiting() }
}

/// Disable waiting for events on EndDrawing(), automatic events polling
pub fn DisableEventWaiting() {
    unsafe { ffi::DisableEventWaiting() }
}

//------------------------------------------------------------------------------------
// Cursor-related functions
//------------------------------------------------------------------------------------

/// Shows cursor
pub fn ShowCursor() {
    unsafe { ffi::ShowCursor() }
}

/// Hides cursor
pub fn HideCursor() {
    unsafe { ffi::HideCursor() }
}

/// Check if cursor is not visible
pub fn IsCursorHidden() -> bool {
    unsafe { ffi::IsCursorHidden() }
}

/// Enables cursor (unlock cursor)
pub fn EnableCursor() {
    unsafe { ffi::EnableCursor() }
}

/// Disables cursor (lock cursor)
pub fn DisableCursor() {
    unsafe { ffi::DisableCursor() }
}

/// Check if cursor is on the screen
pub fn IsCursorOnScreen() -> bool {
    unsafe { ffi::IsCursorOnScreen() }
}

//------------------------------------------------------------------------------------
// Drawing-related functions
//------------------------------------------------------------------------------------

/// Set background color (framebuffer clear color)
pub fn ClearBackground(color: color::Color) {
    unsafe { ffi::ClearBackground(color) }
}

/// Setup canvas (framebuffer) to start drawing
pub fn BeginDrawing() {
    unsafe { ffi::BeginDrawing() }
}

/// End canvas drawing and swap buffers (double buffering)
pub fn EndDrawing() {
    unsafe { ffi::EndDrawing() }
}

/// Begin 2D mode with custom camera (2D)
pub fn BeginMode2D(camera: Camera2D) {
    unsafe { ffi::BeginMode2D(camera) }
}

/// Ends 2D mode with custom camera
pub fn EndMode2D() {
    unsafe { ffi::EndMode2D() }
}

/// Begin 3D mode with custom camera (3D)
pub fn BeginMode3D(camera: Camera3D) {
    unsafe { ffi::BeginMode3D(camera) }
}

/// Ends 3D mode and returns to default 2D orthographic mode
pub fn EndMode3D() {
    unsafe { ffi::EndMode3D() }
}

/// Begin drawing to render texture
pub fn BeginTextureMode(target: RenderTexture2D) {
    unsafe { ffi::BeginTextureMode(target) }
}

/// Ends drawing to render texture
pub fn EndTextureMode() {
    unsafe { ffi::EndTextureMode() }
}

/// Begin custom shader drawing
pub fn BeginShaderMode(shader: Shader) {
    unsafe { ffi::BeginShaderMode(shader) }
}

/// End custom shader drawing (use default shader)
pub fn EndShaderMode() {
    unsafe { ffi::EndShaderMode() }
}

/// Begin blending mode (alpha, additive, multiplied, subtract, custom)
pub fn BeginBlendMode(mode: i32) {
    unsafe { ffi::BeginBlendMode(mode) }
}

/// End blending mode (reset to default: alpha blending)
pub fn EndBlendMode() {
    unsafe { ffi::EndBlendMode() }
}

/// Begin scissor mode (define screen area for following drawing)
pub fn BeginScissorMode(x: i32, y: i32, width: i32, height: i32) {
    unsafe { ffi::BeginScissorMode(x, y, width, height) }
}

/// End scissor mode
pub fn EndScissorMode() {
    unsafe { ffi::EndScissorMode() }
}

/// Begin stereo rendering (requires VR simulator)
pub fn BeginVrStereoMode(config: VrStereoConfig) {
    unsafe { ffi::BeginVrStereoMode(config) }
}

/// End stereo rendering (requires VR simulator)
pub fn EndVrStereoMode() {
    unsafe { ffi::EndVrStereoMode() }
}

//------------------------------------------------------------------------------------
// VR stereo config functions for VR simulator
//------------------------------------------------------------------------------------

/// Load VR stereo config for VR simulator device parameters
pub fn LoadVrStereoConfig(device: VrDeviceInfo) -> VrStereoConfig {
    unsafe { ffi::LoadVrStereoConfig(device) }
}

/// Unload VR stereo config
pub fn UnloadVrStereoConfig(config: VrStereoConfig) {
    unsafe { ffi::UnloadVrStereoConfig(config) }
}

//------------------------------------------------------------------------------------
// Shader management functions
//------------------------------------------------------------------------------------

/// Load shader from files and bind default locations
pub fn LoadShader(vs_file_name: &str, fs_file_name: &str) -> Shader {
    let vs_file_name_c = CString::new(vs_file_name).expect("CString::new failed");
    let fs_file_name_c = CString::new(fs_file_name).expect("CString::new failed");
    unsafe { ffi::LoadShader(vs_file_name_c.as_ptr(), fs_file_name_c.as_ptr()) }
}

/// Load shader from code strings and bind default locations
pub fn LoadShaderFromMemory(vs_code: &str, fs_code: &str) -> Shader {
    let vs_code_c = CString::new(vs_code).expect("CString::new failed");
    let fs_code_c = CString::new(fs_code).expect("CString::new failed");
    unsafe { ffi::LoadShaderFromMemory(vs_code_c.as_ptr(), fs_code_c.as_ptr()) }
}

/// Check if a shader is valid (loaded on GPU)
pub fn IsShaderValid(shader: Shader) -> bool {
    unsafe { ffi::IsShaderValid(shader) }
}

/// Get shader uniform location
pub fn GetShaderLocation(shader: Shader, uniform_name: &str) -> i32 {
    let uniform_name_c = CString::new(uniform_name).expect("CString::new failed");
    unsafe { ffi::GetShaderLocation(shader, uniform_name_c.as_ptr()) }
}

/// Get shader attribute location
pub fn GetShaderLocationAttrib(shader: Shader, attrib_name: &str) -> i32 {
    let attrib_name_c = CString::new(attrib_name).expect("CString::new failed");
    unsafe { ffi::GetShaderLocationAttrib(shader, attrib_name_c.as_ptr()) }
}

/// Set shader uniform value
pub fn SetShaderValue<T>(shader: Shader, loc_index: i32, value: &T, uniform_type: i32) {
    unsafe {
        ffi::SetShaderValue(
            shader,
            loc_index,
            value as *const T as *const c_void,
            uniform_type,
        );
    }
}

/// Set shader uniform value vector
pub fn SetShaderValueV<T>(
    shader: Shader,
    loc_index: i32,
    value: &[T],
    uniform_type: i32,
    count: i32,
) {
    unsafe {
        ffi::SetShaderValueV(
            shader,
            loc_index,
            value.as_ptr() as *const c_void,
            uniform_type,
            count,
        );
    }
}

/// Set shader uniform value (matrix 4x4)
pub fn SetShaderValueMatrix(shader: Shader, loc_index: i32, mat: math::Matrix) {
    unsafe { ffi::SetShaderValueMatrix(shader, loc_index, mat) }
}

/// Set shader uniform value for texture (sampler2d)
pub fn SetShaderValueTexture(shader: Shader, loc_index: i32, texture: Texture2D) {
    unsafe { ffi::SetShaderValueTexture(shader, loc_index, texture) }
}

/// Unload shader from GPU memory (VRAM)
pub fn UnloadShader(shader: Shader) {
    unsafe { ffi::UnloadShader(shader) }
}

//------------------------------------------------------------------------------------
// Screen-space-related functions
//------------------------------------------------------------------------------------

/// Get a ray trace from screen position (i.e mouse)
pub fn GetScreenToWorldRay(position: math::Vector2, camera: Camera) -> Ray {
    unsafe { ffi::GetScreenToWorldRay(position, camera) }
}

/// Compatibility for previous raylib versions
pub fn GetMouseRay(position: math::Vector2, camera: Camera) -> Ray {
    GetScreenToWorldRay(position, camera)
}

/// Get a ray trace from screen position (i.e mouse) in a viewport
pub fn GetScreenToWorldRayEx(
    position: math::Vector2,
    camera: Camera,
    width: i32,
    height: i32,
) -> Ray {
    unsafe { ffi::GetScreenToWorldRayEx(position, camera, width, height) }
}

/// Get the screen space position for a 3d world space position
pub fn GetWorldToScreen(position: math::Vector3, camera: Camera) -> math::Vector2 {
    unsafe { ffi::GetWorldToScreen(position, camera) }
}

/// Get size position for a 3d world space position
pub fn GetWorldToScreenEx(
    position: math::Vector3,
    camera: Camera,
    width: i32,
    height: i32,
) -> math::Vector2 {
    unsafe { ffi::GetWorldToScreenEx(position, camera, width, height) }
}

/// Get the screen space position for a 2d camera world space position
pub fn GetWorldToScreen2D(position: math::Vector2, camera: Camera2D) -> math::Vector2 {
    unsafe { ffi::GetWorldToScreen2D(position, camera) }
}

/// Get the world space position for a 2d camera screen space position
pub fn GetScreenToWorld2D(position: math::Vector2, camera: Camera2D) -> math::Vector2 {
    unsafe { ffi::GetScreenToWorld2D(position, camera) }
}

/// Get camera transform matrix (view matrix)
pub fn GetCameraMatrix(camera: Camera) -> math::Matrix {
    unsafe { ffi::GetCameraMatrix(camera) }
}

/// Get camera 2d transform matrix
pub fn GetCameraMatrix2D(camera: Camera2D) -> math::Matrix {
    unsafe { ffi::GetCameraMatrix2D(camera) }
}

//------------------------------------------------------------------------------------
// Timing-related functions
//------------------------------------------------------------------------------------

/// Set target FPS (maximum)
pub fn SetTargetFPS(fps: i32) {
    unsafe { ffi::SetTargetFPS(fps) }
}

/// Get time in seconds for last frame drawn (delta time)
pub fn GetFrameTime() -> f32 {
    unsafe { ffi::GetFrameTime() }
}

/// Get elapsed time in seconds since InitWindow()
pub fn GetTime() -> f64 {
    unsafe { ffi::GetTime() }
}

/// Get current FPS
pub fn GetFPS() -> i32 {
    unsafe { ffi::GetFPS() }
}

//------------------------------------------------------------------------------------
// Custom frame control functions
//------------------------------------------------------------------------------------

/// Swap back buffer with front buffer (screen drawing)
pub fn SwapScreenBuffer() {
    unsafe { ffi::SwapScreenBuffer() }
}

/// Register all input events
pub fn PollInputEvents() {
    unsafe { ffi::PollInputEvents() }
}

/// Wait for some time (halt program execution)
pub fn WaitTime(seconds: f64) {
    unsafe { ffi::WaitTime(seconds) }
}

//------------------------------------------------------------------------------------
// Misc. functions
//------------------------------------------------------------------------------------

/// Takes a screenshot of current screen (filename extension defines format)
pub fn TakeScreenshot(file_name: &str) {
    let file_name_c = CString::new(file_name).expect("CString::new failed");
    unsafe { ffi::TakeScreenshot(file_name_c.as_ptr()) }
}

/// Setup init configuration flags (view FLAGS)
pub fn SetConfigFlags(flags: u32) {
    unsafe { ffi::SetConfigFlags(flags) }
}

/// Open URL with default system browser (if available)
pub fn OpenURL(url: &str) {
    let url_c = CString::new(url).expect("CString::new failed");
    unsafe { ffi::OpenURL(url_c.as_ptr()) }
}

/// Show trace log messages (a simple wrapper for the variadic C function)
pub fn TraceLog(log_level: i32, text: &str) {
    let text_c = CString::new(text).expect("CString::new failed");
    unsafe { ffi::TraceLog(log_level, text_c.as_ptr()) }
}

/// Set the current threshold (minimum) log level
pub fn SetTraceLogLevel(log_level: i32) {
    unsafe { ffi::SetTraceLogLevel(log_level) }
}

pub type TraceLogCallback = Option<
    unsafe extern "C" fn(logLevel: c_int, text: *const std::os::raw::c_char, args: ffi::va_list),
>;
pub type LoadFileDataCallback = Option<
    unsafe extern "C" fn(fileName: *const std::os::raw::c_char, bytesRead: *mut c_int) -> *mut u8,
>;

// TODO: what to do with trace logging?
// Set custom trace log
// pub fn SetTraceLogCallback(callback: TraceLogCallback) {
//     unsafe { ffi::SetTraceLogCallback(callback) }
// }

// ... similar simple wrappers for SetLoadFileDataCallback, etc.

//------------------------------------------------------------------------------------
// Files management functions
//------------------------------------------------------------------------------------

/// Load file data as byte array (read)
pub fn LoadFileData(file_name: &str) -> Option<&'static mut [u8]> {
    let file_name_c = CString::new(file_name).expect("CString::new failed");
    let mut data_size: c_int = 0;
    unsafe {
        let data = ffi::LoadFileData(file_name_c.as_ptr(), &mut data_size);
        if data.is_null() {
            None
        } else {
            Some(std::slice::from_raw_parts_mut(data, data_size as usize))
        }
    }
}

/// Unload file data allocated by LoadFileData()
pub fn UnloadFileData(data: *mut u8) {
    unsafe { ffi::UnloadFileData(data) }
}

/// Save data to file from byte array (write), returns true on success
pub fn SaveFileData(file_name: &str, data: &mut [u8]) -> bool {
    let file_name_c = CString::new(file_name).expect("CString::new failed");
    unsafe {
        ffi::SaveFileData(
            file_name_c.as_ptr(),
            data.as_mut_ptr() as *mut c_void,
            data.len() as c_int,
        )
    }
}

/// Load text data from file (read), returns a '\0' terminated string
pub fn LoadFileText(file_name: &str) -> Option<String> {
    let file_name_c = CString::new(file_name).expect("CString::new failed");
    unsafe {
        let text_ptr = ffi::LoadFileText(file_name_c.as_ptr());
        if text_ptr.is_null() {
            None
        } else {
            let c_str = CStr::from_ptr(text_ptr);
            let result = c_str.to_string_lossy().into_owned();
            ffi::UnloadFileText(text_ptr); // Important: free the memory allocated by C
            Some(result)
        }
    }
}

/// Save text data to file (write), string must be '\0' terminated, returns true on success
pub fn SaveFileText(file_name: &str, text: &str) -> bool {
    let file_name_c = CString::new(file_name).expect("CString::new failed");
    let mut text_c = CString::new(text)
        .expect("CString::new failed")
        .into_bytes_with_nul();
    unsafe {
        ffi::SaveFileText(
            file_name_c.as_ptr(),
            text_c.as_mut_ptr() as *mut std::os::raw::c_char,
        )
    }
}

//------------------------------------------------------------------------------------
// Input Handling Functions (Module: core)
//------------------------------------------------------------------------------------

/// Check if a key has been pressed once
pub fn IsKeyPressed(key: KeyboardKey) -> bool {
    unsafe { ffi::IsKeyPressed(key as i32) }
}

/// Check if a key has been pressed again
pub fn IsKeyPressedRepeat(key: i32) -> bool {
    unsafe { ffi::IsKeyPressedRepeat(key) }
}

/// Check if a key is being pressed
pub fn IsKeyDown(key: KeyboardKey) -> bool {
    unsafe { ffi::IsKeyDown(key as i32) }
}

/// Check if a key has been released once
pub fn IsKeyReleased(key: KeyboardKey) -> bool {
    unsafe { ffi::IsKeyReleased(key as i32) }
}

// TODO: return KeyboardKey

/// Check if a key is NOT being pressed
pub fn IsKeyUp(key: KeyboardKey) -> bool {
    unsafe { ffi::IsKeyUp(key as i32) }
}

/// Get key pressed (keycode), call it multiple times for keys queued
pub fn GetKeyPressed() -> i32 {
    unsafe { ffi::GetKeyPressed() }
}

/// Get char pressed (unicode), call it multiple times for chars queued
pub fn GetCharPressed() -> i32 {
    unsafe { ffi::GetCharPressed() }
}

/// Set a custom key to exit program
pub fn SetExitKey(key: i32) {
    unsafe { ffi::SetExitKey(key) }
}

/// Check if a gamepad is available
pub fn IsGamepadAvailable(gamepad: i32) -> bool {
    unsafe { ffi::IsGamepadAvailable(gamepad) }
}

/// Get gamepad internal name id
pub fn GetGamepadName(gamepad: i32) -> &'static str {
    unsafe {
        CStr::from_ptr(ffi::GetGamepadName(gamepad))
            .to_str()
            .unwrap()
    }
}

/// Check if a gamepad button has been pressed once
pub fn IsGamepadButtonPressed(gamepad: i32, button: i32) -> bool {
    unsafe { ffi::IsGamepadButtonPressed(gamepad, button) }
}

/// Check if a gamepad button is being pressed
pub fn IsGamepadButtonDown(gamepad: i32, button: i32) -> bool {
    unsafe { ffi::IsGamepadButtonDown(gamepad, button) }
}

/// Check if a gamepad button has been released once
pub fn IsGamepadButtonReleased(gamepad: i32, button: i32) -> bool {
    unsafe { ffi::IsGamepadButtonReleased(gamepad, button) }
}

/// Check if a gamepad button is NOT being pressed
pub fn IsGamepadButtonUp(gamepad: i32, button: i32) -> bool {
    unsafe { ffi::IsGamepadButtonUp(gamepad, button) }
}

/// Get the last gamepad button pressed
pub fn GetGamepadButtonPressed() -> i32 {
    unsafe { ffi::GetGamepadButtonPressed() }
}

/// Get gamepad axis count for a gamepad
pub fn GetGamepadAxisCount(gamepad: i32) -> i32 {
    unsafe { ffi::GetGamepadAxisCount(gamepad) }
}

/// Get axis movement value for a gamepad axis
pub fn GetGamepadAxisMovement(gamepad: i32, axis: i32) -> f32 {
    unsafe { ffi::GetGamepadAxisMovement(gamepad, axis) }
}

/// Set internal gamepad mappings (SDL_GameControllerDB)
pub fn SetGamepadMappings(mappings: &str) -> i32 {
    let mappings_c = CString::new(mappings).unwrap();
    unsafe { ffi::SetGamepadMappings(mappings_c.as_ptr()) }
}

/// Check if a mouse button has been pressed once
pub fn IsMouseButtonPressed(button: i32) -> bool {
    unsafe { ffi::IsMouseButtonPressed(button) }
}

/// Check if a mouse button is being pressed
pub fn IsMouseButtonDown(button: i32) -> bool {
    unsafe { ffi::IsMouseButtonDown(button) }
}

/// Check if a mouse button has been released once
pub fn IsMouseButtonReleased(button: i32) -> bool {
    unsafe { ffi::IsMouseButtonReleased(button) }
}

/// Check if a mouse button is NOT being pressed
pub fn IsMouseButtonUp(button: i32) -> bool {
    unsafe { ffi::IsMouseButtonUp(button) }
}

/// Get mouse position X
pub fn GetMouseX() -> i32 {
    unsafe { ffi::GetMouseX() }
}

/// Get mouse position Y
pub fn GetMouseY() -> i32 {
    unsafe { ffi::GetMouseY() }
}

/// Get mouse position XY
pub fn GetMousePosition() -> math::Vector2 {
    unsafe { ffi::GetMousePosition() }
}

/// Get mouse delta between frames
pub fn GetMouseDelta() -> math::Vector2 {
    unsafe { ffi::GetMouseDelta() }
}

/// Set mouse position XY
pub fn SetMousePosition(x: i32, y: i32) {
    unsafe { ffi::SetMousePosition(x, y) }
}

/// Set mouse offset
pub fn SetMouseOffset(offset_x: i32, offset_y: i32) {
    unsafe { ffi::SetMouseOffset(offset_x, offset_y) }
}

/// Set mouse scaling
pub fn SetMouseScale(scale_x: f32, scale_y: f32) {
    unsafe { ffi::SetMouseScale(scale_x, scale_y) }
}

/// Get mouse wheel movement for X or Y, whichever is larger
pub fn GetMouseWheelMove() -> f32 {
    unsafe { ffi::GetMouseWheelMove() }
}

/// Get mouse wheel movement for both X and Y
pub fn GetMouseWheelMoveV() -> math::Vector2 {
    unsafe { ffi::GetMouseWheelMoveV() }
}

/// Set mouse cursor
pub fn SetMouseCursor(cursor: i32) {
    unsafe { ffi::SetMouseCursor(cursor) }
}

/// Get touch position X for touch point 0
pub fn GetTouchX() -> i32 {
    unsafe { ffi::GetTouchX() }
}

/// Get touch position Y for touch point 0
pub fn GetTouchY() -> i32 {
    unsafe { ffi::GetTouchY() }
}

/// Get touch position XY for a touch point index
pub fn GetTouchPosition(index: i32) -> math::Vector2 {
    unsafe { ffi::GetTouchPosition(index) }
}

/// Get touch point identifier for given index
pub fn GetTouchPointId(index: i32) -> i32 {
    unsafe { ffi::GetTouchPointId(index) }
}

/// Get number of touch points
pub fn GetTouchPointCount() -> i32 {
    unsafe { ffi::GetTouchPointCount() }
}

//------------------------------------------------------------------------------------
// Gestures and Touch Handling Functions (Module: rgestures)
//------------------------------------------------------------------------------------

/// Enable a set of gestures using flags
pub fn SetGesturesEnabled(flags: u32) {
    unsafe { ffi::SetGesturesEnabled(flags) }
}

/// Check if a gesture have been detected
pub fn IsGestureDetected(gesture: u32) -> bool {
    unsafe { ffi::IsGestureDetected(gesture) }
}

/// Get latest detected gesture
pub fn GetGestureDetected() -> i32 {
    unsafe { ffi::GetGestureDetected() }
}

/// Get gesture hold time in seconds
pub fn GetGestureHoldDuration() -> f32 {
    unsafe { ffi::GetGestureHoldDuration() }
}

/// Get gesture drag vector
pub fn GetGestureDragVector() -> math::Vector2 {
    unsafe { ffi::GetGestureDragVector() }
}

/// Get gesture drag angle
pub fn GetGestureDragAngle() -> f32 {
    unsafe { ffi::GetGestureDragAngle() }
}

/// Get gesture pinch delta
pub fn GetGesturePinchVector() -> math::Vector2 {
    unsafe { ffi::GetGesturePinchVector() }
}

/// Get gesture pinch angle
pub fn GetGesturePinchAngle() -> f32 {
    unsafe { ffi::GetGesturePinchAngle() }
}

//------------------------------------------------------------------------------------
// Camera System Functions (Module: rcamera)
//------------------------------------------------------------------------------------

/// Update camera position for selected mode
pub fn UpdateCamera(camera: &mut Camera, mode: i32) {
    unsafe { ffi::UpdateCamera(camera as *mut _, mode) }
}

/// Update camera movement/rotation
pub fn UpdateCameraPro(
    camera: &mut Camera,
    movement: math::Vector3,
    rotation: math::Vector3,
    zoom: f32,
) {
    unsafe { ffi::UpdateCameraPro(camera as *mut _, movement, rotation, zoom) }
}

//------------------------------------------------------------------------------------
// Basic Shapes Drawing Functions (Module: rshapes)
//------------------------------------------------------------------------------------

/// Set texture and rectangle to be used on shapes drawing
pub fn SetShapesTexture(texture: Texture2D, source: math::Rectangle) {
    unsafe { ffi::SetShapesTexture(texture, source) }
}

/// Get texture that is used for shapes drawing
pub fn GetShapesTexture() -> Texture2D {
    unsafe { ffi::GetShapesTexture() }
}

/// Get texture source rectangle that is used for shapes drawing
pub fn GetShapesTextureRectangle() -> math::Rectangle {
    unsafe { ffi::GetShapesTextureRectangle() }
}

/// Draw a pixel
pub fn DrawPixel(pos_x: i32, pos_y: i32, color: color::Color) {
    unsafe { ffi::DrawPixel(pos_x, pos_y, color) }
}

/// Draw a pixel (Vector version)
pub fn DrawPixelV(position: math::Vector2, color: color::Color) {
    unsafe { ffi::DrawPixelV(position, color) }
}

/// Draw a line
pub fn DrawLine(
    start_pos_x: i32,
    start_pos_y: i32,
    end_pos_x: i32,
    end_pos_y: i32,
    color: color::Color,
) {
    unsafe { ffi::DrawLine(start_pos_x, start_pos_y, end_pos_x, end_pos_y, color) }
}

/// Draw a line (Vector version)
pub fn DrawLineV(start_pos: math::Vector2, end_pos: math::Vector2, color: color::Color) {
    unsafe { ffi::DrawLineV(start_pos, end_pos, color) }
}

/// Draw a line (using triangles/quads)
pub fn DrawLineEx(
    start_pos: math::Vector2,
    end_pos: math::Vector2,
    thick: f32,
    color: color::Color,
) {
    unsafe { ffi::DrawLineEx(start_pos, end_pos, thick, color) }
}

/// Draw lines sequence
pub fn DrawLineStrip(points: &[math::Vector2], color: color::Color) {
    unsafe { ffi::DrawLineStrip(points.as_ptr(), points.len() as c_int, color) }
}

/// Draw line segment cubic-bezier in-out interpolation
pub fn DrawLineBezier(
    start_pos: math::Vector2,
    end_pos: math::Vector2,
    thick: f32,
    color: color::Color,
) {
    unsafe { ffi::DrawLineBezier(start_pos, end_pos, thick, color) }
}

/// Draw a color-filled circle
pub fn DrawCircle(center_x: i32, center_y: i32, radius: f32, color: color::Color) {
    unsafe { ffi::DrawCircle(center_x, center_y, radius, color) }
}

/// Draw a piece of a circle
pub fn DrawCircleSector(
    center: math::Vector2,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    segments: i32,
    color: color::Color,
) {
    unsafe { ffi::DrawCircleSector(center, radius, start_angle, end_angle, segments, color) }
}

/// Draw circle sector outline
pub fn DrawCircleSectorLines(
    center: math::Vector2,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    segments: i32,
    color: color::Color,
) {
    unsafe { ffi::DrawCircleSectorLines(center, radius, start_angle, end_angle, segments, color) }
}

/// Draw a gradient-filled circle
pub fn DrawCircleGradient(
    center_x: i32,
    center_y: i32,
    radius: f32,
    inner: color::Color,
    outer: color::Color,
) {
    unsafe { ffi::DrawCircleGradient(center_x, center_y, radius, inner, outer) }
}

/// Draw a color-filled circle (Vector version)
pub fn DrawCircleV(center: math::Vector2, radius: f32, color: color::Color) {
    unsafe { ffi::DrawCircleV(center, radius, color) }
}

/// Draw circle outline
pub fn DrawCircleLines(center_x: i32, center_y: i32, radius: f32, color: color::Color) {
    unsafe { ffi::DrawCircleLines(center_x, center_y, radius, color) }
}

/// Draw circle outline (Vector version)
pub fn DrawCircleLinesV(center: math::Vector2, radius: f32, color: color::Color) {
    unsafe { ffi::DrawCircleLinesV(center, radius, color) }
}

/// Draw ellipse
pub fn DrawEllipse(
    center_x: i32,
    center_y: i32,
    radius_h: f32,
    radius_v: f32,
    color: color::Color,
) {
    unsafe { ffi::DrawEllipse(center_x, center_y, radius_h, radius_v, color) }
}

/// Draw ellipse outline
pub fn DrawEllipseLines(
    center_x: i32,
    center_y: i32,
    radius_h: f32,
    radius_v: f32,
    color: color::Color,
) {
    unsafe { ffi::DrawEllipseLines(center_x, center_y, radius_h, radius_v, color) }
}

/// Draw ring
pub fn DrawRing(
    center: math::Vector2,
    inner_radius: f32,
    outer_radius: f32,
    start_angle: f32,
    end_angle: f32,
    segments: i32,
    color: color::Color,
) {
    unsafe {
        ffi::DrawRing(
            center,
            inner_radius,
            outer_radius,
            start_angle,
            end_angle,
            segments,
            color,
        )
    }
}

/// Draw ring outline
pub fn DrawRingLines(
    center: math::Vector2,
    inner_radius: f32,
    outer_radius: f32,
    start_angle: f32,
    end_angle: f32,
    segments: i32,
    color: color::Color,
) {
    unsafe {
        ffi::DrawRingLines(
            center,
            inner_radius,
            outer_radius,
            start_angle,
            end_angle,
            segments,
            color,
        )
    }
}

/// Draw a color-filled rectangle
pub fn DrawRectangle(pos_x: i32, pos_y: i32, width: i32, height: i32, color: color::Color) {
    unsafe { ffi::DrawRectangle(pos_x, pos_y, width, height, color) }
}

/// Draw a color-filled rectangle (Vector version)
pub fn DrawRectangleV(position: math::Vector2, size: math::Vector2, color: color::Color) {
    unsafe { ffi::DrawRectangleV(position, size, color) }
}

/// Draw a color-filled rectangle
pub fn DrawRectangleRec(rec: math::Rectangle, color: color::Color) {
    unsafe { ffi::DrawRectangleRec(rec, color) }
}

/// Draw a color-filled rectangle with pro parameters
pub fn DrawRectanglePro(
    rec: math::Rectangle,
    origin: math::Vector2,
    rotation: f32,
    color: color::Color,
) {
    unsafe { ffi::DrawRectanglePro(rec, origin, rotation, color) }
}

/// Draw a vertical-gradient-filled rectangle
pub fn DrawRectangleGradientV(
    pos_x: i32,
    pos_y: i32,
    width: i32,
    height: i32,
    top: color::Color,
    bottom: color::Color,
) {
    unsafe { ffi::DrawRectangleGradientV(pos_x, pos_y, width, height, top, bottom) }
}

/// Draw a horizontal-gradient-filled rectangle
pub fn DrawRectangleGradientH(
    pos_x: i32,
    pos_y: i32,
    width: i32,
    height: i32,
    left: color::Color,
    right: color::Color,
) {
    unsafe { ffi::DrawRectangleGradientH(pos_x, pos_y, width, height, left, right) }
}

/// Draw a gradient-filled rectangle with custom vertex colors
pub fn DrawRectangleGradientEx(
    rec: math::Rectangle,
    top_left: color::Color,
    bottom_left: color::Color,
    top_right: color::Color,
    bottom_right: color::Color,
) {
    unsafe { ffi::DrawRectangleGradientEx(rec, top_left, bottom_left, top_right, bottom_right) }
}

/// Draw rectangle outline
pub fn DrawRectangleLines(pos_x: i32, pos_y: i32, width: i32, height: i32, color: color::Color) {
    unsafe { ffi::DrawRectangleLines(pos_x, pos_y, width, height, color) }
}

/// Draw rectangle outline with extended parameters
pub fn DrawRectangleLinesEx(rec: math::Rectangle, line_thick: f32, color: color::Color) {
    unsafe { ffi::DrawRectangleLinesEx(rec, line_thick, color) }
}

/// Draw rectangle with rounded edges
pub fn DrawRectangleRounded(
    rec: math::Rectangle,
    roundness: f32,
    segments: i32,
    color: color::Color,
) {
    unsafe { ffi::DrawRectangleRounded(rec, roundness, segments, color) }
}

/// Draw rectangle lines with rounded edges
pub fn DrawRectangleRoundedLines(
    rec: math::Rectangle,
    roundness: f32,
    segments: i32,
    color: color::Color,
) {
    unsafe { ffi::DrawRectangleRoundedLines(rec, roundness, segments, color) }
}

/// Draw rectangle with rounded edges outline
pub fn DrawRectangleRoundedLinesEx(
    rec: math::Rectangle,
    roundness: f32,
    segments: i32,
    line_thick: f32,
    color: color::Color,
) {
    unsafe { ffi::DrawRectangleRoundedLinesEx(rec, roundness, segments, line_thick, color) }
}

/// Draw a color-filled triangle (vertex in counter-clockwise order!)
pub fn DrawTriangle(v1: math::Vector2, v2: math::Vector2, v3: math::Vector2, color: color::Color) {
    unsafe { ffi::DrawTriangle(v1, v2, v3, color) }
}

/// Draw triangle outline (vertex in counter-clockwise order!)
pub fn DrawTriangleLines(
    v1: math::Vector2,
    v2: math::Vector2,
    v3: math::Vector2,
    color: color::Color,
) {
    unsafe { ffi::DrawTriangleLines(v1, v2, v3, color) }
}

/// Draw a triangle fan defined by points (first vertex is the center)
pub fn DrawTriangleFan(points: &[math::Vector2], color: color::Color) {
    unsafe { ffi::DrawTriangleFan(points.as_ptr(), points.len() as c_int, color) }
}

/// Draw a triangle strip defined by points
pub fn DrawTriangleStrip(points: &[math::Vector2], color: color::Color) {
    unsafe { ffi::DrawTriangleStrip(points.as_ptr(), points.len() as c_int, color) }
}

/// Draw a regular polygon (Vector version)
pub fn DrawPoly(
    center: math::Vector2,
    sides: i32,
    radius: f32,
    rotation: f32,
    color: color::Color,
) {
    unsafe { ffi::DrawPoly(center, sides, radius, rotation, color) }
}

/// Draw a polygon outline of n sides
pub fn DrawPolyLines(
    center: math::Vector2,
    sides: i32,
    radius: f32,
    rotation: f32,
    color: color::Color,
) {
    unsafe { ffi::DrawPolyLines(center, sides, radius, rotation, color) }
}

/// Draw a polygon outline of n sides with extended parameters
pub fn DrawPolyLinesEx(
    center: math::Vector2,
    sides: i32,
    radius: f32,
    rotation: f32,
    line_thick: f32,
    color: color::Color,
) {
    unsafe { ffi::DrawPolyLinesEx(center, sides, radius, rotation, line_thick, color) }
}

//------------------------------------------------------------------------------------
// Splines Drawing Functions
//------------------------------------------------------------------------------------

/// Draw spline: Linear, minimum 2 points
pub fn DrawSplineLinear(points: &[math::Vector2], thick: f32, color: color::Color) {
    unsafe { ffi::DrawSplineLinear(points.as_ptr(), points.len() as c_int, thick, color) }
}

/// Draw spline: B-Spline, minimum 4 points
pub fn DrawSplineBasis(points: &[math::Vector2], thick: f32, color: color::Color) {
    unsafe { ffi::DrawSplineBasis(points.as_ptr(), points.len() as c_int, thick, color) }
}

/// Draw spline: Catmull-Rom, minimum 4 points
pub fn DrawSplineCatmullRom(points: &[math::Vector2], thick: f32, color: color::Color) {
    unsafe { ffi::DrawSplineCatmullRom(points.as_ptr(), points.len() as c_int, thick, color) }
}

/// Draw spline: Quadratic Bezier, minimum 3 points
pub fn DrawSplineBezierQuadratic(points: &[math::Vector2], thick: f32, color: color::Color) {
    unsafe { ffi::DrawSplineBezierQuadratic(points.as_ptr(), points.len() as c_int, thick, color) }
}

/// Draw spline: Cubic Bezier, minimum 4 points
pub fn DrawSplineBezierCubic(points: &[math::Vector2], thick: f32, color: color::Color) {
    unsafe { ffi::DrawSplineBezierCubic(points.as_ptr(), points.len() as c_int, thick, color) }
}

/// Draw spline segment: Linear, 2 points
pub fn DrawSplineSegmentLinear(
    p1: math::Vector2,
    p2: math::Vector2,
    thick: f32,
    color: color::Color,
) {
    unsafe { ffi::DrawSplineSegmentLinear(p1, p2, thick, color) }
}

/// Draw spline segment: B-Spline, 4 points
pub fn DrawSplineSegmentBasis(
    p1: math::Vector2,
    p2: math::Vector2,
    p3: math::Vector2,
    p4: math::Vector2,
    thick: f32,
    color: color::Color,
) {
    unsafe { ffi::DrawSplineSegmentBasis(p1, p2, p3, p4, thick, color) }
}

/// Draw spline segment: Catmull-Rom, 4 points
pub fn DrawSplineSegmentCatmullRom(
    p1: math::Vector2,
    p2: math::Vector2,
    p3: math::Vector2,
    p4: math::Vector2,
    thick: f32,
    color: color::Color,
) {
    unsafe { ffi::DrawSplineSegmentCatmullRom(p1, p2, p3, p4, thick, color) }
}

/// Draw spline segment: Quadratic Bezier, 2 points, 1 control point
pub fn DrawSplineSegmentBezierQuadratic(
    p1: math::Vector2,
    c2: math::Vector2,
    p3: math::Vector2,
    thick: f32,
    color: color::Color,
) {
    unsafe { ffi::DrawSplineSegmentBezierQuadratic(p1, c2, p3, thick, color) }
}

/// Draw spline segment: Cubic Bezier, 2 points, 2 control points
pub fn DrawSplineSegmentBezierCubic(
    p1: math::Vector2,
    c2: math::Vector2,
    c3: math::Vector2,
    p4: math::Vector2,
    thick: f32,
    color: color::Color,
) {
    unsafe { ffi::DrawSplineSegmentBezierCubic(p1, c2, c3, p4, thick, color) }
}

/// Get (evaluate) spline point: Linear
pub fn GetSplinePointLinear(
    start_pos: math::Vector2,
    end_pos: math::Vector2,
    t: f32,
) -> math::Vector2 {
    unsafe { ffi::GetSplinePointLinear(start_pos, end_pos, t) }
}

/// Get (evaluate) spline point: B-Spline
pub fn GetSplinePointBasis(
    p1: math::Vector2,
    p2: math::Vector2,
    p3: math::Vector2,
    p4: math::Vector2,
    t: f32,
) -> math::Vector2 {
    unsafe { ffi::GetSplinePointBasis(p1, p2, p3, p4, t) }
}

/// Get (evaluate) spline point: Catmull-Rom
pub fn GetSplinePointCatmullRom(
    p1: math::Vector2,
    p2: math::Vector2,
    p3: math::Vector2,
    p4: math::Vector2,
    t: f32,
) -> math::Vector2 {
    unsafe { ffi::GetSplinePointCatmullRom(p1, p2, p3, p4, t) }
}

/// Get (evaluate) spline point: Quadratic Bezier
pub fn GetSplinePointBezierQuad(
    p1: math::Vector2,
    c2: math::Vector2,
    p3: math::Vector2,
    t: f32,
) -> math::Vector2 {
    unsafe { ffi::GetSplinePointBezierQuad(p1, c2, p3, t) }
}

/// Get (evaluate) spline point: Cubic Bezier
pub fn GetSplinePointBezierCubic(
    p1: math::Vector2,
    c2: math::Vector2,
    c3: math::Vector2,
    p4: math::Vector2,
    t: f32,
) -> math::Vector2 {
    unsafe { ffi::GetSplinePointBezierCubic(p1, c2, c3, p4, t) }
}

//------------------------------------------------------------------------------------
// Basic shapes collision detection functions
//------------------------------------------------------------------------------------

/// Check collision between two rectangles
pub fn CheckCollisionRecs(rec1: math::Rectangle, rec2: math::Rectangle) -> bool {
    unsafe { ffi::CheckCollisionRecs(rec1, rec2) }
}

/// Check collision between two circles
pub fn CheckCollisionCircles(
    center1: math::Vector2,
    radius1: f32,
    center2: math::Vector2,
    radius2: f32,
) -> bool {
    unsafe { ffi::CheckCollisionCircles(center1, radius1, center2, radius2) }
}

/// Check collision between circle and rectangle
pub fn CheckCollisionCircleRec(center: math::Vector2, radius: f32, rec: math::Rectangle) -> bool {
    unsafe { ffi::CheckCollisionCircleRec(center, radius, rec) }
}

/// Check if circle collides with a line
pub fn CheckCollisionCircleLine(
    center: math::Vector2,
    radius: f32,
    p1: math::Vector2,
    p2: math::Vector2,
) -> bool {
    unsafe { ffi::CheckCollisionCircleLine(center, radius, p1, p2) }
}

/// Check if point is inside rectangle
pub fn CheckCollisionPointRec(point: math::Vector2, rec: math::Rectangle) -> bool {
    unsafe { ffi::CheckCollisionPointRec(point, rec) }
}

/// Check if point is inside circle
pub fn CheckCollisionPointCircle(point: math::Vector2, center: math::Vector2, radius: f32) -> bool {
    unsafe { ffi::CheckCollisionPointCircle(point, center, radius) }
}

/// Check if point is inside a triangle
pub fn CheckCollisionPointTriangle(
    point: math::Vector2,
    p1: math::Vector2,
    p2: math::Vector2,
    p3: math::Vector2,
) -> bool {
    unsafe { ffi::CheckCollisionPointTriangle(point, p1, p2, p3) }
}

/// Check if point belongs to line with defined margin in pixels
pub fn CheckCollisionPointLine(
    point: math::Vector2,
    p1: math::Vector2,
    p2: math::Vector2,
    threshold: i32,
) -> bool {
    unsafe { ffi::CheckCollisionPointLine(point, p1, p2, threshold) }
}

/// Check if point is within a polygon described by array of vertices
pub fn CheckCollisionPointPoly(point: math::Vector2, points: &[math::Vector2]) -> bool {
    unsafe { ffi::CheckCollisionPointPoly(point, points.as_ptr(), points.len() as c_int) }
}

/// Check collision between two lines, returns collision point by reference
pub fn CheckCollisionLines(
    start_pos1: math::Vector2,
    end_pos1: math::Vector2,
    start_pos2: math::Vector2,
    end_pos2: math::Vector2,
    collision_point: &mut math::Vector2,
) -> bool {
    unsafe {
        ffi::CheckCollisionLines(
            start_pos1,
            end_pos1,
            start_pos2,
            end_pos2,
            collision_point as *mut _,
        )
    }
}

/// Get collision rectangle for two rectangles collision
pub fn GetCollisionRec(rec1: math::Rectangle, rec2: math::Rectangle) -> math::Rectangle {
    unsafe { ffi::GetCollisionRec(rec1, rec2) }
}

//------------------------------------------------------------------------------------
// Font and Text Functions (Module: text)
//------------------------------------------------------------------------------------

// Font loading/unloading functions

/// Get the default Font
pub fn GetFontDefault() -> Font {
    unsafe { ffi::GetFontDefault() }
}

/// Load font from file into GPU memory (VRAM)
pub fn LoadFont(file_name: &str) -> Font {
    let file_name_c = CString::new(file_name).expect("CString::new failed for file_name");
    unsafe { ffi::LoadFont(file_name_c.as_ptr()) }
}

/// Load font from file with extended parameters
pub fn LoadFontEx(file_name: &str, font_size: i32, codepoints: Option<&[i32]>) -> Font {
    let file_name_c = CString::new(file_name).expect("CString::new failed for file_name");
    let (codepoints_ptr, codepoints_count) = match codepoints {
        Some(slice) => (slice.as_ptr(), slice.len() as c_int),
        None => (std::ptr::null(), 0),
    };
    unsafe {
        ffi::LoadFontEx(
            file_name_c.as_ptr(),
            font_size,
            codepoints_ptr as *mut i32,
            codepoints_count,
        )
    }
}

/// Load font from Image (XNA style)
pub fn LoadFontFromImage(image: Image, key: color::Color, first_char: i32) -> Font {
    unsafe { ffi::LoadFontFromImage(image, key, first_char) }
}

/// Load font from memory buffer, fileType refers to extension: i.e. '.ttf'
pub fn LoadFontFromMemory(
    file_type: &str,
    file_data: &[u8],
    font_size: i32,
    codepoints: Option<&[i32]>,
) -> ffi::Font {
    let file_type_c = CString::new(file_type).expect("CString::new failed for file_type");
    let (codepoints_ptr, codepoints_count) = match codepoints {
        Some(slice) => (slice.as_ptr(), slice.len() as c_int),
        None => (std::ptr::null(), 0),
    };
    unsafe {
        ffi::LoadFontFromMemory(
            file_type_c.as_ptr(),
            file_data.as_ptr(),
            file_data.len() as c_int,
            font_size,
            codepoints_ptr as *mut i32,
            codepoints_count,
        )
    }
}

/// Check if a font is valid (font data loaded, WARNING: GPU texture not checked)
pub fn IsFontValid(font: ffi::Font) -> bool {
    unsafe { ffi::IsFontValid(font) }
}

/// Unload font from GPU memory (VRAM)
pub fn UnloadFont(font: ffi::Font) {
    unsafe { ffi::UnloadFont(font) }
}

/// Export font as code file, returns true on success
pub fn ExportFontAsCode(font: ffi::Font, file_name: &str) -> bool {
    let file_name_c = CString::new(file_name).expect("CString::new failed for file_name");
    unsafe { ffi::ExportFontAsCode(font, file_name_c.as_ptr()) }
}

// Text drawing functions

/// Draw current FPS
pub fn DrawFPS(pos_x: i32, pos_y: i32) {
    unsafe { ffi::DrawFPS(pos_x, pos_y) }
}

/// Draw text (using default font)
pub fn DrawText(text: &str, pos_x: i32, pos_y: i32, font_size: i32, color: color::Color) {
    let text_c = CString::new(text).expect("CString::new failed for text");
    unsafe { ffi::DrawText(text_c.as_ptr(), pos_x, pos_y, font_size, color) }
}

/// Draw text using font and additional parameters
pub fn DrawTextEx(
    font: ffi::Font,
    text: &str,
    position: math::Vector2,
    font_size: f32,
    spacing: f32,
    tint: color::Color,
) {
    let text_c = CString::new(text).expect("CString::new failed for text");
    unsafe { ffi::DrawTextEx(font, text_c.as_ptr(), position, font_size, spacing, tint) }
}

/// Draw text using Font and pro parameters (rotation)
pub fn DrawTextPro(
    font: ffi::Font,
    text: &str,
    position: math::Vector2,
    origin: math::Vector2,
    rotation: f32,
    font_size: f32,
    spacing: f32,
    tint: color::Color,
) {
    let text_c = CString::new(text).expect("CString::new failed for text");
    unsafe {
        ffi::DrawTextPro(
            font,
            text_c.as_ptr(),
            position,
            origin,
            rotation,
            font_size,
            spacing,
            tint,
        )
    }
}

/// Draw one character (codepoint)
pub fn DrawTextCodepoint(
    font: ffi::Font,
    codepoint: i32,
    position: math::Vector2,
    font_size: f32,
    tint: color::Color,
) {
    unsafe { ffi::DrawTextCodepoint(font, codepoint, position, font_size, tint) }
}

/// Draw multiple characters (codepoint)
pub fn DrawTextCodepoints(
    font: ffi::Font,
    codepoints: &[i32],
    position: math::Vector2,
    font_size: f32,
    spacing: f32,
    tint: color::Color,
) {
    unsafe {
        ffi::DrawTextCodepoints(
            font,
            codepoints.as_ptr(),
            codepoints.len() as c_int,
            position,
            font_size,
            spacing,
            tint,
        )
    }
}

// Text font info functions

/// Set vertical line spacing when drawing with line-breaks
pub fn SetTextLineSpacing(spacing: i32) {
    unsafe { ffi::SetTextLineSpacing(spacing) }
}

/// Measure string width for default font
pub fn MeasureText(text: &str, font_size: i32) -> i32 {
    let text_c = CString::new(text).expect("CString::new failed for text");
    unsafe { ffi::MeasureText(text_c.as_ptr(), font_size) }
}

/// Measure string size for Font
pub fn MeasureTextEx(font: ffi::Font, text: &str, font_size: f32, spacing: f32) -> math::Vector2 {
    let text_c = CString::new(text).expect("CString::new failed for text");
    unsafe { ffi::MeasureTextEx(font, text_c.as_ptr(), font_size, spacing) }
}

/// Get glyph index position in font for a codepoint (unicode character), fallback to '?' if not found
pub fn GetGlyphIndex(font: ffi::Font, codepoint: i32) -> i32 {
    unsafe { ffi::GetGlyphIndex(font, codepoint) }
}

// /// Get glyph font info data for a codepoint (unicode character), fallback to '?' if not found
// pub fn GetGlyphInfo(font: ffi::Font, codepoint: i32) -> GlyphInfo {
//     unsafe { ffi::GetGlyphInfo(font, codepoint) }
// }
//
/// Get glyph rectangle in font atlas for a codepoint (unicode character), fallback to '?' if not found
pub fn GetGlyphAtlasRec(font: ffi::Font, codepoint: i32) -> math::Rectangle {
    unsafe { ffi::GetGlyphAtlasRec(font, codepoint) }
}

// Text codepoints management functions (unicode characters)

/// Load all codepoints from a UTF-8 text string
pub fn LoadCodepoints(text: &str) -> Vec<i32> {
    let text_c = CString::new(text).expect("CString::new failed for text");
    let mut count: c_int = 0;
    unsafe {
        let codepoints_ptr = ffi::LoadCodepoints(text_c.as_ptr(), &mut count);
        let slice = std::slice::from_raw_parts(codepoints_ptr, count as usize);
        let result = slice.to_vec();
        ffi::UnloadCodepoints(codepoints_ptr);
        result
    }
}

/// Get total number of codepoints in a UTF-8 encoded string
pub fn GetCodepointCount(text: &str) -> i32 {
    let text_c = CString::new(text).expect("CString::new failed for text");
    unsafe { ffi::GetCodepointCount(text_c.as_ptr()) }
}

/// Get next codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
pub fn GetCodepoint(text: &str, codepoint_size: &mut i32) -> i32 {
    let text_c = CString::new(text).expect("CString::new failed for text");
    unsafe { ffi::GetCodepoint(text_c.as_ptr(), codepoint_size as *mut _) }
}

/// Encode one codepoint into UTF-8 byte array (array length returned as parameter)
pub fn CodepointToUTF8(codepoint: i32, utf8_size: &mut i32) -> &'static str {
    unsafe {
        let c_str = ffi::CodepointToUTF8(codepoint, utf8_size as *mut _);
        CStr::from_ptr(c_str)
            .to_str()
            .expect("Invalid UTF8 from CodepointToUTF8")
    }
}

// Text strings management functions

/// Check if two text string are equal
pub fn TextIsEqual(text1: &str, text2: &str) -> bool {
    let text1_c = CString::new(text1).expect("CString::new failed for text1");
    let text2_c = CString::new(text2).expect("CString::new failed for text2");
    unsafe { ffi::TextIsEqual(text1_c.as_ptr(), text2_c.as_ptr()) }
}

/// Get text length, checks for '\0' ending
pub fn TextLength(text: &str) -> u32 {
    let text_c = CString::new(text).expect("CString::new failed for text");
    unsafe { ffi::TextLength(text_c.as_ptr()) }
}

/// Get a piece of a text string
pub fn TextSubtext(text: &str, position: i32, length: i32) -> String {
    let text_c = CString::new(text).expect("CString::new failed for text");
    unsafe {
        let sub_ptr = ffi::TextSubtext(text_c.as_ptr(), position, length);
        CStr::from_ptr(sub_ptr).to_string_lossy().into_owned()
    }
}

/// Replace text string (allocates new memory)
pub fn TextReplace(text: &str, replace: &str, by: &str) -> String {
    let text_c = CString::new(text).expect("CString::new failed");
    let replace_c = CString::new(replace).expect("CString::new failed");
    let by_c = CString::new(by).expect("CString::new failed");
    unsafe {
        let result_ptr = ffi::TextReplace(text_c.as_ptr(), replace_c.as_ptr(), by_c.as_ptr());
        let result = CStr::from_ptr(result_ptr).to_string_lossy().into_owned();
        ffi::MemFree(result_ptr as *mut c_void); // Free the C-allocated memory
        result
    }
}

/// Insert text in a position (allocates new memory)
pub fn TextInsert(text: &str, insert: &str, position: i32) -> String {
    let text_c = CString::new(text).expect("CString::new failed");
    let insert_c = CString::new(insert).expect("CString::new failed");
    unsafe {
        let result_ptr = ffi::TextInsert(text_c.as_ptr(), insert_c.as_ptr(), position);
        let result = CStr::from_ptr(result_ptr).to_string_lossy().into_owned();
        ffi::MemFree(result_ptr as *mut c_void); // Free the C-allocated memory
        result
    }
}

/// Join text strings with delimiter
pub fn TextJoin(text_list: &[&str], delimiter: &str) -> String {
    let c_strings: Vec<CString> = text_list
        .iter()
        .map(|&s| CString::new(s).unwrap())
        .collect();
    let c_pointers: Vec<*const c_char> = c_strings.iter().map(|cs| cs.as_ptr()).collect();
    let delimiter_c = CString::new(delimiter).expect("CString::new failed");
    unsafe {
        let result_ptr = ffi::TextJoin(
            c_pointers.as_ptr() as *mut *const i8,
            c_pointers.len() as c_int,
            delimiter_c.as_ptr(),
        );
        CStr::from_ptr(result_ptr).to_string_lossy().into_owned()
    }
}

/// Split text into multiple strings
pub fn TextSplit(text: &str, delimiter: char) -> Vec<String> {
    let text_c = CString::new(text).expect("CString::new failed");
    let mut count: c_int = 0;
    unsafe {
        let result_ptr = ffi::TextSplit(text_c.as_ptr(), delimiter as c_char, &mut count);
        let mut result_vec = Vec::with_capacity(count as usize);
        for i in 0..(count as isize) {
            let item_ptr = *result_ptr.offset(i);
            let item_str = CStr::from_ptr(item_ptr).to_string_lossy().into_owned();
            result_vec.push(item_str);
        }
        // The list of pointers is allocated by Raylib and should be freed
        ffi::MemFree(result_ptr as *mut c_void);
        result_vec
    }
}

/// Find first text occurrence within a string
pub fn TextFindIndex(text: &str, find: &str) -> i32 {
    let text_c = CString::new(text).expect("CString::new failed");
    let find_c = CString::new(find).expect("CString::new failed");
    unsafe { ffi::TextFindIndex(text_c.as_ptr(), find_c.as_ptr()) }
}

/// Get upper case version of provided string
pub fn TextToUpper(text: &str) -> String {
    let text_c = CString::new(text).expect("CString::new failed");
    unsafe {
        CStr::from_ptr(ffi::TextToUpper(text_c.as_ptr()))
            .to_string_lossy()
            .into_owned()
    }
}

/// Get lower case version of provided string
pub fn TextToLower(text: &str) -> String {
    let text_c = CString::new(text).expect("CString::new failed");
    unsafe {
        CStr::from_ptr(ffi::TextToLower(text_c.as_ptr()))
            .to_string_lossy()
            .into_owned()
    }
}

/// Get Pascal case notation version of provided string
pub fn TextToPascal(text: &str) -> String {
    let text_c = CString::new(text).expect("CString::new failed");
    unsafe {
        CStr::from_ptr(ffi::TextToPascal(text_c.as_ptr()))
            .to_string_lossy()
            .into_owned()
    }
}

/// Get integer value from text
pub fn TextToInteger(text: &str) -> i32 {
    let text_c = CString::new(text).expect("CString::new failed");
    unsafe { ffi::TextToInteger(text_c.as_ptr()) }
}

//----------------------------------------------------------------------------------
// Module: rmath - Utils math
//----------------------------------------------------------------------------------

/// Clamp float value
pub fn Clamp(value: f32, min: f32, max: f32) -> f32 {
    unsafe { ffi::Clamp(value, min, max) }
}

/// Calculate linear interpolation between two floats
pub fn Lerp(start: f32, end: f32, amount: f32) -> f32 {
    unsafe { ffi::Lerp(start, end, amount) }
}

/// Normalize input value within input range
pub fn Normalize(value: f32, start: f32, end: f32) -> f32 {
    unsafe { ffi::Normalize(value, start, end) }
}

/// Remap input value within input range to output range
pub fn Remap(
    value: f32,
    input_start: f32,
    input_end: f32,
    output_start: f32,
    output_end: f32,
) -> f32 {
    unsafe { ffi::Remap(value, input_start, input_end, output_start, output_end) }
}

/// Wrap input value from min to max
pub fn Wrap(value: f32, min: f32, max: f32) -> f32 {
    unsafe { ffi::Wrap(value, min, max) }
}

/// Check whether two given floats are almost equal
pub fn FloatEquals(x: f32, y: f32) -> bool {
    unsafe { ffi::FloatEquals(x, y) != 0 }
}

//----------------------------------------------------------------------------------
// Module: rmath - Vector2 math
//----------------------------------------------------------------------------------

/// Vector with components value 0.0f
pub fn Vector2Zero() -> math::Vector2 {
    unsafe { ffi::Vector2Zero() }
}

/// Vector with components value 1.0f
pub fn Vector2One() -> math::Vector2 {
    unsafe { ffi::Vector2One() }
}

/// Add two vectors (v1 + v2)
pub fn Vector2Add(v1: math::Vector2, v2: math::Vector2) -> math::Vector2 {
    unsafe { ffi::Vector2Add(v1, v2) }
}

/// Add vector and float value
pub fn Vector2AddValue(v: math::Vector2, add: f32) -> math::Vector2 {
    unsafe { ffi::Vector2AddValue(v, add) }
}

/// Subtract two vectors (v1 - v2)
pub fn Vector2Subtract(v1: math::Vector2, v2: math::Vector2) -> math::Vector2 {
    unsafe { ffi::Vector2Subtract(v1, v2) }
}

/// Subtract vector by float value
pub fn Vector2SubtractValue(v: math::Vector2, sub: f32) -> math::Vector2 {
    unsafe { ffi::Vector2SubtractValue(v, sub) }
}

/// Calculate vector length
pub fn Vector2Length(v: math::Vector2) -> f32 {
    unsafe { ffi::Vector2Length(v) }
}

/// Calculate vector square length
pub fn Vector2LengthSqr(v: math::Vector2) -> f32 {
    unsafe { ffi::Vector2LengthSqr(v) }
}

/// Calculate two vectors dot product
pub fn Vector2DotProduct(v1: math::Vector2, v2: math::Vector2) -> f32 {
    unsafe { ffi::Vector2DotProduct(v1, v2) }
}

/// Calculate distance between two vectors
pub fn Vector2Distance(v1: math::Vector2, v2: math::Vector2) -> f32 {
    unsafe { ffi::Vector2Distance(v1, v2) }
}

/// Calculate square distance between two vectors
pub fn Vector2DistanceSqr(v1: math::Vector2, v2: math::Vector2) -> f32 {
    unsafe { ffi::Vector2DistanceSqr(v1, v2) }
}

/// Calculate angle from two vectors
pub fn Vector2Angle(v1: math::Vector2, v2: math::Vector2) -> f32 {
    unsafe { ffi::Vector2Angle(v1, v2) }
}

/// Scale vector (multiply by value)
pub fn Vector2Scale(v: math::Vector2, scale: f32) -> math::Vector2 {
    unsafe { ffi::Vector2Scale(v, scale) }
}

/// Multiply vector by vector
pub fn Vector2Multiply(v1: math::Vector2, v2: math::Vector2) -> math::Vector2 {
    unsafe { ffi::Vector2Multiply(v1, v2) }
}

/// Negate vector
pub fn Vector2Negate(v: math::Vector2) -> math::Vector2 {
    unsafe { ffi::Vector2Negate(v) }
}

/// Divide vector by vector
pub fn Vector2Divide(v1: math::Vector2, v2: math::Vector2) -> math::Vector2 {
    unsafe { ffi::Vector2Divide(v1, v2) }
}

/// Normalize provided vector
pub fn Vector2Normalize(v: math::Vector2) -> math::Vector2 {
    unsafe { ffi::Vector2Normalize(v) }
}

/// Transforms a Vector2 by a given Matrix
pub fn Vector2Transform(v: math::Vector2, mat: math::Matrix) -> math::Vector2 {
    unsafe { ffi::Vector2Transform(v, mat) }
}

/// Calculate linear interpolation between two vectors
pub fn Vector2Lerp(v1: math::Vector2, v2: math::Vector2, amount: f32) -> math::Vector2 {
    unsafe { ffi::Vector2Lerp(v1, v2, amount) }
}

/// Calculate reflected vector to normal
pub fn Vector2Reflect(v: math::Vector2, normal: math::Vector2) -> math::Vector2 {
    unsafe { ffi::Vector2Reflect(v, normal) }
}

/// Rotate vector by angle
pub fn Vector2Rotate(v: math::Vector2, angle: f32) -> math::Vector2 {
    unsafe { ffi::Vector2Rotate(v, angle) }
}

/// Move Vector towards target
pub fn Vector2MoveTowards(
    v: math::Vector2,
    target: math::Vector2,
    max_distance: f32,
) -> math::Vector2 {
    unsafe { ffi::Vector2MoveTowards(v, target, max_distance) }
}

/// Invert the given vector
pub fn Vector2Invert(v: math::Vector2) -> math::Vector2 {
    unsafe { ffi::Vector2Invert(v) }
}

/// Clamp the components of the vector between min and max values specified by the given vectors
pub fn Vector2Clamp(v: math::Vector2, min: math::Vector2, max: math::Vector2) -> math::Vector2 {
    unsafe { ffi::Vector2Clamp(v, min, max) }
}

/// Clamp the magnitude of the vector between two min and max values
pub fn Vector2ClampValue(v: math::Vector2, min: f32, max: f32) -> math::Vector2 {
    unsafe { ffi::Vector2ClampValue(v, min, max) }
}

/// Check whether two given vectors are almost equal
pub fn Vector2Equals(p: math::Vector2, q: math::Vector2) -> bool {
    unsafe { ffi::Vector2Equals(p, q) != 0 }
}

//----------------------------------------------------------------------------------
// Module: rmath - Vector3 math
//----------------------------------------------------------------------------------

/// Vector with components value 0.0f
pub fn Vector3Zero() -> math::Vector3 {
    unsafe { ffi::Vector3Zero() }
}

/// Vector with components value 1.0f
pub fn Vector3One() -> math::Vector3 {
    unsafe { ffi::Vector3One() }
}

/// Add two vectors
pub fn Vector3Add(v1: math::Vector3, v2: math::Vector3) -> math::Vector3 {
    unsafe { ffi::Vector3Add(v1, v2) }
}

/// Add vector and float value
pub fn Vector3AddValue(v: math::Vector3, add: f32) -> math::Vector3 {
    unsafe { ffi::Vector3AddValue(v, add) }
}

/// Subtract two vectors
pub fn Vector3Subtract(v1: math::Vector3, v2: math::Vector3) -> math::Vector3 {
    unsafe { ffi::Vector3Subtract(v1, v2) }
}

/// Subtract vector by float value
pub fn Vector3SubtractValue(v: math::Vector3, sub: f32) -> math::Vector3 {
    unsafe { ffi::Vector3SubtractValue(v, sub) }
}

/// Multiply vector by scalar
pub fn Vector3Scale(v: math::Vector3, scalar: f32) -> math::Vector3 {
    unsafe { ffi::Vector3Scale(v, scalar) }
}

/// Multiply vector by vector
pub fn Vector3Multiply(v1: math::Vector3, v2: math::Vector3) -> math::Vector3 {
    unsafe { ffi::Vector3Multiply(v1, v2) }
}

/// Calculate two vectors cross product
pub fn Vector3CrossProduct(v1: math::Vector3, v2: math::Vector3) -> math::Vector3 {
    unsafe { ffi::Vector3CrossProduct(v1, v2) }
}

/// Calculate one vector perpendicular vector
pub fn Vector3Perpendicular(v: math::Vector3) -> math::Vector3 {
    unsafe { ffi::Vector3Perpendicular(v) }
}

/// Calculate vector length
pub fn Vector3Length(v: math::Vector3) -> f32 {
    unsafe { ffi::Vector3Length(v) }
}

/// Calculate vector square length
pub fn Vector3LengthSqr(v: math::Vector3) -> f32 {
    unsafe { ffi::Vector3LengthSqr(v) }
}

/// Calculate two vectors dot product
pub fn Vector3DotProduct(v1: math::Vector3, v2: math::Vector3) -> f32 {
    unsafe { ffi::Vector3DotProduct(v1, v2) }
}

/// Calculate distance between two vectors
pub fn Vector3Distance(v1: math::Vector3, v2: math::Vector3) -> f32 {
    unsafe { ffi::Vector3Distance(v1, v2) }
}

/// Calculate square distance between two vectors
pub fn Vector3DistanceSqr(v1: math::Vector3, v2: math::Vector3) -> f32 {
    unsafe { ffi::Vector3DistanceSqr(v1, v2) }
}

/// Calculate angle between two vectors
pub fn Vector3Angle(v1: math::Vector3, v2: math::Vector3) -> f32 {
    unsafe { ffi::Vector3Angle(v1, v2) }
}

/// Negate provided vector (invert direction)
pub fn Vector3Negate(v: math::Vector3) -> math::Vector3 {
    unsafe { ffi::Vector3Negate(v) }
}

/// Divide vector by vector
pub fn Vector3Divide(v1: math::Vector3, v2: math::Vector3) -> math::Vector3 {
    unsafe { ffi::Vector3Divide(v1, v2) }
}

/// Normalize provided vector
pub fn Vector3Normalize(v: math::Vector3) -> math::Vector3 {
    unsafe { ffi::Vector3Normalize(v) }
}

/// Orthonormalize provided vectors
pub fn Vector3OrthoNormalize(v1: &mut math::Vector3, v2: &mut math::Vector3) {
    unsafe { ffi::Vector3OrthoNormalize(v1 as *mut _, v2 as *mut _) }
}

/// Transforms a Vector3 by a given Matrix
pub fn Vector3Transform(v: math::Vector3, mat: math::Matrix) -> math::Vector3 {
    unsafe { ffi::Vector3Transform(v, mat) }
}

/// Transform a vector by quaternion rotation
pub fn Vector3RotateByQuaternion(v: math::Vector3, q: math::Quaternion) -> math::Vector3 {
    unsafe { ffi::Vector3RotateByQuaternion(v, q) }
}

/// Rotates a vector around an axis
pub fn Vector3RotateByAxisAngle(
    v: math::Vector3,
    axis: math::Vector3,
    angle: f32,
) -> math::Vector3 {
    unsafe { ffi::Vector3RotateByAxisAngle(v, axis, angle) }
}

/// Calculate linear interpolation between two vectors
pub fn Vector3Lerp(v1: math::Vector3, v2: math::Vector3, amount: f32) -> math::Vector3 {
    unsafe { ffi::Vector3Lerp(v1, v2, amount) }
}

/// Calculate reflected vector to normal
pub fn Vector3Reflect(v: math::Vector3, normal: math::Vector3) -> math::Vector3 {
    unsafe { ffi::Vector3Reflect(v, normal) }
}

/// Get min value for each pair of components
pub fn Vector3Min(v1: math::Vector3, v2: math::Vector3) -> math::Vector3 {
    unsafe { ffi::Vector3Min(v1, v2) }
}

/// Get max value for each pair of components
pub fn Vector3Max(v1: math::Vector3, v2: math::Vector3) -> math::Vector3 {
    unsafe { ffi::Vector3Max(v1, v2) }
}

/// Compute barycenter coordinates for point p with respect to triangle (a, b, c)
pub fn Vector3Barycenter(
    p: math::Vector3,
    a: math::Vector3,
    b: math::Vector3,
    c: math::Vector3,
) -> math::Vector3 {
    unsafe { ffi::Vector3Barycenter(p, a, b, c) }
}

/// Projects a Vector3 from screen space into object space
pub fn Vector3Unproject(
    source: math::Vector3,
    projection: math::Matrix,
    view: math::Matrix,
) -> math::Vector3 {
    unsafe { ffi::Vector3Unproject(source, projection, view) }
}

/// Get Vector3 as float array
pub fn Vector3ToFloatV(v: math::Vector3) -> float3 {
    unsafe { ffi::Vector3ToFloatV(v) }
}

/// Invert the given vector
pub fn Vector3Invert(v: math::Vector3) -> math::Vector3 {
    unsafe { ffi::Vector3Invert(v) }
}

/// Clamp the components of the vector between min and max values specified by the given vectors
pub fn Vector3Clamp(v: math::Vector3, min: math::Vector3, max: math::Vector3) -> math::Vector3 {
    unsafe { ffi::Vector3Clamp(v, min, max) }
}

/// Clamp the magnitude of the vector between two values
pub fn Vector3ClampValue(v: math::Vector3, min: f32, max: f32) -> math::Vector3 {
    unsafe { ffi::Vector3ClampValue(v, min, max) }
}

/// Check whether two given vectors are almost equal
pub fn Vector3Equals(p: math::Vector3, q: math::Vector3) -> bool {
    unsafe { ffi::Vector3Equals(p, q) != 0 }
}

/// Compute the direction of a refracted ray
pub fn Vector3Refract(v: math::Vector3, n: math::Vector3, r: f32) -> math::Vector3 {
    unsafe { ffi::Vector3Refract(v, n, r) }
}

//----------------------------------------------------------------------------------
// Module: rmath - Matrix math
//----------------------------------------------------------------------------------

/// Compute matrix determinant
pub fn MatrixDeterminant(mat: math::Matrix) -> f32 {
    unsafe { ffi::MatrixDeterminant(mat) }
}

/// Get the trace of the matrix
pub fn MatrixTrace(mat: math::Matrix) -> f32 {
    unsafe { ffi::MatrixTrace(mat) }
}

/// Transposes provided matrix
pub fn MatrixTranspose(mat: math::Matrix) -> math::Matrix {
    unsafe { ffi::MatrixTranspose(mat) }
}

/// Invert provided matrix
pub fn MatrixInvert(mat: math::Matrix) -> math::Matrix {
    unsafe { ffi::MatrixInvert(mat) }
}

/// Get identity matrix
pub fn MatrixIdentity() -> math::Matrix {
    unsafe { ffi::MatrixIdentity() }
}

/// Add two matrices
pub fn MatrixAdd(left: math::Matrix, right: math::Matrix) -> math::Matrix {
    unsafe { ffi::MatrixAdd(left, right) }
}

/// Subtract two matrices (left - right)
pub fn MatrixSubtract(left: math::Matrix, right: math::Matrix) -> math::Matrix {
    unsafe { ffi::MatrixSubtract(left, right) }
}

/// Get two matrix multiplication
pub fn MatrixMultiply(left: math::Matrix, right: math::Matrix) -> math::Matrix {
    unsafe { ffi::MatrixMultiply(left, right) }
}

/// Get translation matrix
pub fn MatrixTranslate(x: f32, y: f32, z: f32) -> math::Matrix {
    unsafe { ffi::MatrixTranslate(x, y, z) }
}

/// Create rotation matrix from axis and angle (in radians)
pub fn MatrixRotate(axis: math::Vector3, angle: f32) -> math::Matrix {
    unsafe { ffi::MatrixRotate(axis, angle) }
}

/// Get x-rotation matrix (angle in radians)
pub fn MatrixRotateX(angle: f32) -> math::Matrix {
    unsafe { ffi::MatrixRotateX(angle) }
}

/// Get y-rotation matrix (angle in radians)
pub fn MatrixRotateY(angle: f32) -> math::Matrix {
    unsafe { ffi::MatrixRotateY(angle) }
}

/// Get z-rotation matrix (angle in radians)
pub fn MatrixRotateZ(angle: f32) -> math::Matrix {
    unsafe { ffi::MatrixRotateZ(angle) }
}

/// Get xyz-rotation matrix (angles in radians)
pub fn MatrixRotateXYZ(angle: math::Vector3) -> math::Matrix {
    unsafe { ffi::MatrixRotateXYZ(angle) }
}

/// Get zyx-rotation matrix (angles in radians)
pub fn MatrixRotateZYX(angle: math::Vector3) -> math::Matrix {
    unsafe { ffi::MatrixRotateZYX(angle) }
}

/// Get scaling matrix
pub fn MatrixScale(x: f32, y: f32, z: f32) -> math::Matrix {
    unsafe { ffi::MatrixScale(x, y, z) }
}

/// Get perspective projection matrix
pub fn MatrixFrustum(
    left: f64,
    right: f64,
    bottom: f64,
    top: f64,
    near: f64,
    far: f64,
) -> math::Matrix {
    unsafe { ffi::MatrixFrustum(left, right, bottom, top, near, far) }
}

/// Get perspective projection matrix (fovy angle in radians)
pub fn MatrixPerspective(fovy: f64, aspect: f64, near: f64, far: f64) -> math::Matrix {
    unsafe { ffi::MatrixPerspective(fovy, aspect, near, far) }
}

/// Get orthographic projection matrix
pub fn MatrixOrtho(
    left: f64,
    right: f64,
    bottom: f64,
    top: f64,
    near: f64,
    far: f64,
) -> math::Matrix {
    unsafe { ffi::MatrixOrtho(left, right, bottom, top, near, far) }
}

/// Get camera look-at matrix (view matrix)
pub fn MatrixLookAt(eye: math::Vector3, target: math::Vector3, up: math::Vector3) -> math::Matrix {
    unsafe { ffi::MatrixLookAt(eye, target, up) }
}

/// Get float array of matrix data
pub fn MatrixToFloatV(mat: math::Matrix) -> float16 {
    unsafe { ffi::MatrixToFloatV(mat) }
}

//----------------------------------------------------------------------------------
// Module: rmath - Quaternion math
//----------------------------------------------------------------------------------

/// Add two quaternions
pub fn QuaternionAdd(q1: math::Quaternion, q2: math::Quaternion) -> math::Quaternion {
    unsafe { ffi::QuaternionAdd(q1, q2) }
}

/// Add quaternion and float value
pub fn QuaternionAddValue(q: math::Quaternion, add: f32) -> math::Quaternion {
    unsafe { ffi::QuaternionAddValue(q, add) }
}

/// Subtract two quaternions
pub fn QuaternionSubtract(q1: math::Quaternion, q2: math::Quaternion) -> math::Quaternion {
    unsafe { ffi::QuaternionSubtract(q1, q2) }
}

/// Subtract quaternion and float value
pub fn QuaternionSubtractValue(q: math::Quaternion, sub: f32) -> math::Quaternion {
    unsafe { ffi::QuaternionSubtractValue(q, sub) }
}

/// Get identity quaternion
pub fn QuaternionIdentity() -> math::Quaternion {
    unsafe { ffi::QuaternionIdentity() }
}

/// Computes the length of a quaternion
pub fn QuaternionLength(q: math::Quaternion) -> f32 {
    unsafe { ffi::QuaternionLength(q) }
}

/// Normalize provided quaternion
pub fn QuaternionNormalize(q: math::Quaternion) -> math::Quaternion {
    unsafe { ffi::QuaternionNormalize(q) }
}

/// Invert provided quaternion
pub fn QuaternionInvert(q: math::Quaternion) -> math::Quaternion {
    unsafe { ffi::QuaternionInvert(q) }
}

/// Calculate two quaternion multiplication
pub fn QuaternionMultiply(q1: math::Quaternion, q2: math::Quaternion) -> math::Quaternion {
    unsafe { ffi::QuaternionMultiply(q1, q2) }
}

/// Scale quaternion by float value
pub fn QuaternionScale(q: math::Quaternion, mul: f32) -> math::Quaternion {
    unsafe { ffi::QuaternionScale(q, mul) }
}

/// Divide two quaternions
pub fn QuaternionDivide(q1: math::Quaternion, q2: math::Quaternion) -> math::Quaternion {
    unsafe { ffi::QuaternionDivide(q1, q2) }
}

/// Calculate linear interpolation between two quaternions
pub fn QuaternionLerp(q1: math::Quaternion, q2: math::Quaternion, amount: f32) -> math::Quaternion {
    unsafe { ffi::QuaternionLerp(q1, q2, amount) }
}

/// Calculate slerp-optimized interpolation between two quaternions
pub fn QuaternionNlerp(
    q1: math::Quaternion,
    q2: math::Quaternion,
    amount: f32,
) -> math::Quaternion {
    unsafe { ffi::QuaternionNlerp(q1, q2, amount) }
}

/// Calculates spherical linear interpolation between two quaternions
pub fn QuaternionSlerp(
    q1: math::Quaternion,
    q2: math::Quaternion,
    amount: f32,
) -> math::Quaternion {
    unsafe { ffi::QuaternionSlerp(q1, q2, amount) }
}

/// Calculate quaternion based on the rotation from one vector to another
pub fn QuaternionFromVector3ToVector3(from: math::Vector3, to: math::Vector3) -> math::Quaternion {
    unsafe { ffi::QuaternionFromVector3ToVector3(from, to) }
}

/// Get a quaternion for a given rotation matrix
pub fn QuaternionFromMatrix(mat: math::Matrix) -> math::Quaternion {
    unsafe { ffi::QuaternionFromMatrix(mat) }
}

/// Get a matrix for a given quaternion
pub fn QuaternionToMatrix(q: math::Quaternion) -> math::Matrix {
    unsafe { ffi::QuaternionToMatrix(q) }
}

/// Get rotation quaternion for an angle and axis (angle in radians)
pub fn QuaternionFromAxisAngle(axis: math::Vector3, angle: f32) -> math::Quaternion {
    unsafe { ffi::QuaternionFromAxisAngle(axis, angle) }
}

/// Get the rotation angle and axis for a given quaternion
pub fn QuaternionToAxisAngle(q: math::Quaternion) -> (math::Vector3, f32) {
    let mut out_axis = math::Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let mut out_angle = 0.0f32;
    unsafe {
        ffi::QuaternionToAxisAngle(q, &mut out_axis as *mut _, &mut out_angle as *mut _);
    }
    (out_axis, out_angle)
}

/// Get the quaternion equivalent to Euler angles (ZYX rotation order)
pub fn QuaternionFromEuler(pitch: f32, yaw: f32, roll: f32) -> math::Quaternion {
    unsafe { ffi::QuaternionFromEuler(pitch, yaw, roll) }
}

/// Get the Euler angles equivalent to quaternion (roll, pitch, yaw) in radians
pub fn QuaternionToEuler(q: math::Quaternion) -> math::Vector3 {
    unsafe { ffi::QuaternionToEuler(q) }
}

/// Transform a quaternion given a transformation matrix
pub fn QuaternionTransform(q: math::Quaternion, mat: math::Matrix) -> math::Quaternion {
    unsafe { ffi::QuaternionTransform(q, mat) }
}

/// Check whether two given quaternions are almost equal
pub fn QuaternionEquals(p: math::Quaternion, q: math::Quaternion) -> bool {
    unsafe { ffi::QuaternionEquals(p, q) != 0 }
}
