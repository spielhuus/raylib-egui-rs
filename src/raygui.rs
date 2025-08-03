use crate::ffi;
use crate::{
    color::Color,
    math::{Rectangle, Vector2, Vector3},
    raylib::Font,
};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

pub use ffi::{
    GuiControl, GuiControlProperty, GuiDefaultProperty, GuiIconName, GuiListViewProperty,
    GuiSliderProperty, GuiState, GuiTextAlignment, GuiTextAlignmentVertical, GuiTextWrapMode,
};

macro_rules! cstr {
    ($s:expr) => {
        if let Some(text) = $s {
            let mut c_text: Vec<u8> = text.as_bytes().to_vec();
            c_text.resize(text.len() + 1, 0);
            c_text
        } else {
            todo!()
            // unsafe { ffi::GuiButton(bounds, std::ptr::null()) }
        }
    };
}

//----------------------------------------------------------------------------------
// Load the Styles
//----------------------------------------------------------------------------------

pub fn GuiLoadStyleAmber() {
    unsafe {
        ffi::DynGuiLoadStyleAmber();
    }
}
pub fn GuiLoadStyleGenesis() {
    unsafe {
        ffi::DynGuiLoadStyleGenesis();
    }
}
pub fn GuiLoadStyleJungle() {
    unsafe {
        ffi::DynGuiLoadStyleJungle();
    }
}
pub fn GuiLoadStyleCyber() {
    unsafe {
        ffi::DynGuiLoadStyleCyber();
    }
}
pub fn GuiLoadStyleLavanda() {
    unsafe {
        ffi::DynGuiLoadStyleLavanda();
    }
}
pub fn GuiLoadStyleCherry() {
    unsafe {
        ffi::DynGuiLoadStyleCherry();
    }
}
pub fn GuiLoadStyleEnefete() {
    unsafe {
        ffi::DynGuiLoadStyleEnefete();
    }
}
pub fn GuiLoadStyleRLTech() {
    unsafe {
        ffi::DynGuiLoadStyleRLTech();
    }
}
pub fn GuiLoadStyleSunny() {
    unsafe {
        ffi::DynGuiLoadStyleSunny();
    }
}
pub fn GuiLoadStyleTerminal() {
    unsafe {
        ffi::DynGuiLoadStyleTerminal();
    }
}
pub fn GuiLoadStyleBluish() {
    unsafe {
        ffi::DynGuiLoadStyleBluish();
    }
}
pub fn GuiLoadStyleDark() {
    unsafe {
        ffi::DynGuiLoadStyleDark();
    }
}
pub fn GuiLoadStyleCandy() {
    unsafe {
        ffi::DynGuiLoadStyleCandy();
    }
}
pub fn GuiLoadStyleAshes() {
    unsafe {
        ffi::DynGuiLoadStyleAshes();
    }
}

//----------------------------------------------------------------------------------
// Global gui state control functions
//----------------------------------------------------------------------------------

/// Enable gui controls (global state)
pub fn GuiEnable() {
    unsafe { ffi::GuiEnable() }
}

/// Disable gui controls (global state)
pub fn GuiDisable() {
    unsafe { ffi::GuiDisable() }
}

/// Lock gui controls (global state)
pub fn GuiLock() {
    unsafe { ffi::GuiLock() }
}

/// Unlock gui controls (global state)
pub fn GuiUnlock() {
    unsafe { ffi::GuiUnlock() }
}

/// Check if gui is locked (global state)
pub fn GuiIsLocked() -> bool {
    unsafe { ffi::GuiIsLocked() }
}

/// Set gui controls alpha (global state), alpha goes from 0.0f to 1.0f
pub fn GuiSetAlpha(alpha: f32) {
    unsafe { ffi::GuiSetAlpha(alpha) }
}

/// Set gui state (global state)
pub fn GuiSetState(state: GuiState) {
    unsafe { ffi::GuiSetState(state as i32) }
}

/// Get gui state (global state)
pub fn GuiGetState() -> i32 {
    unsafe { ffi::GuiGetState() }
}

