use std::ffi::{CStr, CString, c_char};
use std::os::raw::{c_int, c_void};

use crate::color;
use crate::ffi;
use crate::math;

use crate::ffi::{
    AudioStream, BoundingBox, Camera, Camera2D, Camera3D, Font, Image, KeyboardKey, Material, Mesh,
    Model, ModelAnimation, Music, NPatchInfo, Ray, RayCollision, RenderTexture2D, Shader, Sound,
    Texture2D, TextureCubemap, VrDeviceInfo, VrStereoConfig, Wave, float3, float16,
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
// Image loading functions
//------------------------------------------------------------------------------------

/// Load image from file into CPU memory (RAM)
pub fn LoadImage(file_name: &str) -> Image {
    let file_name_c = CString::new(file_name).expect("CString::new failed");
    unsafe { ffi::LoadImage(file_name_c.as_ptr()) }
}

/// Load image from RAW file data
pub fn LoadImageRaw(
    file_name: &str,
    width: i32,
    height: i32,
    format: i32,
    header_size: i32,
) -> Image {
    let file_name_c = CString::new(file_name).expect("CString::new failed");
    unsafe { ffi::LoadImageRaw(file_name_c.as_ptr(), width, height, format, header_size) }
}

/// Load image sequence from file (frames appended to image.data)
pub fn LoadImageAnim(file_name: &str) -> (Image, i32) {
    let file_name_c = CString::new(file_name).expect("CString::new failed");
    let mut frames = 0;
    let image = unsafe { ffi::LoadImageAnim(file_name_c.as_ptr(), &mut frames) };
    (image, frames)
}

/// Load image sequence from memory buffer
pub fn LoadImageAnimFromMemory(file_type: &str, file_data: &[u8]) -> (Image, i32) {
    let file_type_c = CString::new(file_type).expect("CString::new failed");
    let mut frames = 0;
    let image = unsafe {
        ffi::LoadImageAnimFromMemory(
            file_type_c.as_ptr(),
            file_data.as_ptr(),
            file_data.len() as c_int,
            &mut frames,
        )
    };
    (image, frames)
}

/// Load image from memory buffer, fileType refers to extension: i.e. '.png'
pub fn LoadImageFromMemory(file_type: &str, file_data: &[u8]) -> Image {
    let file_type_c = CString::new(file_type).expect("CString::new failed");
    unsafe {
        ffi::LoadImageFromMemory(
            file_type_c.as_ptr(),
            file_data.as_ptr(),
            file_data.len() as c_int,
        )
    }
}

/// Load image from GPU texture data
pub fn LoadImageFromTexture(texture: Texture2D) -> Image {
    unsafe { ffi::LoadImageFromTexture(texture) }
}

/// Load image from screen buffer (screenshot)
pub fn LoadImageFromScreen() -> Image {
    unsafe { ffi::LoadImageFromScreen() }
}

/// Check if an image is valid (data and parameters)
pub fn IsImageValid(image: Image) -> bool {
    unsafe { ffi::IsImageValid(image) }
}

/// Unload image from CPU memory (RAM)
pub fn UnloadImage(image: Image) {
    unsafe { ffi::UnloadImage(image) }
}

/// Export image data to file, returns true on success
pub fn ExportImage(image: Image, file_name: &str) -> bool {
    let file_name_c = CString::new(file_name).expect("CString::new failed");
    unsafe { ffi::ExportImage(image, file_name_c.as_ptr()) }
}

/// Export image to memory buffer
pub fn ExportImageToMemory(image: Image, file_type: &str) -> Vec<u8> {
    let file_type_c = CString::new(file_type).expect("CString::new failed");
    let mut file_size = 0;
    unsafe {
        let data_ptr = ffi::ExportImageToMemory(image, file_type_c.as_ptr(), &mut file_size);
        let slice = std::slice::from_raw_parts(data_ptr, file_size as usize);
        let vec = slice.to_vec();
        ffi::MemFree(data_ptr as *mut c_void); // Free the C-allocated memory
        vec
    }
}

/// Export image as code file defining an array of bytes, returns true on success
pub fn ExportImageAsCode(image: Image, file_name: &str) -> bool {
    let file_name_c = CString::new(file_name).expect("CString::new failed");
    unsafe { ffi::ExportImageAsCode(image, file_name_c.as_ptr()) }
}

//------------------------------------------------------------------------------------
// Image generation functions
//------------------------------------------------------------------------------------

/// Generate image: plain color
pub fn GenImageColor(width: i32, height: i32, color: color::Color) -> Image {
    unsafe { ffi::GenImageColor(width, height, color) }
}

/// Generate image: linear gradient
pub fn GenImageGradientLinear(
    width: i32,
    height: i32,
    direction: i32,
    start: color::Color,
    end: color::Color,
) -> Image {
    unsafe { ffi::GenImageGradientLinear(width, height, direction, start, end) }
}

/// Generate image: radial gradient
pub fn GenImageGradientRadial(
    width: i32,
    height: i32,
    density: f32,
    inner: color::Color,
    outer: color::Color,
) -> Image {
    unsafe { ffi::GenImageGradientRadial(width, height, density, inner, outer) }
}

/// Generate image: square gradient
pub fn GenImageGradientSquare(
    width: i32,
    height: i32,
    density: f32,
    inner: color::Color,
    outer: color::Color,
) -> Image {
    unsafe { ffi::GenImageGradientSquare(width, height, density, inner, outer) }
}

/// Generate image: checked
pub fn GenImageChecked(
    width: i32,
    height: i32,
    checks_x: i32,
    checks_y: i32,
    col1: color::Color,
    col2: color::Color,
) -> Image {
    unsafe { ffi::GenImageChecked(width, height, checks_x, checks_y, col1, col2) }
}

/// Generate image: white noise
pub fn GenImageWhiteNoise(width: i32, height: i32, factor: f32) -> Image {
    unsafe { ffi::GenImageWhiteNoise(width, height, factor) }
}

/// Generate image: perlin noise
pub fn GenImagePerlinNoise(
    width: i32,
    height: i32,
    offset_x: i32,
    offset_y: i32,
    scale: f32,
) -> Image {
    unsafe { ffi::GenImagePerlinNoise(width, height, offset_x, offset_y, scale) }
}

/// Generate image: cellular algorithm
pub fn GenImageCellular(width: i32, height: i32, tile_size: i32) -> Image {
    unsafe { ffi::GenImageCellular(width, height, tile_size) }
}

/// Generate image: grayscale image from text data
pub fn GenImageText(width: i32, height: i32, text: &str) -> Image {
    let text_c = CString::new(text).expect("CString::new failed");
    unsafe { ffi::GenImageText(width, height, text_c.as_ptr()) }
}

//------------------------------------------------------------------------------------
// Image manipulation functions
//------------------------------------------------------------------------------------

/// Create an image duplicate (useful for transformations)
pub fn ImageCopy(image: Image) -> Image {
    unsafe { ffi::ImageCopy(image) }
}

/// Create an image from another image piece
pub fn ImageFromImage(image: Image, rec: math::Rectangle) -> Image {
    unsafe { ffi::ImageFromImage(image, rec) }
}

/// Create an image from a selected channel of another image (GRAYSCALE)
pub fn ImageFromChannel(image: Image, selected_channel: i32) -> Image {
    unsafe { ffi::ImageFromChannel(image, selected_channel) }
}

/// Create an image from text (default font)
pub fn ImageText(text: &str, font_size: i32, color: color::Color) -> Image {
    let text_c = CString::new(text).expect("CString::new failed");
    unsafe { ffi::ImageText(text_c.as_ptr(), font_size, color) }
}

/// Create an image from text (custom sprite font)
pub fn ImageTextEx(
    font: Font,
    text: &str,
    font_size: f32,
    spacing: f32,
    tint: color::Color,
) -> Image {
    let text_c = CString::new(text).expect("CString::new failed");
    unsafe { ffi::ImageTextEx(font, text_c.as_ptr(), font_size, spacing, tint) }
}

/// Convert image data to desired format
pub fn ImageFormat(image: &mut Image, new_format: i32) {
    unsafe { ffi::ImageFormat(image, new_format) }
}

/// Convert image to POT (power-of-two)
pub fn ImageToPOT(image: &mut Image, fill: color::Color) {
    unsafe { ffi::ImageToPOT(image, fill) }
}

/// Crop an image to a defined rectangle
pub fn ImageCrop(image: &mut Image, crop: math::Rectangle) {
    unsafe { ffi::ImageCrop(image, crop) }
}

/// Crop image depending on alpha value
pub fn ImageAlphaCrop(image: &mut Image, threshold: f32) {
    unsafe { ffi::ImageAlphaCrop(image, threshold) }
}

/// Clear alpha channel to desired color
pub fn ImageAlphaClear(image: &mut Image, color: color::Color, threshold: f32) {
    unsafe { ffi::ImageAlphaClear(image, color, threshold) }
}

/// Apply alpha mask to image
pub fn ImageAlphaMask(image: &mut Image, alpha_mask: Image) {
    unsafe { ffi::ImageAlphaMask(image, alpha_mask) }
}

/// Premultiply alpha channel
pub fn ImageAlphaPremultiply(image: &mut Image) {
    unsafe { ffi::ImageAlphaPremultiply(image) }
}

/// Apply Gaussian blur using a box blur approximation
pub fn ImageBlurGaussian(image: &mut Image, blur_size: i32) {
    unsafe { ffi::ImageBlurGaussian(image, blur_size) }
}

/// Apply custom square convolution kernel to image
pub fn ImageKernelConvolution(image: &mut Image, kernel: &[f32]) {
    unsafe { ffi::ImageKernelConvolution(image, kernel.as_ptr(), kernel.len() as c_int) }
}

/// Resize image (Bicubic scaling algorithm)
pub fn ImageResize(image: &mut Image, new_width: i32, new_height: i32) {
    unsafe { ffi::ImageResize(image, new_width, new_height) }
}

/// Resize image (Nearest-Neighbor scaling algorithm)
pub fn ImageResizeNN(image: &mut Image, new_width: i32, new_height: i32) {
    unsafe { ffi::ImageResizeNN(image, new_width, new_height) }
}

/// Resize canvas and fill with color
pub fn ImageResizeCanvas(
    image: &mut Image,
    new_width: i32,
    new_height: i32,
    offset_x: i32,
    offset_y: i32,
    fill: color::Color,
) {
    unsafe { ffi::ImageResizeCanvas(image, new_width, new_height, offset_x, offset_y, fill) }
}

/// Compute all mipmap levels for a provided image
pub fn ImageMipmaps(image: &mut Image) {
    unsafe { ffi::ImageMipmaps(image) }
}

/// Dither image data to 16bpp or lower (Floyd-Steinberg dithering)
pub fn ImageDither(image: &mut Image, r_bpp: i32, g_bpp: i32, b_bpp: i32, a_bpp: i32) {
    unsafe { ffi::ImageDither(image, r_bpp, g_bpp, b_bpp, a_bpp) }
}

/// Flip image vertically
pub fn ImageFlipVertical(image: &mut Image) {
    unsafe { ffi::ImageFlipVertical(image) }
}

/// Flip image horizontally
pub fn ImageFlipHorizontal(image: &mut Image) {
    unsafe { ffi::ImageFlipHorizontal(image) }
}

/// Rotate image by input angle in degrees (-359 to 359)
pub fn ImageRotate(image: &mut Image, degrees: i32) {
    unsafe { ffi::ImageRotate(image, degrees) }
}

/// Rotate image clockwise 90deg
pub fn ImageRotateCW(image: &mut Image) {
    unsafe { ffi::ImageRotateCW(image) }
}

/// Rotate image counter-clockwise 90deg
pub fn ImageRotateCCW(image: &mut Image) {
    unsafe { ffi::ImageRotateCCW(image) }
}

/// Modify image color: tint
pub fn ImageColorTint(image: &mut Image, color: color::Color) {
    unsafe { ffi::ImageColorTint(image, color) }
}

/// Modify image color: invert
pub fn ImageColorInvert(image: &mut Image) {
    unsafe { ffi::ImageColorInvert(image) }
}

/// Modify image color: grayscale
pub fn ImageColorGrayscale(image: &mut Image) {
    unsafe { ffi::ImageColorGrayscale(image) }
}

/// Modify image color: contrast (-100 to 100)
pub fn ImageColorContrast(image: &mut Image, contrast: f32) {
    unsafe { ffi::ImageColorContrast(image, contrast) }
}

/// Modify image color: brightness (-255 to 255)
pub fn ImageColorBrightness(image: &mut Image, brightness: i32) {
    unsafe { ffi::ImageColorBrightness(image, brightness) }
}

/// Modify image color: replace color
pub fn ImageColorReplace(image: &mut Image, color: color::Color, replace: color::Color) {
    unsafe { ffi::ImageColorReplace(image, color, replace) }
}

/// Load color data from image as a Color array (RGBA - 32bit)
pub fn LoadImageColors(image: Image) -> Vec<color::Color> {
    unsafe {
        let colors_ptr = ffi::LoadImageColors(image);
        let count = (image.width * image.height) as usize;
        let slice = std::slice::from_raw_parts(colors_ptr, count);
        let vec = slice.to_vec();
        ffi::UnloadImageColors(colors_ptr); // Free the C-allocated memory
        vec
    }
}

/// Load colors palette from image as a Color array (RGBA - 32bit)
pub fn LoadImagePalette(image: Image, max_palette_size: i32) -> Vec<color::Color> {
    let mut color_count = 0;
    unsafe {
        let palette_ptr = ffi::LoadImagePalette(image, max_palette_size, &mut color_count);
        let slice = std::slice::from_raw_parts(palette_ptr, color_count as usize);
        let vec = slice.to_vec();
        ffi::UnloadImagePalette(palette_ptr); // Free the C-allocated memory
        vec
    }
}

/// Get image alpha border rectangle
pub fn GetImageAlphaBorder(image: Image, threshold: f32) -> math::Rectangle {
    unsafe { ffi::GetImageAlphaBorder(image, threshold) }
}

/// Get image pixel color at (x, y) position
pub fn GetImageColor(image: Image, x: i32, y: i32) -> color::Color {
    unsafe { ffi::GetImageColor(image, x, y) }
}

//------------------------------------------------------------------------------------
// Image drawing functions
//------------------------------------------------------------------------------------

/// Clear image background with given color
pub fn ImageClearBackground(dst: &mut Image, color: color::Color) {
    unsafe { ffi::ImageClearBackground(dst, color) }
}

/// Draw pixel within an image
pub fn ImageDrawPixel(dst: &mut Image, pos_x: i32, pos_y: i32, color: color::Color) {
    unsafe { ffi::ImageDrawPixel(dst, pos_x, pos_y, color) }
}

/// Draw pixel within an image (Vector version)
pub fn ImageDrawPixelV(dst: &mut Image, position: math::Vector2, color: color::Color) {
    unsafe { ffi::ImageDrawPixelV(dst, position, color) }
}

/// Draw line within an image
pub fn ImageDrawLine(
    dst: &mut Image,
    start_pos_x: i32,
    start_pos_y: i32,
    end_pos_x: i32,
    end_pos_y: i32,
    color: color::Color,
) {
    unsafe { ffi::ImageDrawLine(dst, start_pos_x, start_pos_y, end_pos_x, end_pos_y, color) }
}

/// Draw line within an image (Vector version)
pub fn ImageDrawLineV(
    dst: &mut Image,
    start: math::Vector2,
    end: math::Vector2,
    color: color::Color,
) {
    unsafe { ffi::ImageDrawLineV(dst, start, end, color) }
}

/// Draw rectangle within an image
pub fn ImageDrawRectangle(
    dst: &mut Image,
    pos_x: i32,
    pos_y: i32,
    width: i32,
    height: i32,
    color: color::Color,
) {
    unsafe { ffi::ImageDrawRectangle(dst, pos_x, pos_y, width, height, color) }
}

/// Draw rectangle within an image (Vector version)
pub fn ImageDrawRectangleV(
    dst: &mut Image,
    position: math::Vector2,
    size: math::Vector2,
    color: color::Color,
) {
    unsafe { ffi::ImageDrawRectangleV(dst, position, size, color) }
}

/// Draw rectangle within an image
pub fn ImageDrawRectangleRec(dst: &mut Image, rec: math::Rectangle, color: color::Color) {
    unsafe { ffi::ImageDrawRectangleRec(dst, rec, color) }
}

/// Draw rectangle lines within an image
pub fn ImageDrawRectangleLines(
    dst: &mut Image,
    rec: math::Rectangle,
    thick: i32,
    color: color::Color,
) {
    unsafe { ffi::ImageDrawRectangleLines(dst, rec, thick, color) }
}

/// Draw a source image within a destination image (tint applied to source)
pub fn ImageDraw(
    dst: &mut Image,
    src: Image,
    src_rec: math::Rectangle,
    dst_rec: math::Rectangle,
    tint: color::Color,
) {
    unsafe { ffi::ImageDraw(dst, src, src_rec, dst_rec, tint) }
}

/// Draw text (using default font) within an image (destination)
pub fn ImageDrawText(
    dst: &mut Image,
    text: &str,
    pos_x: i32,
    pos_y: i32,
    font_size: i32,
    color: color::Color,
) {
    let text_c = CString::new(text).expect("CString::new failed");
    unsafe { ffi::ImageDrawText(dst, text_c.as_ptr(), pos_x, pos_y, font_size, color) }
}

/// Draw text (custom sprite font) within an image (destination)
pub fn ImageDrawTextEx(
    dst: &mut Image,
    font: Font,
    text: &str,
    position: math::Vector2,
    font_size: f32,
    spacing: f32,
    tint: color::Color,
) {
    let text_c = CString::new(text).expect("CString::new failed");
    unsafe {
        ffi::ImageDrawTextEx(
            dst,
            font,
            text_c.as_ptr(),
            position,
            font_size,
            spacing,
            tint,
        )
    }
}

//------------------------------------------------------------------------------------
// Texture loading functions
//------------------------------------------------------------------------------------

/// Load texture from file into GPU memory (VRAM)
pub fn LoadTexture(file_name: &str) -> Texture2D {
    let file_name_c = CString::new(file_name).expect("CString::new failed");
    unsafe { ffi::LoadTexture(file_name_c.as_ptr()) }
}

/// Load texture from image data
pub fn LoadTextureFromImage(image: Image) -> Texture2D {
    unsafe { ffi::LoadTextureFromImage(image) }
}

/// Load cubemap from image, multiple image cubemap layouts supported
pub fn LoadTextureCubemap(image: Image, layout: i32) -> TextureCubemap {
    unsafe { ffi::LoadTextureCubemap(image, layout) }
}

/// Load texture for rendering (framebuffer)
pub fn LoadRenderTexture(width: i32, height: i32) -> RenderTexture2D {
    unsafe { ffi::LoadRenderTexture(width, height) }
}

/// Check if a texture is valid (loaded in GPU)
pub fn IsTextureValid(texture: Texture2D) -> bool {
    unsafe { ffi::IsTextureValid(texture) }
}

/// Unload texture from GPU memory (VRAM)
pub fn UnloadTexture(texture: Texture2D) {
    unsafe { ffi::UnloadTexture(texture) }
}

/// Check if a render texture is valid (loaded in GPU)
pub fn IsRenderTextureValid(target: RenderTexture2D) -> bool {
    unsafe { ffi::IsRenderTextureValid(target) }
}

/// Unload render texture from GPU memory (VRAM)
pub fn UnloadRenderTexture(target: RenderTexture2D) {
    unsafe { ffi::UnloadRenderTexture(target) }
}

/// Update GPU texture with new data
pub fn UpdateTexture(texture: Texture2D, pixels: &[u8]) {
    unsafe { ffi::UpdateTexture(texture, pixels.as_ptr() as *const c_void) }
}

/// Update GPU texture rectangle with new data
pub fn UpdateTextureRec(texture: Texture2D, rec: math::Rectangle, pixels: &[u8]) {
    unsafe { ffi::UpdateTextureRec(texture, rec, pixels.as_ptr() as *const c_void) }
}

//------------------------------------------------------------------------------------
// Texture configuration functions
//------------------------------------------------------------------------------------

/// Generate GPU mipmaps for a texture
pub fn GenTextureMipmaps(texture: &mut Texture2D) {
    unsafe { ffi::GenTextureMipmaps(texture) }
}

/// Set texture scaling filter mode
pub fn SetTextureFilter(texture: Texture2D, filter: i32) {
    unsafe { ffi::SetTextureFilter(texture, filter) }
}

/// Set texture wrapping mode
pub fn SetTextureWrap(texture: Texture2D, wrap: i32) {
    unsafe { ffi::SetTextureWrap(texture, wrap) }
}

//------------------------------------------------------------------------------------
// Texture drawing functions
//------------------------------------------------------------------------------------

/// Draw a Texture2D
pub fn DrawTexture(texture: Texture2D, pos_x: i32, pos_y: i32, tint: color::Color) {
    unsafe { ffi::DrawTexture(texture, pos_x, pos_y, tint) }
}

/// Draw a Texture2D with position defined as Vector2
pub fn DrawTextureV(texture: Texture2D, position: math::Vector2, tint: color::Color) {
    unsafe { ffi::DrawTextureV(texture, position, tint) }
}

/// Draw a Texture2D with extended parameters
pub fn DrawTextureEx(
    texture: Texture2D,
    position: math::Vector2,
    rotation: f32,
    scale: f32,
    tint: color::Color,
) {
    unsafe { ffi::DrawTextureEx(texture, position, rotation, scale, tint) }
}

/// Draw a part of a texture defined by a rectangle
pub fn DrawTextureRec(
    texture: Texture2D,
    source: math::Rectangle,
    position: math::Vector2,
    tint: color::Color,
) {
    unsafe { ffi::DrawTextureRec(texture, source, position, tint) }
}

/// Draw a part of a texture defined by a rectangle with 'pro' parameters
pub fn DrawTexturePro(
    texture: Texture2D,
    source: math::Rectangle,
    dest: math::Rectangle,
    origin: math::Vector2,
    rotation: f32,
    tint: color::Color,
) {
    unsafe { ffi::DrawTexturePro(texture, source, dest, origin, rotation, tint) }
}

/// Draws a texture (or part of it) that stretches or shrinks nicely
pub fn DrawTextureNPatch(
    texture: Texture2D,
    n_patch_info: NPatchInfo,
    dest: math::Rectangle,
    origin: math::Vector2,
    rotation: f32,
    tint: color::Color,
) {
    unsafe { ffi::DrawTextureNPatch(texture, n_patch_info, dest, origin, rotation, tint) }
}

//------------------------------------------------------------------------------------
// Color/pixel related functions
//------------------------------------------------------------------------------------

/// Check if two colors are equal
pub fn ColorIsEqual(col1: color::Color, col2: color::Color) -> bool {
    col1.r == col2.r && col1.g == col2.g && col1.b == col2.b && col1.a == col2.a
}

/// Get color with alpha applied, alpha goes from 0.0f to 1.0f
pub fn Fade(color: color::Color, alpha: f32) -> color::Color {
    unsafe { ffi::Fade(color, alpha) }
}

/// Get hexadecimal value for a Color (0xRRGGBBAA)
pub fn ColorToInt(color: color::Color) -> i32 {
    unsafe { ffi::ColorToInt(color) }
}

/// Get Color normalized as float [0..1]
pub fn ColorNormalize(color: color::Color) -> math::Vector4 {
    unsafe { ffi::ColorNormalize(color) }
}

/// Get Color from normalized values [0..1]
pub fn ColorFromNormalized(normalized: math::Vector4) -> color::Color {
    unsafe { ffi::ColorFromNormalized(normalized) }
}

/// Get HSV values for a Color
pub fn ColorToHSV(color: color::Color) -> math::Vector3 {
    unsafe { ffi::ColorToHSV(color) }
}

/// Get a Color from HSV values
pub fn ColorFromHSV(hue: f32, saturation: f32, value: f32) -> color::Color {
    unsafe { ffi::ColorFromHSV(hue, saturation, value) }
}

/// Get Color structure from hexadecimal value
pub fn GetColor(hex_value: u32) -> color::Color {
    unsafe { ffi::GetColor(hex_value) }
}

/// Get pixel data size in bytes for certain format
pub fn GetPixelDataSize(width: i32, height: i32, format: i32) -> i32 {
    unsafe { ffi::GetPixelDataSize(width, height, format) }
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

//------------------------------------------------------------------------------------
// Basic geometric 3D shapes drawing functions
//------------------------------------------------------------------------------------

/// Draw a line in 3D world space
pub fn DrawLine3D(start_pos: math::Vector3, end_pos: math::Vector3, color: color::Color) {
    unsafe { ffi::DrawLine3D(start_pos, end_pos, color) }
}

/// Draw a point in 3D space, actually a small line
pub fn DrawPoint3D(position: math::Vector3, color: color::Color) {
    unsafe { ffi::DrawPoint3D(position, color) }
}

/// Draw a circle in 3D world space
pub fn DrawCircle3D(
    center: math::Vector3,
    radius: f32,
    rotation_axis: math::Vector3,
    rotation_angle: f32,
    color: color::Color,
) {
    unsafe { ffi::DrawCircle3D(center, radius, rotation_axis, rotation_angle, color) }
}

/// Draw a color-filled triangle (vertex in counter-clockwise order!)
pub fn DrawTriangle3D(
    v1: math::Vector3,
    v2: math::Vector3,
    v3: math::Vector3,
    color: color::Color,
) {
    unsafe { ffi::DrawTriangle3D(v1, v2, v3, color) }
}

/// Draw a triangle strip defined by points
pub fn DrawTriangleStrip3D(points: &[math::Vector3], color: color::Color) {
    unsafe { ffi::DrawTriangleStrip3D(points.as_ptr(), points.len() as c_int, color) }
}

/// Draw cube
pub fn DrawCube(
    position: math::Vector3,
    width: f32,
    height: f32,
    length: f32,
    color: color::Color,
) {
    unsafe { ffi::DrawCube(position, width, height, length, color) }
}

/// Draw cube (Vector version)
pub fn DrawCubeV(position: math::Vector3, size: math::Vector3, color: color::Color) {
    unsafe { ffi::DrawCubeV(position, size, color) }
}

/// Draw cube wires
pub fn DrawCubeWires(
    position: math::Vector3,
    width: f32,
    height: f32,
    length: f32,
    color: color::Color,
) {
    unsafe { ffi::DrawCubeWires(position, width, height, length, color) }
}

/// Draw cube wires (Vector version)
pub fn DrawCubeWiresV(position: math::Vector3, size: math::Vector3, color: color::Color) {
    unsafe { ffi::DrawCubeWiresV(position, size, color) }
}

/// Draw sphere
pub fn DrawSphere(center_pos: math::Vector3, radius: f32, color: color::Color) {
    unsafe { ffi::DrawSphere(center_pos, radius, color) }
}

/// Draw sphere with extended parameters
pub fn DrawSphereEx(
    center_pos: math::Vector3,
    radius: f32,
    rings: i32,
    slices: i32,
    color: color::Color,
) {
    unsafe { ffi::DrawSphereEx(center_pos, radius, rings, slices, color) }
}

/// Draw sphere wires
pub fn DrawSphereWires(
    center_pos: math::Vector3,
    radius: f32,
    rings: i32,
    slices: i32,
    color: color::Color,
) {
    unsafe { ffi::DrawSphereWires(center_pos, radius, rings, slices, color) }
}

/// Draw a cylinder/cone
pub fn DrawCylinder(
    position: math::Vector3,
    radius_top: f32,
    radius_bottom: f32,
    height: f32,
    slices: i32,
    color: color::Color,
) {
    unsafe { ffi::DrawCylinder(position, radius_top, radius_bottom, height, slices, color) }
}

/// Draw a cylinder with base at startPos and top at endPos
pub fn DrawCylinderEx(
    start_pos: math::Vector3,
    end_pos: math::Vector3,
    start_radius: f32,
    end_radius: f32,
    sides: i32,
    color: color::Color,
) {
    unsafe { ffi::DrawCylinderEx(start_pos, end_pos, start_radius, end_radius, sides, color) }
}

/// Draw a cylinder/cone wires
pub fn DrawCylinderWires(
    position: math::Vector3,
    radius_top: f32,
    radius_bottom: f32,
    height: f32,
    slices: i32,
    color: color::Color,
) {
    unsafe { ffi::DrawCylinderWires(position, radius_top, radius_bottom, height, slices, color) }
}

/// Draw a cylinder wires with base at startPos and top at endPos
pub fn DrawCylinderWiresEx(
    start_pos: math::Vector3,
    end_pos: math::Vector3,
    start_radius: f32,
    end_radius: f32,
    sides: i32,
    color: color::Color,
) {
    unsafe { ffi::DrawCylinderWiresEx(start_pos, end_pos, start_radius, end_radius, sides, color) }
}

/// Draw a capsule with the center of its sphere caps at startPos and endPos
pub fn DrawCapsule(
    start_pos: math::Vector3,
    end_pos: math::Vector3,
    radius: f32,
    slices: i32,
    rings: i32,
    color: color::Color,
) {
    unsafe { ffi::DrawCapsule(start_pos, end_pos, radius, slices, rings, color) }
}

/// Draw capsule wireframe with the center of its sphere caps at startPos and endPos
pub fn DrawCapsuleWires(
    start_pos: math::Vector3,
    end_pos: math::Vector3,
    radius: f32,
    slices: i32,
    rings: i32,
    color: color::Color,
) {
    unsafe { ffi::DrawCapsuleWires(start_pos, end_pos, radius, slices, rings, color) }
}

/// Draw a plane XZ
pub fn DrawPlane(center_pos: math::Vector3, size: math::Vector2, color: color::Color) {
    unsafe { ffi::DrawPlane(center_pos, size, color) }
}

/// Draw a ray line
pub fn DrawRay(ray: Ray, color: color::Color) {
    unsafe { ffi::DrawRay(ray, color) }
}

/// Draw a grid (centered at (0, 0, 0))
pub fn DrawGrid(slices: i32, spacing: f32) {
    unsafe { ffi::DrawGrid(slices, spacing) }
}

//------------------------------------------------------------------------------------
// Model 3d Loading and Drawing Functions (Module: models)
//------------------------------------------------------------------------------------

/// Load model from files (meshes and materials)
pub fn LoadModel(file_name: &str) -> Model {
    let file_name_c = CString::new(file_name).expect("CString::new failed");
    unsafe { ffi::LoadModel(file_name_c.as_ptr()) }
}

/// Load model from generated mesh (default material)
pub fn LoadModelFromMesh(mesh: Mesh) -> Model {
    unsafe { ffi::LoadModelFromMesh(mesh) }
}

/// Check if a model is valid (loaded in GPU, VAO/VBOs)
pub fn IsModelValid(model: Model) -> bool {
    unsafe { ffi::IsModelValid(model) }
}

/// Unload model (including meshes) from memory (RAM and/or VRAM)
pub fn UnloadModel(model: Model) {
    unsafe { ffi::UnloadModel(model) }
}

/// Compute model bounding box limits (considers all meshes)
pub fn GetModelBoundingBox(model: Model) -> BoundingBox {
    unsafe { ffi::GetModelBoundingBox(model) }
}

/// Draw a model (with texture if set)
pub fn DrawModel(model: Model, position: math::Vector3, scale: f32, tint: color::Color) {
    unsafe { ffi::DrawModel(model, position, scale, tint) }
}

/// Draw a model with extended parameters
pub fn DrawModelEx(
    model: Model,
    position: math::Vector3,
    rotation_axis: math::Vector3,
    rotation_angle: f32,
    scale: math::Vector3,
    tint: color::Color,
) {
    unsafe { ffi::DrawModelEx(model, position, rotation_axis, rotation_angle, scale, tint) }
}

/// Draw a model wires (with texture if set)
pub fn DrawModelWires(model: Model, position: math::Vector3, scale: f32, tint: color::Color) {
    unsafe { ffi::DrawModelWires(model, position, scale, tint) }
}

/// Draw a model wires (with texture if set) with extended parameters
pub fn DrawModelWiresEx(
    model: Model,
    position: math::Vector3,
    rotation_axis: math::Vector3,
    rotation_angle: f32,
    scale: math::Vector3,
    tint: color::Color,
) {
    unsafe { ffi::DrawModelWiresEx(model, position, rotation_axis, rotation_angle, scale, tint) }
}

/// Draw a model as points
pub fn DrawModelPoints(model: Model, position: math::Vector3, scale: f32, tint: color::Color) {
    unsafe { ffi::DrawModelPoints(model, position, scale, tint) }
}

/// Draw a model as points with extended parameters
pub fn DrawModelPointsEx(
    model: Model,
    position: math::Vector3,
    rotation_axis: math::Vector3,
    rotation_angle: f32,
    scale: math::Vector3,
    tint: color::Color,
) {
    unsafe { ffi::DrawModelPointsEx(model, position, rotation_axis, rotation_angle, scale, tint) }
}

/// Draw bounding box (wires)
pub fn DrawBoundingBox(box_obj: BoundingBox, color: color::Color) {
    unsafe { ffi::DrawBoundingBox(box_obj, color) }
}

/// Draw a billboard texture
pub fn DrawBillboard(
    camera: Camera,
    texture: Texture2D,
    position: math::Vector3,
    scale: f32,
    tint: color::Color,
) {
    unsafe { ffi::DrawBillboard(camera, texture, position, scale, tint) }
}

/// Draw a billboard texture defined by source
pub fn DrawBillboardRec(
    camera: Camera,
    texture: Texture2D,
    source: math::Rectangle,
    position: math::Vector3,
    size: math::Vector2,
    tint: color::Color,
) {
    unsafe { ffi::DrawBillboardRec(camera, texture, source, position, size, tint) }
}

/// Draw a billboard texture defined by source and rotation
pub fn DrawBillboardPro(
    camera: Camera,
    texture: Texture2D,
    source: math::Rectangle,
    position: math::Vector3,
    up: math::Vector3,
    size: math::Vector2,
    origin: math::Vector2,
    rotation: f32,
    tint: color::Color,
) {
    unsafe {
        ffi::DrawBillboardPro(
            camera, texture, source, position, up, size, origin, rotation, tint,
        )
    }
}

//------------------------------------------------------------------------------------
// Mesh management functions
//------------------------------------------------------------------------------------

/// Upload mesh vertex data in GPU and provide VAO/VBO ids
pub fn UploadMesh(mesh: &mut Mesh, dynamic: bool) {
    unsafe { ffi::UploadMesh(mesh as *mut _, dynamic) }
}

/// Update mesh vertex data in GPU for a specific buffer index
pub fn UpdateMeshBuffer<T>(mesh: Mesh, index: i32, data: &[T], offset: i32) {
    unsafe {
        ffi::UpdateMeshBuffer(
            mesh,
            index,
            data.as_ptr() as *const c_void,
            (data.len() * std::mem::size_of::<T>()) as c_int,
            offset,
        )
    }
}

/// Unload mesh data from CPU and GPU
pub fn UnloadMesh(mesh: Mesh) {
    unsafe { ffi::UnloadMesh(mesh) }
}

/// Draw a 3d mesh with material and transform
pub fn DrawMesh(mesh: Mesh, material: Material, transform: math::Matrix) {
    unsafe { ffi::DrawMesh(mesh, material, transform) }
}

/// Draw multiple mesh instances with material and different transforms
pub fn DrawMeshInstanced(mesh: Mesh, material: Material, transforms: &[math::Matrix]) {
    unsafe {
        ffi::DrawMeshInstanced(
            mesh,
            material,
            transforms.as_ptr(),
            transforms.len() as c_int,
        )
    }
}

/// Compute mesh bounding box limits
pub fn GetMeshBoundingBox(mesh: Mesh) -> BoundingBox {
    unsafe { ffi::GetMeshBoundingBox(mesh) }
}

/// Compute mesh tangents
pub fn GenMeshTangents(mesh: &mut Mesh) {
    unsafe { ffi::GenMeshTangents(mesh as *mut _) }
}

/// Export mesh data to file, returns true on success
pub fn ExportMesh(mesh: Mesh, file_name: &str) -> bool {
    let file_name_c = CString::new(file_name).expect("CString::new failed");
    unsafe { ffi::ExportMesh(mesh, file_name_c.as_ptr()) }
}

/// Export mesh as code file (.h) defining multiple arrays of vertex attributes
pub fn ExportMeshAsCode(mesh: Mesh, file_name: &str) -> bool {
    let file_name_c = CString::new(file_name).expect("CString::new failed");
    unsafe { ffi::ExportMeshAsCode(mesh, file_name_c.as_ptr()) }
}

//------------------------------------------------------------------------------------
// Mesh generation functions
//------------------------------------------------------------------------------------

/// Generate polygonal mesh
pub fn GenMeshPoly(sides: i32, radius: f32) -> Mesh {
    unsafe { ffi::GenMeshPoly(sides, radius) }
}

/// Generate plane mesh (with subdivisions)
pub fn GenMeshPlane(width: f32, length: f32, res_x: i32, res_z: i32) -> Mesh {
    unsafe { ffi::GenMeshPlane(width, length, res_x, res_z) }
}

/// Generate cuboid mesh
pub fn GenMeshCube(width: f32, height: f32, length: f32) -> Mesh {
    unsafe { ffi::GenMeshCube(width, height, length) }
}

/// Generate sphere mesh (standard sphere)
pub fn GenMeshSphere(radius: f32, rings: i32, slices: i32) -> Mesh {
    unsafe { ffi::GenMeshSphere(radius, rings, slices) }
}

/// Generate half-sphere mesh (no bottom cap)
pub fn GenMeshHemiSphere(radius: f32, rings: i32, slices: i32) -> Mesh {
    unsafe { ffi::GenMeshHemiSphere(radius, rings, slices) }
}

/// Generate cylinder mesh
pub fn GenMeshCylinder(radius: f32, height: f32, slices: i32) -> Mesh {
    unsafe { ffi::GenMeshCylinder(radius, height, slices) }
}

/// Generate cone/pyramid mesh
pub fn GenMeshCone(radius: f32, height: f32, slices: i32) -> Mesh {
    unsafe { ffi::GenMeshCone(radius, height, slices) }
}

/// Generate torus mesh
pub fn GenMeshTorus(radius: f32, size: f32, rad_seg: i32, sides: i32) -> Mesh {
    unsafe { ffi::GenMeshTorus(radius, size, rad_seg, sides) }
}

/// Generate trefoil knot mesh
pub fn GenMeshKnot(radius: f32, size: f32, rad_seg: i32, sides: i32) -> Mesh {
    unsafe { ffi::GenMeshKnot(radius, size, rad_seg, sides) }
}

/// Generate heightmap mesh from image data
pub fn GenMeshHeightmap(heightmap: Image, size: math::Vector3) -> Mesh {
    unsafe { ffi::GenMeshHeightmap(heightmap, size) }
}

/// Generate cubes-based map mesh from image data
pub fn GenMeshCubicmap(cubicmap: Image, cube_size: math::Vector3) -> Mesh {
    unsafe { ffi::GenMeshCubicmap(cubicmap, cube_size) }
}

//------------------------------------------------------------------------------------
// Material loading/unloading functions
//------------------------------------------------------------------------------------

/// Load materials from model file.
/// NOTE: The returned slice is valid until you unload the materials manually.
pub fn LoadMaterials(file_name: &str) -> &'static mut [Material] {
    let file_name_c = CString::new(file_name).expect("CString::new failed");
    let mut count = 0;
    unsafe {
        let materials_ptr = ffi::LoadMaterials(file_name_c.as_ptr(), &mut count);
        std::slice::from_raw_parts_mut(materials_ptr, count as usize)
    }
}

/// Load default material
pub fn LoadMaterialDefault() -> Material {
    unsafe { ffi::LoadMaterialDefault() }
}

/// Check if a material is valid (shader assigned, map textures loaded in GPU)
pub fn IsMaterialValid(material: Material) -> bool {
    unsafe { ffi::IsMaterialValid(material) }
}

/// Unload material from GPU memory (VRAM)
pub fn UnloadMaterial(material: Material) {
    unsafe { ffi::UnloadMaterial(material) }
}

/// Set texture for a material map type
pub fn SetMaterialTexture(material: &mut Material, map_type: i32, texture: Texture2D) {
    unsafe { ffi::SetMaterialTexture(material as *mut _, map_type, texture) }
}

/// Set material for a mesh
pub fn SetModelMeshMaterial(model: &mut Model, mesh_id: i32, material_id: i32) {
    unsafe { ffi::SetModelMeshMaterial(model as *mut _, mesh_id, material_id) }
}

//------------------------------------------------------------------------------------
// Model animations loading/unloading functions
//------------------------------------------------------------------------------------

/// Load model animations from file.
/// NOTE: The returned slice is valid until you unload the animations manually.
pub fn LoadModelAnimations(file_name: &str) -> &'static mut [ModelAnimation] {
    let file_name_c = CString::new(file_name).expect("CString::new failed");
    let mut count = 0;
    unsafe {
        let anims_ptr = ffi::LoadModelAnimations(file_name_c.as_ptr(), &mut count);
        std::slice::from_raw_parts_mut(anims_ptr, count as usize)
    }
}

