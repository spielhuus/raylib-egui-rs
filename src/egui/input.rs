// input.rs

use std::time::Instant;

use crate::raylib::*;
use egui::{Key, Modifiers, PointerButton, Pos2, RawInput, pos2, vec2};

/// Contains and manages everything related to `egui` input.
pub struct Input {
    dt: Instant,
    pointer_pos: Pos2,
    pub(crate) raw: RawInput,
    pub(crate) scale_factor: f32,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            dt: Instant::now(),
            pointer_pos: Default::default(),
            raw: Default::default(),
            scale_factor: 1.0,
        }
    }
}

impl Input {
    pub(crate) fn take(&mut self) -> RawInput {
        // We use predicted_dt for the time until the next frame, which is what egui wants.
        // raylib's get_frame_time is the duration of the last frame.
        self.raw.predicted_dt = self.dt.elapsed().as_secs_f32();
        self.dt = Instant::now();
        self.raw.take()
    }

    /// Updates egui with the latest input from Raylib.
    pub fn update(&mut self) {
        // Update screen rectangle
        self.resize_event(GetScreenWidth() as f32, GetScreenHeight() as f32);

        let modifiers = translate_modifiers();

        // --- Mouse ---
        let mouse_pos = GetMousePosition();
        self.pointer_pos = pos2(
            mouse_pos.x / self.scale_factor,
            mouse_pos.y / self.scale_factor,
        );
        self.raw
            .events
            .push(egui::Event::PointerMoved(self.pointer_pos));

        for (rl_button, egui_button) in [
            (MouseButton::MOUSE_BUTTON_LEFT, PointerButton::Primary),
            (MouseButton::MOUSE_BUTTON_RIGHT, PointerButton::Secondary),
            (MouseButton::MOUSE_BUTTON_MIDDLE, PointerButton::Middle),
        ] {
            if IsMouseButtonPressed(rl_button as i32) {
                self.raw.events.push(egui::Event::PointerButton {
                    pos: self.pointer_pos,
                    button: egui_button,
                    pressed: true,
                    modifiers,
                });
            }
            if IsMouseButtonReleased(rl_button as i32) {
                self.raw.events.push(egui::Event::PointerButton {
                    pos: self.pointer_pos,
                    button: egui_button,
                    pressed: false,
                    modifiers,
                });
            }
        }

        // --- Mouse Wheel ---
        let wheel_move = GetMouseWheelMoveV();
        if wheel_move.x != 0.0 || wheel_move.y != 0.0 {
            // TODO
            // self.raw.events.push(egui::Event::Scroll(vec2(
            //     wheel_move.x * 8.0,
            //     wheel_move.y * 8.0,
            // )));
        }

        // --- Keyboard ---
        // Text input
        let key_code = GetCharPressed();
        while key_code != 0 {
            let ch = char::from_u32(key_code as u32).unwrap_or('\0');
            if is_printable(ch) {
                self.raw.events.push(egui::Event::Text(ch.to_string()));
            }
        }

        // Key presses and releases
        for key in ALL_KEYS {
            if let Some(egui_key) = translate_keycode(key) {
                if IsKeyPressed(key) {
                    self.raw.events.push(egui::Event::Key {
                        key: egui_key,
                        physical_key: None,
                        pressed: true,
                        repeat: false,
                        modifiers,
                    });
                }
                if IsKeyPressed(key) {
                    self.raw.events.push(egui::Event::Key {
                        key: egui_key,
                        physical_key: None,
                        pressed: false,
                        repeat: false,
                        modifiers,
                    });
                }
            }
        }
    }

    /// Set the scale_factor and update the screen_rect
    pub fn set_scale_factor(&mut self, scale_factor: f32, (w, h): (f32, f32)) {
        self.scale_factor = scale_factor;
        self.resize_event(w, h);
    }

    /// Update screen_rect data with window size
    pub fn resize_event(&mut self, w: f32, h: f32) {
        self.raw.screen_rect = Some(egui::Rect::from_min_size(
            Default::default(),
            vec2(w, h) / self.scale_factor,
        ));
    }
}

// --- Helper Functions ---

fn is_printable(chr: char) -> bool {
    let is_in_private_use_area = ('\u{e000}'..='\u{f8ff}').contains(&chr)
        || ('\u{f0000}'..='\u{ffffd}').contains(&chr)
        || ('\u{100000}'..='\u{10fffd}').contains(&chr);
    !is_in_private_use_area && !chr.is_ascii_control()
}

fn translate_modifiers() -> Modifiers {
    Modifiers {
        alt: IsKeyDown(KeyboardKey::KEY_LEFT_ALT) || IsKeyDown(KeyboardKey::KEY_RIGHT_ALT),
        ctrl: IsKeyDown(KeyboardKey::KEY_LEFT_CONTROL) || IsKeyDown(KeyboardKey::KEY_RIGHT_CONTROL),
        shift: IsKeyDown(KeyboardKey::KEY_LEFT_SHIFT) || IsKeyDown(KeyboardKey::KEY_RIGHT_SHIFT),
        mac_cmd: cfg!(target_os = "macos")
            && (IsKeyDown(KeyboardKey::KEY_LEFT_SUPER) || IsKeyDown(KeyboardKey::KEY_RIGHT_SUPER)),
        command: if cfg!(target_os = "macos") {
            IsKeyDown(KeyboardKey::KEY_LEFT_SUPER) || IsKeyDown(KeyboardKey::KEY_RIGHT_SUPER)
        } else {
            IsKeyDown(KeyboardKey::KEY_LEFT_CONTROL) || IsKeyDown(KeyboardKey::KEY_RIGHT_CONTROL)
        },
    }
}