//----------------------------------------------------------------------------------
// Font set/get functions
//----------------------------------------------------------------------------------

/// Set gui custom font (global state)
pub fn GuiSetFont(font: Font) {
    unsafe { ffi::GuiSetFont(font) }
}

/// Get gui custom font (global state)
pub fn GuiGetFont() -> Font {
    unsafe { ffi::GuiGetFont() }
}

//----------------------------------------------------------------------------------
// Style set/get functions
//----------------------------------------------------------------------------------

/// Set one style property
pub fn GuiSetStyle(control: GuiControl, property: i32, value: i32) {
    unsafe { ffi::GuiSetStyle(control as i32, property, value) }
}

/// Get one style property
pub fn GuiGetStyle(control: GuiControl, property: i32) -> i32 {
    unsafe { ffi::GuiGetStyle(control as i32, property) }
}

//----------------------------------------------------------------------------------
// Styles loading functions
//----------------------------------------------------------------------------------

/// Load style file over global style variable (.rgs)
pub fn GuiLoadStyle(file_name: &str) {
    let file_name = CString::new(file_name).expect("Failed to create CString");
    unsafe { ffi::GuiLoadStyle(file_name.as_ptr()) }
}

/// Load style default over global style
pub fn GuiLoadStyleDefault() {
    unsafe { ffi::GuiLoadStyleDefault() }
}

//----------------------------------------------------------------------------------
// Tooltips management functions
//----------------------------------------------------------------------------------

/// Enable gui tooltips (global state)
pub fn GuiEnableTooltip() {
    unsafe { ffi::GuiEnableTooltip() }
}

/// Disable gui tooltips (global state)
pub fn GuiDisableTooltip() {
    unsafe { ffi::GuiDisableTooltip() }
}

/// Set tooltip string
pub fn GuiSetTooltip(tooltip: &str) {
    let tooltip = CString::new(tooltip).expect("Failed to create CString");
    unsafe { ffi::GuiSetTooltip(tooltip.as_ptr()) }
}

//----------------------------------------------------------------------------------
// Icons functionality
//----------------------------------------------------------------------------------

/// Get text with icon id prepended (if supported)
pub fn GuiIconText(icon_id: i32, text: Option<&str>) -> String {
    // TODO
    // if let Some(text) = text {
    //     let mut c_text: Vec<u8> = text.as_bytes().to_vec();
    //     c_text.resize(text.len() + 1, 0);
    //     unsafe {
    let c_text = cstr!(text);
    unsafe {
        let result_ptr = ffi::GuiIconText(icon_id, c_text.as_ptr() as *mut c_char);
        CStr::from_ptr(result_ptr).to_string_lossy().into_owned()
    }
    //     }
    // } else {
    //     unsafe {
    //         let result_ptr = ffi::GuiIconText(icon_id, std::ptr::null());
    //         CStr::from_ptr(result_ptr).to_string_lossy().into_owned()
    //     }
    // }
}

/// Set default icon drawing size
pub fn GuiSetIconScale(scale: i32) {
    unsafe { ffi::GuiSetIconScale(scale) }
}

/// Get raygui icons data pointer
pub fn GuiGetIcons() -> *mut u32 {
    unsafe { ffi::GuiGetIcons() }
}

/// Load raygui icons file (.rgi) into internal icons data.
///
/// # Safety
/// The returned pointer is to a C-style array of C-style strings allocated by raygui.
/// The C API is unclear about memory management (e.g., how to free this memory or get the item count).
/// Use with extreme caution.
pub fn GuiLoadIcons(file_name: &str, load_icons_name: bool) -> *mut *mut c_char {
    let file_name = CString::new(file_name).expect("Failed to create CString");
    unsafe { ffi::GuiLoadIcons(file_name.as_ptr(), load_icons_name) }
}