/// Update model animation pose (CPU)
pub fn UpdateModelAnimation(model: Model, anim: ModelAnimation, frame: i32) {
    unsafe { ffi::UpdateModelAnimation(model, anim, frame) }
}

/// Update model animation mesh bone matrices (GPU skinning)
pub fn UpdateModelAnimationBones(model: Model, anim: ModelAnimation, frame: i32) {
    unsafe { ffi::UpdateModelAnimationBones(model, anim, frame) }
}

/// Unload animation data
pub fn UnloadModelAnimation(anim: ModelAnimation) {
    unsafe { ffi::UnloadModelAnimation(anim) }
}

/// Unload animation array data
pub fn UnloadModelAnimations(animations: &mut [ModelAnimation]) {
    unsafe { ffi::UnloadModelAnimations(animations.as_mut_ptr(), animations.len() as c_int) }
}

/// Check model animation skeleton match
pub fn IsModelAnimationValid(model: Model, anim: ModelAnimation) -> bool {
    unsafe { ffi::IsModelAnimationValid(model, anim) }
}

//------------------------------------------------------------------------------------
// Collision detection functions
//------------------------------------------------------------------------------------

/// Check collision between two spheres
pub fn CheckCollisionSpheres(
    center1: math::Vector3,
    radius1: f32,
    center2: math::Vector3,
    radius2: f32,
) -> bool {
    unsafe { ffi::CheckCollisionSpheres(center1, radius1, center2, radius2) }
}