fn translate_keycode(key: KeyboardKey) -> Option<egui::Key> {
    Some(match key {
        KeyboardKey::KEY_ESCAPE => Key::Escape,
        KeyboardKey::KEY_INSERT => Key::Insert,
        KeyboardKey::KEY_HOME => Key::Home,
        KeyboardKey::KEY_DELETE => Key::Delete,
        KeyboardKey::KEY_END => Key::End,
        KeyboardKey::KEY_PAGE_DOWN => Key::PageDown,
        KeyboardKey::KEY_PAGE_UP => Key::PageUp,
        KeyboardKey::KEY_LEFT => Key::ArrowLeft,
        KeyboardKey::KEY_UP => Key::ArrowUp,
        KeyboardKey::KEY_RIGHT => Key::ArrowRight,
        KeyboardKey::KEY_DOWN => Key::ArrowDown,
        KeyboardKey::KEY_BACKSPACE => Key::Backspace,
        KeyboardKey::KEY_ENTER => Key::Enter,
        KeyboardKey::KEY_TAB => Key::Tab,
        KeyboardKey::KEY_SPACE => Key::Space,
        KeyboardKey::KEY_A => Key::A,
        KeyboardKey::KEY_B => Key::B,
        KeyboardKey::KEY_C => Key::C,
        KeyboardKey::KEY_D => Key::D,
        KeyboardKey::KEY_E => Key::E,
        KeyboardKey::KEY_F => Key::F,
        KeyboardKey::KEY_G => Key::G,
        KeyboardKey::KEY_H => Key::H,
        KeyboardKey::KEY_I => Key::I,
        KeyboardKey::KEY_J => Key::J,
        KeyboardKey::KEY_K => Key::K,
        KeyboardKey::KEY_L => Key::L,
        KeyboardKey::KEY_M => Key::M,
        KeyboardKey::KEY_N => Key::N,
        KeyboardKey::KEY_O => Key::O,
        KeyboardKey::KEY_P => Key::P,
        KeyboardKey::KEY_Q => Key::Q,
        KeyboardKey::KEY_R => Key::R,
        KeyboardKey::KEY_S => Key::S,
        KeyboardKey::KEY_T => Key::T,
        KeyboardKey::KEY_U => Key::U,
        KeyboardKey::KEY_V => Key::V,
        KeyboardKey::KEY_W => Key::W,
        KeyboardKey::KEY_X => Key::X,
        KeyboardKey::KEY_Y => Key::Y,
        KeyboardKey::KEY_Z => Key::Z,
        _ => return None,
    })
}

// A list of all keys we care about. Raylib doesn't have a way to iterate
// over pressed keys, so we check them all.
const ALL_KEYS: [KeyboardKey; 41] = [
    KeyboardKey::KEY_ESCAPE,
    KeyboardKey::KEY_INSERT,
    KeyboardKey::KEY_HOME,
    KeyboardKey::KEY_DELETE,
    KeyboardKey::KEY_END,
    KeyboardKey::KEY_PAGE_DOWN,
    KeyboardKey::KEY_PAGE_UP,
    KeyboardKey::KEY_LEFT,
    KeyboardKey::KEY_UP,
    KeyboardKey::KEY_RIGHT,
    KeyboardKey::KEY_DOWN,
    KeyboardKey::KEY_BACKSPACE,
    KeyboardKey::KEY_ENTER,
    KeyboardKey::KEY_TAB,
    KeyboardKey::KEY_SPACE,
    KeyboardKey::KEY_A,
    KeyboardKey::KEY_B,
    KeyboardKey::KEY_C,
    KeyboardKey::KEY_D,
    KeyboardKey::KEY_E,
    KeyboardKey::KEY_F,
    KeyboardKey::KEY_G,
    KeyboardKey::KEY_H,
    KeyboardKey::KEY_I,
    KeyboardKey::KEY_J,
    KeyboardKey::KEY_K,
    KeyboardKey::KEY_L,
    KeyboardKey::KEY_M,
    KeyboardKey::KEY_N,
    KeyboardKey::KEY_O,
    KeyboardKey::KEY_P,
    KeyboardKey::KEY_Q,
    KeyboardKey::KEY_R,
    KeyboardKey::KEY_S,
    KeyboardKey::KEY_T,
    KeyboardKey::KEY_U,
    KeyboardKey::KEY_V,
    KeyboardKey::KEY_W,
    KeyboardKey::KEY_X,
    KeyboardKey::KEY_Y,
    KeyboardKey::KEY_Z,
];