/// Draw icon using pixel size at specified position
pub fn GuiDrawIcon(icon_id: i32, pos_x: i32, pos_y: i32, pixel_size: i32, color: Color) {
    unsafe { ffi::GuiDrawIcon(icon_id, pos_x, pos_y, pixel_size, color) }
}

//----------------------------------------------------------------------------------
// Container/separator controls
//----------------------------------------------------------------------------------

/// Window Box control, shows a window that can be closed
pub fn GuiWindowBox(bounds: Rectangle, title: Option<&str>) -> i32 {
    let c_title = cstr!(title);
    unsafe { ffi::GuiWindowBox(bounds, c_title.as_ptr() as *mut c_char) }
}

/// Group Box control with text name
pub fn GuiGroupBox(bounds: Rectangle, text: Option<&str>) -> i32 {
    let c_text = cstr!(text);
    unsafe { ffi::GuiGroupBox(bounds, c_text.as_ptr() as *mut c_char) }
}

/// Line separator control, could contain text
pub fn GuiLine(bounds: Rectangle, text: Option<&str>) -> i32 {
    let c_text = cstr!(text);
    unsafe { ffi::GuiLine(bounds, c_text.as_ptr() as *mut c_char) }
}

/// Panel control, useful to group controls
pub fn GuiPanel(bounds: Rectangle, text: Option<&str>) -> i32 {
    let c_text = cstr!(text);
    unsafe { ffi::GuiPanel(bounds, c_text.as_ptr() as *mut c_char) }
}

/// Tab Bar control, returns TAB to be closed or -1
pub fn GuiTabBar(bounds: Rectangle, text: &[&str], active: &mut i32) -> i32 {
    let c_strings: Vec<CString> = text
        .iter()
        .map(|s| CString::new(*s).expect("Failed to create CString for TabBar"))
        .collect();
    let c_pointers: Vec<*const c_char> = c_strings.iter().map(|cs| cs.as_ptr()).collect();

    unsafe {
        ffi::GuiTabBar(
            bounds,
            c_pointers.as_ptr() as *mut *const i8,
            text.len() as c_int,
            active as *mut c_int,
        )
    }
}

/// Scroll Panel control
pub fn GuiScrollPanel(
    bounds: Rectangle,
    text: Option<&str>,
    content: Rectangle,
    scroll: &mut Vector2,
    view: &mut Rectangle,
) -> i32 {
    let c_text = cstr!(text);
    unsafe {
        ffi::GuiScrollPanel(
            bounds,
            c_text.as_ptr() as *mut c_char,
            content,
            scroll as *mut Vector2,
            view as *mut Rectangle,
        )
    }
}

//----------------------------------------------------------------------------------
// Basic controls
//----------------------------------------------------------------------------------

/// Label control
pub fn GuiLabel(bounds: Rectangle, text: Option<&str>) -> i32 {
    let c_text = cstr!(text);
    unsafe { ffi::GuiLabel(bounds, c_text.as_ptr() as *mut c_char) }
}

/// Button control, returns true when clicked
pub fn GuiButton(bounds: Rectangle, text: Option<&str>) -> i32 {
    // TODO
    let c_text = cstr!(text);
    // if let Some(text) = text {
    //     let mut c_text: Vec<u8> = text.as_bytes().to_vec();
    //     c_text.resize(text.len() + 1, 0);
    unsafe { ffi::GuiButton(bounds, c_text.as_ptr() as *mut c_char) }
    // } else {
    //     unsafe { ffi::GuiButton(bounds, std::ptr::null()) }
    // }
}

/// Label button control, returns true when clicked
pub fn GuiLabelButton(bounds: Rectangle, text: &str) -> i32 {
    let c_text = CString::new(text).expect("Failed to create CString");
    unsafe { ffi::GuiLabelButton(bounds, c_text.as_ptr()) }
}

/// Toggle Button control
pub fn GuiToggle(bounds: Rectangle, text: Option<&str>, active: &mut bool) -> i32 {
    let c_text = cstr!(text);
    unsafe { ffi::GuiToggle(bounds, c_text.as_ptr() as *mut c_char, active as *mut bool) }
}