/// Check collision between two bounding boxes
pub fn CheckCollisionBoxes(box1: BoundingBox, box2: BoundingBox) -> bool {
    unsafe { ffi::CheckCollisionBoxes(box1, box2) }
}

/// Check collision between box and sphere
pub fn CheckCollisionBoxSphere(box_obj: BoundingBox, center: math::Vector3, radius: f32) -> bool {
    unsafe { ffi::CheckCollisionBoxSphere(box_obj, center, radius) }
}

/// Get collision info between ray and sphere
pub fn GetRayCollisionSphere(ray: Ray, center: math::Vector3, radius: f32) -> RayCollision {
    unsafe { ffi::GetRayCollisionSphere(ray, center, radius) }
}

/// Get collision info between ray and box
pub fn GetRayCollisionBox(ray: Ray, box_obj: BoundingBox) -> RayCollision {
    unsafe { ffi::GetRayCollisionBox(ray, box_obj) }
}

/// Get collision info between ray and mesh
pub fn GetRayCollisionMesh(ray: Ray, mesh: Mesh, transform: math::Matrix) -> RayCollision {
    unsafe { ffi::GetRayCollisionMesh(ray, mesh, transform) }
}

/// Get collision info between ray and triangle
pub fn GetRayCollisionTriangle(
    ray: Ray,
    p1: math::Vector3,
    p2: math::Vector3,
    p3: math::Vector3,
) -> RayCollision {
    unsafe { ffi::GetRayCollisionTriangle(ray, p1, p2, p3) }
}

/// Get collision info between ray and quad
pub fn GetRayCollisionQuad(
    ray: Ray,
    p1: math::Vector3,
    p2: math::Vector3,
    p3: math::Vector3,
    p4: math::Vector3,
) -> RayCollision {
    unsafe { ffi::GetRayCollisionQuad(ray, p1, p2, p3, p4) }
}

//------------------------------------------------------------------------------------
// Audio Device Functions (Module: raudio)
//------------------------------------------------------------------------------------

/// Initialize audio device and context
pub fn InitAudioDevice() {
    unsafe { ffi::InitAudioDevice() }
}

/// Close the audio device and context
pub fn CloseAudioDevice() {
    unsafe { ffi::CloseAudioDevice() }
}

/// Check if audio device has been initialized successfully
pub fn IsAudioDeviceReady() -> bool {
    unsafe { ffi::IsAudioDeviceReady() }
}

/// Set master volume (listener)
pub fn SetMasterVolume(volume: f32) {
    unsafe { ffi::SetMasterVolume(volume) }
}

/// Get master volume (listener)
pub fn GetMasterVolume() -> f32 {
    unsafe { ffi::GetMasterVolume() }
}

//------------------------------------------------------------------------------------
// Wave/Sound Loading and Management
//------------------------------------------------------------------------------------