/// Toggle Group control (text is a semicolon separated list)
pub fn GuiToggleGroup(bounds: Rectangle, text: &str, active: &mut i32) -> i32 {
    let c_text = CString::new(text).expect("Failed to create CString");
    unsafe { ffi::GuiToggleGroup(bounds, c_text.as_ptr(), active as *mut c_int) }
}

/// Toggle Slider control (text is a semicolon separated list)
pub fn GuiToggleSlider(bounds: Rectangle, text: &str, active: &mut i32) -> i32 {
    let c_text = CString::new(text).expect("Failed to create CString");
    unsafe { ffi::GuiToggleSlider(bounds, c_text.as_ptr(), active as *mut c_int) }
}

/// Check Box control, returns true when active
pub fn GuiCheckBox(bounds: Rectangle, text: Option<&str>, checked: &mut bool) -> i32 {
    let c_text = cstr!(text);
    unsafe { ffi::GuiCheckBox(bounds, c_text.as_ptr() as *mut c_char, checked as *mut bool) }
}

/// Combo Box control (text is a semicolon separated list)
pub fn GuiComboBox(bounds: Rectangle, text: &str, active: &mut i32) -> i32 {
    let c_text = CString::new(text).expect("Failed to create CString");
    unsafe { ffi::GuiComboBox(bounds, c_text.as_ptr(), active as *mut c_int) }
}

/// Dropdown Box control (text is a semicolon separated list)
pub fn GuiDropdownBox(bounds: Rectangle, text: &str, active: &mut i32, edit_mode: bool) -> i32 {
    let c_text = CString::new(text).expect("Failed to create CString");
    unsafe { ffi::GuiDropdownBox(bounds, c_text.as_ptr(), active as *mut c_int, edit_mode) }
}

/// Spinner control
pub fn GuiSpinner(
    bounds: Rectangle,
    text: Option<&str>,
    value: &mut i32,
    min_value: i32,
    max_value: i32,
    edit_mode: bool,
) -> i32 {
    let c_text = cstr!(text);
    unsafe {
        ffi::GuiSpinner(
            bounds,
            c_text.as_ptr() as *mut c_char,
            value as *mut c_int,
            min_value,
            max_value,
            edit_mode,
        )
    }
}

/// Value Box control, updates input text with numbers
pub fn GuiValueBox(
    bounds: Rectangle,
    text: Option<&str>,
    value: &mut i32,
    min_value: i32,
    max_value: i32,
    edit_mode: bool,
) -> i32 {
    // TODO:
    // let c_text = cstr!(text);

    let c_text = if let Some(text) = text {
        let mut c_text: Vec<u8> = text.as_bytes().to_vec();
        c_text.resize(text.len() + 1, 0);
        c_text.as_mut_ptr()
    } else {
        std::ptr::null()
    };

    unsafe {
        ffi::GuiValueBox(
            bounds,
            c_text as *mut c_char,
            value as *mut c_int,
            min_value,
            max_value,
            edit_mode,
        )
    }
}

/// Value box control for float values. `text_value` is the string buffer.
pub fn GuiValueBoxFloat(
    bounds: Rectangle,
    text: Option<&str>,
    text_value: &mut [u8],
    value: &mut f32,
    edit_mode: bool,
) -> i32 {
    let c_text = cstr!(text);
    unsafe {
        ffi::GuiValueBoxFloat(
            bounds,
            c_text.as_ptr() as *mut c_char,
            text_value.as_mut_ptr() as *mut c_char,
            value as *mut f32,
            edit_mode,
        )
    }
}

/// Text Box control, updates input text. `text` is the string buffer.
pub fn GuiTextBox(bounds: Rectangle, text: &mut String, text_size: usize, edit_mode: bool) -> i32 {
    let mut c_text: Vec<u8> = text.as_bytes().to_vec();
    c_text.resize(text_size, 0);
    unsafe {
        let res = ffi::GuiTextBox(
            bounds,
            c_text.as_mut_ptr() as *mut c_char,
            text_size as c_int,
            edit_mode,
        );

        let first_null = c_text.iter().position(|&c| c == 0).unwrap_or(c_text.len());
        let new_text = String::from_utf8_lossy(&c_text[..first_null]);
        text.clear();
        text.push_str(&new_text);
        res
    }
}