/// Load wave data from file
pub fn LoadWave(file_name: &str) -> Wave {
    let file_name_c = CString::new(file_name).expect("CString::new failed");
    unsafe { ffi::LoadWave(file_name_c.as_ptr()) }
}

/// Load wave from memory buffer, fileType refers to extension: i.e. ".wav"
pub fn LoadWaveFromMemory(file_type: &str, file_data: &[u8]) -> Wave {
    let file_type_c = CString::new(file_type).expect("CString::new failed");
    unsafe {
        ffi::LoadWaveFromMemory(
            file_type_c.as_ptr(),
            file_data.as_ptr(),
            file_data.len() as c_int,
        )
    }
}

/// Checks if wave data is valid
pub fn IsWaveValid(wave: Wave) -> bool {
    unsafe { ffi::IsWaveValid(wave) }
}

/// Load sound from file
pub fn LoadSound(file_name: &str) -> Sound {
    let file_name_c = CString::new(file_name).expect("CString::new failed");
    unsafe { ffi::LoadSound(file_name_c.as_ptr()) }
}

/// Load sound from wave data
pub fn LoadSoundFromWave(wave: Wave) -> Sound {
    unsafe { ffi::LoadSoundFromWave(wave) }
}

/// Create a new sound that shares the same sample data as the source sound
pub fn LoadSoundAlias(source: Sound) -> Sound {
    unsafe { ffi::LoadSoundAlias(source) }
}

/// Checks if a sound is valid
pub fn IsSoundValid(sound: Sound) -> bool {
    unsafe { ffi::IsSoundValid(sound) }
}

/// Update sound buffer with new data
pub fn UpdateSound<T>(sound: Sound, data: &[T], sample_count: i32) {
    unsafe {
        ffi::UpdateSound(sound, data.as_ptr() as *const c_void, sample_count);
    }
}

/// Unload wave data
pub fn UnloadWave(wave: Wave) {
    unsafe { ffi::UnloadWave(wave) }
}

/// Unload sound
pub fn UnloadSound(sound: Sound) {
    unsafe { ffi::UnloadSound(sound) }
}

/// Unload a sound alias
pub fn UnloadSoundAlias(alias: Sound) {
    unsafe { ffi::UnloadSoundAlias(alias) }
}

/// Export wave data to file, returns true on success
pub fn ExportWave(wave: Wave, file_name: &str) -> bool {
    let file_name_c = CString::new(file_name).expect("CString::new failed");
    unsafe { ffi::ExportWave(wave, file_name_c.as_ptr()) }
}

/// Export wave sample data to code (.h), returns true on success
pub fn ExportWaveAsCode(wave: Wave, file_name: &str) -> bool {
    let file_name_c = CString::new(file_name).expect("CString::new failed");
    unsafe { ffi::ExportWaveAsCode(wave, file_name_c.as_ptr()) }
}