/// Slider control
pub fn GuiSlider(
    bounds: Rectangle,
    text_left: Option<&str>,
    text_right: Option<&str>,
    value: &mut f32,
    min_value: f32,
    max_value: f32,
) -> i32 {
    let c_text_left = cstr!(text_left);
    let c_text_right = cstr!(text_right);
    unsafe {
        ffi::GuiSlider(
            bounds,
            c_text_left.as_ptr() as *mut c_char,
            c_text_right.as_ptr() as *mut c_char,
            value as *mut f32,
            min_value,
            max_value,
        )
    }
}

/// Slider Bar control
pub fn GuiSliderBar(
    bounds: Rectangle,
    text_left: Option<&str>,
    text_right: Option<&str>,
    value: &mut f32,
    min_value: f32,
    max_value: f32,
) -> i32 {
    let c_text_left = cstr!(text_left);
    let c_text_right = cstr!(text_right);
    unsafe {
        ffi::GuiSliderBar(
            bounds,
            c_text_left.as_ptr() as *mut c_char,
            c_text_right.as_ptr() as *mut c_char,
            value as *mut f32,
            min_value,
            max_value,
        )
    }
}

/// Progress Bar control
pub fn GuiProgressBar(
    bounds: Rectangle,
    text_left: Option<&str>,
    text_right: Option<&str>,
    value: &mut f32,
    min_value: f32,
    max_value: f32,
) -> i32 {
    let c_text_left = cstr!(text_left);
    let c_text_right = cstr!(text_right);
    unsafe {
        ffi::GuiProgressBar(
            bounds,
            c_text_left.as_ptr() as *mut c_char,
            c_text_right.as_ptr() as *mut c_char,
            value as *mut f32,
            min_value,
            max_value,
        )
    }
}

/// Status Bar control, shows info text
pub fn GuiStatusBar(bounds: Rectangle, text: Option<&str>) -> i32 {
    let c_text = cstr!(text);
    unsafe { ffi::GuiStatusBar(bounds, c_text.as_ptr() as *mut c_char) }
}

/// Dummy control for placeholders
pub fn GuiDummyRec(bounds: Rectangle, text: Option<&str>) -> i32 {
    let c_text = cstr!(text);
    unsafe { ffi::GuiDummyRec(bounds, c_text.as_ptr() as *mut c_char) }
}

/// Grid control
pub fn GuiGrid(
    bounds: Rectangle,
    text: Option<&str>,
    spacing: f32,
    subdivs: i32,
    mouse_cell: &mut Vector2,
) -> i32 {
    let c_text = cstr!(text);
    unsafe {
        ffi::GuiGrid(
            bounds,
            c_text.as_ptr() as *mut c_char,
            spacing,
            subdivs,
            mouse_cell as *mut Vector2,
        )
    }
}

//----------------------------------------------------------------------------------
// Advance controls
//----------------------------------------------------------------------------------

/// List View control (text is a semicolon separated list)
pub fn GuiListView(
    bounds: Rectangle,
    text: Option<&str>,
    scroll_index: &mut i32,
    active: &mut i32,
) -> i32 {
    let c_text = cstr!(text);
    unsafe {
        ffi::GuiListView(
            bounds,
            c_text.as_ptr() as *mut c_char,
            scroll_index as *mut c_int,
            active as *mut c_int,
        )
    }
}