/// Play a sound
pub fn PlaySound(sound: Sound) {
    unsafe { ffi::PlaySound(sound) }
}

/// Stop playing a sound
pub fn StopSound(sound: Sound) {
    unsafe { ffi::StopSound(sound) }
}

/// Pause a sound
pub fn PauseSound(sound: Sound) {
    unsafe { ffi::PauseSound(sound) }
}

/// Resume a paused sound
pub fn ResumeSound(sound: Sound) {
    unsafe { ffi::ResumeSound(sound) }
}

/// Check if a sound is currently playing
pub fn IsSoundPlaying(sound: Sound) -> bool {
    unsafe { ffi::IsSoundPlaying(sound) }
}

/// Set volume for a sound (1.0 is max level)
pub fn SetSoundVolume(sound: Sound, volume: f32) {
    unsafe { ffi::SetSoundVolume(sound, volume) }
}

/// Set pitch for a sound (1.0 is base level)
pub fn SetSoundPitch(sound: Sound, pitch: f32) {
    unsafe { ffi::SetSoundPitch(sound, pitch) }
}

/// Set pan for a sound (0.5 is center)
pub fn SetSoundPan(sound: Sound, pan: f32) {
    unsafe { ffi::SetSoundPan(sound, pan) }
}

/// Copy a wave to a new wave
pub fn WaveCopy(wave: Wave) -> Wave {
    unsafe { ffi::WaveCopy(wave) }
}

/// Crop a wave to defined frames range
pub fn WaveCrop(wave: &mut Wave, init_frame: i32, final_frame: i32) {
    unsafe { ffi::WaveCrop(wave as *mut _, init_frame, final_frame) }
}

/// Convert wave data to desired format
pub fn WaveFormat(wave: &mut Wave, sample_rate: i32, sample_size: i32, channels: i32) {
    unsafe { ffi::WaveFormat(wave as *mut _, sample_rate, sample_size, channels) }
}

/// Load samples data from wave as a 32bit float data array.
/// NOTE: This function automatically unloads the C-allocated memory and returns a safe `Vec<f32>`.
pub fn LoadWaveSamples(wave: Wave) -> Vec<f32> {
    unsafe {
        let samples_ptr = ffi::LoadWaveSamples(wave);
        // The total number of samples is frameCount * channels
        let sample_count = (wave.frameCount * wave.channels) as usize;
        let samples_slice = std::slice::from_raw_parts(samples_ptr, sample_count);
        let result_vec = samples_slice.to_vec();
        // Free the C-allocated memory
        ffi::UnloadWaveSamples(samples_ptr);
        result_vec
    }
}