/// List View with extended parameters
pub fn GuiListViewEx(
    bounds: Rectangle,
    text: &[&str],
    scroll_index: &mut i32,
    active: &mut i32,
    focus: &mut i32,
) -> i32 {
    let c_strings: Vec<CString> = text
        .iter()
        .map(|s| CString::new(*s).expect("Failed to create CString for ListViewEx"))
        .collect();
    let c_pointers: Vec<*const c_char> = c_strings.iter().map(|cs| cs.as_ptr()).collect();

    unsafe {
        ffi::GuiListViewEx(
            bounds,
            c_pointers.as_ptr() as *mut *const i8,
            text.len() as c_int,
            scroll_index as *mut c_int,
            active as *mut c_int,
            focus as *mut c_int,
        )
    }
}

/// Message Box control, displays a message
pub fn GuiMessageBox(bounds: Rectangle, title: &str, message: &str, buttons: &str) -> i32 {
    let c_title = CString::new(title).expect("Failed to create CString for title");
    let c_message = CString::new(message).expect("Failed to create CString for message");
    let c_buttons = CString::new(buttons).expect("Failed to create CString for buttons");
    unsafe {
        ffi::GuiMessageBox(
            bounds,
            c_title.as_ptr(),
            c_message.as_ptr(),
            c_buttons.as_ptr(),
        )
    }
}

/// Text Input Box control, ask for text, supports secret. `text` is the string buffer.
pub fn GuiTextInputBox(
    bounds: Rectangle,
    title: &str,
    message: &str,
    buttons: &str,
    text: &mut [u8],
    secret_view_active: &mut bool,
) -> i32 {
    let c_title = CString::new(title).expect("Failed to create CString for title");
    let c_message = CString::new(message).expect("Failed to create CString for message");
    let c_buttons = CString::new(buttons).expect("Failed to create CString for buttons");
    unsafe {
        ffi::GuiTextInputBox(
            bounds,
            c_title.as_ptr(),
            c_message.as_ptr(),
            c_buttons.as_ptr(),
            text.as_mut_ptr() as *mut c_char,
            text.len() as c_int,
            secret_view_active as *mut bool,
        )
    }
}

/// Color Picker control (multiple color controls)
pub fn GuiColorPicker(bounds: Rectangle, text: Option<&str>, color: &mut Color) -> i32 {
    let c_text = cstr!(text);
    unsafe { ffi::GuiColorPicker(bounds, c_text.as_ptr() as *mut c_char, color as *mut Color) }
}

/// Color Panel control
pub fn GuiColorPanel(bounds: Rectangle, text: Option<&str>, color: &mut Color) -> i32 {
    let c_text = cstr!(text);
    unsafe { ffi::GuiColorPanel(bounds, c_text.as_ptr() as *mut c_char, color as *mut Color) }
}

/// Color Bar Alpha control
pub fn GuiColorBarAlpha(bounds: Rectangle, text: Option<&str>, alpha: &mut f32) -> i32 {
    let c_text = cstr!(text);
    unsafe { ffi::GuiColorBarAlpha(bounds, c_text.as_ptr() as *mut c_char, alpha as *mut f32) }
}

/// Color Bar Hue control
pub fn GuiColorBarHue(bounds: Rectangle, text: Option<&str>, value: &mut f32) -> i32 {
    let c_text = cstr!(text);
    unsafe { ffi::GuiColorBarHue(bounds, c_text.as_ptr() as *mut c_char, value as *mut f32) }
}

/// Color Picker control that avoids conversion to RGB on each call
pub fn GuiColorPickerHSV(bounds: Rectangle, text: Option<&str>, color_hsv: &mut Vector3) -> i32 {
    let c_text = cstr!(text);
    unsafe {
        ffi::GuiColorPickerHSV(
            bounds,
            c_text.as_ptr() as *mut c_char,
            color_hsv as *mut Vector3,
        )
    }
}

/// Color Panel control that updates Hue-Saturation-Value color value
pub fn GuiColorPanelHSV(bounds: Rectangle, text: Option<&str>, color_hsv: &mut Vector3) -> i32 {
    let c_text = cstr!(text);
    unsafe {
        ffi::GuiColorPanelHSV(
            bounds,
            c_text.as_ptr() as *mut c_char,
            color_hsv as *mut Vector3,
        )
    }
}