// UnloadWaveSamples is now an implementation detail of the safe LoadWaveSamples wrapper above.
// pub fn UnloadWaveSamples(samples: *mut f32) {
//     unsafe { ffi::UnloadWaveSamples(samples) }
// }

//------------------------------------------------------------------------------------
// Music Streaming Functions
//------------------------------------------------------------------------------------

/// Load music stream from file
pub fn LoadMusicStream(file_name: &str) -> Music {
    let file_name_c = CString::new(file_name).expect("CString::new failed");
    unsafe { ffi::LoadMusicStream(file_name_c.as_ptr()) }
}

/// Load music stream from data
pub fn LoadMusicStreamFromMemory(file_type: &str, data: &[u8]) -> Music {
    let file_type_c = CString::new(file_type).expect("CString::new failed");
    unsafe {
        ffi::LoadMusicStreamFromMemory(file_type_c.as_ptr(), data.as_ptr(), data.len() as c_int)
    }
}

/// Checks if a music stream is valid
pub fn IsMusicValid(music: Music) -> bool {
    unsafe { ffi::IsMusicValid(music) }
}

/// Unload music stream
pub fn UnloadMusicStream(music: Music) {
    unsafe { ffi::UnloadMusicStream(music) }
}

/// Start music playing
pub fn PlayMusicStream(music: Music) {
    unsafe { ffi::PlayMusicStream(music) }
}

/// Check if music is playing
pub fn IsMusicStreamPlaying(music: Music) -> bool {
    unsafe { ffi::IsMusicStreamPlaying(music) }
}

/// Updates buffers for music streaming
pub fn UpdateMusicStream(music: Music) {
    unsafe { ffi::UpdateMusicStream(music) }
}

/// Stop music playing
pub fn StopMusicStream(music: Music) {
    unsafe { ffi::StopMusicStream(music) }
}

/// Pause music playing
pub fn PauseMusicStream(music: Music) {
    unsafe { ffi::PauseMusicStream(music) }
}

/// Resume playing paused music
pub fn ResumeMusicStream(music: Music) {
    unsafe { ffi::ResumeMusicStream(music) }
}

/// Seek music to a position (in seconds)
pub fn SeekMusicStream(music: Music, position: f32) {
    unsafe { ffi::SeekMusicStream(music, position) }
}

/// Set volume for music (1.0 is max level)
pub fn SetMusicVolume(music: Music, volume: f32) {
    unsafe { ffi::SetMusicVolume(music, volume) }
}

/// Set pitch for a music (1.0 is base level)
pub fn SetMusicPitch(music: Music, pitch: f32) {
    unsafe { ffi::SetMusicPitch(music, pitch) }
}

/// Set pan for a music (0.5 is center)
pub fn SetMusicPan(music: Music, pan: f32) {
    unsafe { ffi::SetMusicPan(music, pan) }
}

/// Get music time length (in seconds)
pub fn GetMusicTimeLength(music: Music) -> f32 {
    unsafe { ffi::GetMusicTimeLength(music) }
}

/// Get current music time played (in seconds)
pub fn GetMusicTimePlayed(music: Music) -> f32 {
    unsafe { ffi::GetMusicTimePlayed(music) }
}

//------------------------------------------------------------------------------------
// AudioStream Functions
//------------------------------------------------------------------------------------

/// Represents the audio callback function signature.
pub type AudioCallback = Option<unsafe extern "C" fn(buffer: *mut c_void, frames: u32)>;

/// Load audio stream (to stream raw audio pcm data)
pub fn LoadAudioStream(sample_rate: u32, sample_size: u32, channels: u32) -> AudioStream {
    unsafe { ffi::LoadAudioStream(sample_rate, sample_size, channels) }
}

/// Checks if an audio stream is valid
pub fn IsAudioStreamValid(stream: AudioStream) -> bool {
    unsafe { ffi::IsAudioStreamValid(stream) }
}

/// Unload audio stream and free memory
pub fn UnloadAudioStream(stream: AudioStream) {
    unsafe { ffi::UnloadAudioStream(stream) }
}

/// Update audio stream buffers with data
pub fn UpdateAudioStream<T>(stream: AudioStream, data: &[T], frame_count: i32) {
    unsafe {
        ffi::UpdateAudioStream(stream, data.as_ptr() as *const c_void, frame_count);
    }
}

/// Check if any audio stream buffers requires refill
pub fn IsAudioStreamProcessed(stream: AudioStream) -> bool {
    unsafe { ffi::IsAudioStreamProcessed(stream) }
}

/// Play audio stream
pub fn PlayAudioStream(stream: AudioStream) {
    unsafe { ffi::PlayAudioStream(stream) }
}

/// Pause audio stream
pub fn PauseAudioStream(stream: AudioStream) {
    unsafe { ffi::PauseAudioStream(stream) }
}

/// Resume audio stream
pub fn ResumeAudioStream(stream: AudioStream) {
    unsafe { ffi::ResumeAudioStream(stream) }
}

/// Check if audio stream is playing
pub fn IsAudioStreamPlaying(stream: AudioStream) -> bool {
    unsafe { ffi::IsAudioStreamPlaying(stream) }
}

/// Stop audio stream
pub fn StopAudioStream(stream: AudioStream) {
    unsafe { ffi::StopAudioStream(stream) }
}

/// Set volume for audio stream (1.0 is max level)
pub fn SetAudioStreamVolume(stream: AudioStream, volume: f32) {
    unsafe { ffi::SetAudioStreamVolume(stream, volume) }
}

/// Set pitch for audio stream (1.0 is base level)
pub fn SetAudioStreamPitch(stream: AudioStream, pitch: f32) {
    unsafe { ffi::SetAudioStreamPitch(stream, pitch) }
}

/// Set pan for audio stream (0.5 is centered)
pub fn SetAudioStreamPan(stream: AudioStream, pan: f32) {
    unsafe { ffi::SetAudioStreamPan(stream, pan) }
}

/// Default size for new audio streams
pub fn SetAudioStreamBufferSizeDefault(size: i32) {
    unsafe { ffi::SetAudioStreamBufferSizeDefault(size) }
}

/// Audio thread callback to request new data
pub fn SetAudioStreamCallback(stream: AudioStream, callback: AudioCallback) {
    unsafe { ffi::SetAudioStreamCallback(stream, callback) }
}

/// Attach audio stream processor to stream
pub fn AttachAudioStreamProcessor(stream: AudioStream, processor: AudioCallback) {
    unsafe { ffi::AttachAudioStreamProcessor(stream, processor) }
}

/// Detach audio stream processor from stream
pub fn DetachAudioStreamProcessor(stream: AudioStream, processor: AudioCallback) {
    unsafe { ffi::DetachAudioStreamProcessor(stream, processor) }
}

/// Attach audio stream processor to the entire audio pipeline
pub fn AttachAudioMixedProcessor(processor: AudioCallback) {
    unsafe { ffi::AttachAudioMixedProcessor(processor) }
}

/// Detach audio stream processor from the entire audio pipeline
pub fn DetachAudioMixedProcessor(processor: AudioCallback) {
    unsafe { ffi::DetachAudioMixedProcessor(processor) }
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
