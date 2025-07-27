use raylib_egui_rs::math::*;
use raylib_egui_rs::raygui::*;
use raylib_egui_rs::*;

const SCREEN_WIDTH: i32 = 960;
const SCREEN_HEIGHT: i32 = 560;

struct Params {
    dropdownBox000Active: i32,
    dropDown000EditMode: bool,

    dropdownBox001Active: i32,
    dropDown001EditMode: bool,

    spinner001Value: i32,
    spinnerEditMode: bool,

    valueBox002Value: i32,
    valueBoxEditMode: bool,
    textBoxText: String,
    textBoxEditMode: bool,

    textBoxMultiText: String,
    textBoxMultiEditMode: bool,

    listViewScrollIndex: i32,
    listViewActive: i32,

    listViewExScrollIndex: i32,
    listViewExActive: i32,
    listViewExFocus: i32,
    listViewExList: Vec<&'static str>,

    colorPickerValue: crate::color::Color,

    sliderValue: f32,
    sliderBarValue: f32,
    progressValue: f32,

    forceSquaredChecked: bool,

    alphaValue: f32,

    //i32 comboBoxActive = 1;
    visualStyleActive: i32,
    prevVisualStyleActive: i32,

    toggleActive: bool,
    toggleGroupActive: i32,
    toggleSliderActive: i32,

    viewScroll: Vector2,
    //----------------------------------------------------------------------------------

    // Custom GUI font loading
    //Font font = LoadFontEx("fonts/rainyhearts16.ttf", 12, 0, 0);
    //GuiSetFont(font);
    exitWindow: bool,
    showMessageBox: bool,

    textInput: [u8; 256],
    textInputFileName: [u8; 256],
    showTextInputBox: bool,

    alpha: f32,
}

impl Params {
    pub fn new() -> Self {
        Self {
            dropdownBox000Active: 0,
            dropDown000EditMode: false,

            dropdownBox001Active: 0,
            dropDown001EditMode: false,

            spinner001Value: 0,
            spinnerEditMode: false,

            valueBox002Value: 0,
            valueBoxEditMode: false,

            textBoxText: String::from("Text box"),
            textBoxEditMode: false,

            textBoxMultiText: String::from(
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.\n\nDuis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.\n\nThisisastringlongerthanexpectedwithoutspacestotestcharbreaksforthosecases,checkingifworkingasexpected.\n\nExcepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
            ),
            textBoxMultiEditMode: false,

            listViewScrollIndex: 0,
            listViewActive: -1,

            listViewExScrollIndex: 0,
            listViewExActive: 2,
            listViewExFocus: -1,
            listViewExList: vec![
                "This",
                "is",
                "a",
                "list view",
                "with",
                "disable",
                "elements",
                "amazing!",
            ],

            colorPickerValue: color::Color::RED,

            sliderValue: 50.0,
            sliderBarValue: 60.0,
            progressValue: 0.1,

            forceSquaredChecked: false,

            alphaValue: 0.5,

            //int comboBoxActive = 1;
            visualStyleActive: 0,
            prevVisualStyleActive: 0,

            toggleActive: false,
            toggleGroupActive: 0,
            toggleSliderActive: 0,

            viewScroll: Vector2 { x: 0.0, y: 0.0 },
            //----------------------------------------------------------------------------------

            // Custom GUI font loading
            //Font font = LoadFontEx("fonts/rainyhearts16.ttf", 12, 0, 0);
            //GuiSetFont(font);
            exitWindow: false,
            showMessageBox: false,

            textInput: [0 as u8; 256],
            textInputFileName: [0 as u8; 256],
            showTextInputBox: false,

            alpha: 1.0,
        }
    }
}

pub fn main() {
    InitWindow(SCREEN_WIDTH, SCREEN_HEIGHT, "raygui - controls test suite");
    SetTargetFPS(60);

    // let mut show_message_box = 0;
    let mut params = Params::new();
    while !WindowShouldClose() {
        if IsKeyPressed(KeyboardKey::KEY_ESCAPE) {
            params.showMessageBox = !params.showMessageBox
        };

        if (IsKeyDown(KeyboardKey::KEY_LEFT_CONTROL) && IsKeyPressed(KeyboardKey::KEY_S)) {
            params.showTextInputBox = true
        };

        // TODO:
        // if (IsFileDropped())
        // {
        //     FilePathList droppedFiles = LoadDroppedFiles();
        //
        //     if ((droppedFiles.count > 0) && IsFileExtension(droppedFiles.paths[0], ".rgs")) GuiLoadStyle(droppedFiles.paths[0]);
        //
        //     UnloadDroppedFiles(droppedFiles);    // Clear internal buffers
        // }

        //alpha -= 0.002f;
        if (params.alpha < 0.0) {
            params.alpha = 0.0
        };
        if (IsKeyPressed(KeyboardKey::KEY_SPACE)) {
            params.alpha = 1.0
        };

        GuiSetAlpha(params.alpha);

        //progressValue += 0.002f;
        if (IsKeyPressed(KeyboardKey::KEY_LEFT)) {
            params.progressValue -= 0.1
        } else if (IsKeyPressed(KeyboardKey::KEY_RIGHT)) {
            params.progressValue += 0.1
        }
        if (params.progressValue > 1.0) {
            params.progressValue = 1.0
        } else if (params.progressValue < 0.0) {
            params.progressValue = 0.0
        };

        if params.visualStyleActive != params.prevVisualStyleActive {
            GuiLoadStyleDefault();

            match params.visualStyleActive {
                1 => GuiLoadStyleJungle(),
                2 => GuiLoadStyleLavanda(),
                3 => GuiLoadStyleDark(),
                4 => GuiLoadStyleBluish(),
                5 => GuiLoadStyleCyber(),
                6 => GuiLoadStyleTerminal(),
                7 => GuiLoadStyleCandy(),
                8 => GuiLoadStyleCherry(),
                9 => GuiLoadStyleAshes(),
                10 => GuiLoadStyleEnefete(),
                11 => GuiLoadStyleSunny(),
                12 => GuiLoadStyleAmber(),
                13 => GuiLoadStyleGenesis(),
                14 => GuiLoadStyleRLTech(),
                _ => {}
            }

            GuiSetStyle(
                GuiControl::LABEL,
                GuiControlProperty::TEXT_ALIGNMENT as i32,
                GuiTextAlignment::TEXT_ALIGN_LEFT as i32,
            );

            params.prevVisualStyleActive = params.visualStyleActive;
        }

        //----------------------------------------------------------------------------------
        // Draw
        //----------------------------------------------------------------------------------
        BeginDrawing();
        ClearBackground(color::Color::from(GuiGetStyle(
            GuiControl::DEFAULT,
            GuiDefaultProperty::BACKGROUND_COLOR as i32,
        )));

        // raygui: controls drawing
        //----------------------------------------------------------------------------------
        // Check all possible events that require GuiLock
        if params.dropDown000EditMode || params.dropDown001EditMode {
            GuiLock()
        };

        // First GUI column
        GuiSetStyle(
            GuiControl::CHECKBOX,
            GuiControlProperty::TEXT_ALIGNMENT as i32,
            GuiTextAlignment::TEXT_ALIGN_LEFT as i32,
        );
        let text_check = "FORCE_CHECK!";
        GuiCheckBox(
            Rectangle {
                x: 25.0,
                y: 108.0,
                width: 15.0,
                height: 15.0,
            },
            Some(text_check),
            &mut params.forceSquaredChecked,
        );

        GuiSetStyle(
            GuiControl::TEXTBOX,
            GuiControlProperty::TEXT_ALIGNMENT as i32,
            GuiTextAlignment::TEXT_ALIGN_CENTER as i32,
        );
        GuiSetStyle(
            GuiControl::VALUEBOX,
            GuiControlProperty::TEXT_ALIGNMENT as i32,
            GuiTextAlignment::TEXT_ALIGN_LEFT as i32,
        );
        if (GuiSpinner(
            Rectangle {
                x: 25.0,
                y: 135.0,
                width: 125.0,
                height: 30.0,
            },
            Some(""), // TODO: None,
            &mut params.spinner001Value,
            0,
            100,
            params.spinnerEditMode,
        )) != 0
        {
            params.spinnerEditMode = !params.spinnerEditMode
        };
        if (GuiValueBox(
            Rectangle {
                x: 25.0,
                y: 175.0,
                width: 125.0,
                height: 30.0,
            },
            None,
            &mut params.valueBox002Value,
            0,
            100,
            params.valueBoxEditMode,
        )) != 0
        {
            params.valueBoxEditMode = !params.valueBoxEditMode
        };
        GuiSetStyle(
            GuiControl::TEXTBOX,
            GuiControlProperty::TEXT_ALIGNMENT as i32,
            GuiTextAlignment::TEXT_ALIGN_LEFT as i32,
        );
        if (GuiTextBox(
            Rectangle {
                x: 25.0,
                y: 215.0,
                width: 125.0,
                height: 30.0,
            },
            &mut params.textBoxText,
            64,
            params.textBoxEditMode,
        )) != 0
        {
            params.textBoxEditMode = !params.textBoxEditMode
        };

        GuiSetStyle(
            GuiControl::BUTTON,
            GuiControlProperty::TEXT_ALIGNMENT as i32,
            GuiTextAlignment::TEXT_ALIGN_CENTER as i32,
        );

        if (GuiButton(
            Rectangle {
                x: 25.0,
                y: 255.0,
                width: 125.0,
                height: 30.0,
            },
            Some(GuiIconText(GuiIconName::ICON_FILE_SAVE as i32, Some("Save File")).as_str()),
        )) != 0
        {
            params.showTextInputBox = true
        };

        GuiGroupBox(
            Rectangle {
                x: 25.0,
                y: 310.0,
                width: 125.0,
                height: 150.0,
            },
            Some("STATES"),
        );
        //GuiLock();
        GuiSetState(GuiState::STATE_NORMAL);
        if (GuiButton(
            Rectangle {
                x: 30.0,
                y: 320.0,
                width: 115.0,
                height: 30.0,
            },
            Some("NORMAL"),
        )) != 0
        {}
        GuiSetState(GuiState::STATE_FOCUSED);
        if (GuiButton(
            Rectangle {
                x: 30.0,
                y: 355.0,
                width: 115.0,
                height: 30.0,
            },
            Some("FOCUSED"),
        )) != 0
        {}
        GuiSetState(GuiState::STATE_PRESSED);
        if (GuiButton(
            Rectangle {
                x: 30.0,
                y: 390.0,
                width: 115.0,
                height: 30.0,
            },
            Some("#15#PRESSED"),
        )) != 0
        {}
        GuiSetState(GuiState::STATE_DISABLED);
        if (GuiButton(
            Rectangle {
                x: 30.0,
                y: 425.0,
                width: 115.0,
                height: 30.0,
            },
            Some("DISABLED"),
        )) != 0
        {}

        GuiSetState(GuiState::STATE_NORMAL);
        GuiUnlock();

        GuiComboBox(
            Rectangle {
                x: 25.0,
                y: 480.0,
                width: 125.0,
                height: 30.0,
            },
            "default;Jungle;Lavanda;Dark;Bluish;Cyber;Terminal;Candy;Cherry;Ashes;Enefete;Sunny;Amber;Genesis;RLTech",
            &mut params.visualStyleActive,
        );

        // NOTE: GuiDropdownBox must draw after any other control that can be covered on unfolding
        GuiUnlock();
        GuiSetStyle(
            GuiControl::DROPDOWNBOX,
            GuiControlProperty::TEXT_PADDING as i32,
            4,
        );
        GuiSetStyle(
            GuiControl::DROPDOWNBOX,
            GuiControlProperty::TEXT_ALIGNMENT as i32,
            GuiTextAlignment::TEXT_ALIGN_LEFT as i32,
        );
        if (GuiDropdownBox(
            Rectangle {
                x: 25.0,
                y: 65.0,
                width: 125.0,
                height: 30.0,
            },
            "#01#ONE;#02#TWO;#03#THREE;#04#FOUR",
            &mut params.dropdownBox001Active,
            params.dropDown001EditMode,
        )) != 0
        {
            params.dropDown001EditMode = !params.dropDown001EditMode
        };
        GuiSetStyle(
            GuiControl::DROPDOWNBOX,
            GuiControlProperty::TEXT_ALIGNMENT as i32,
            GuiTextAlignment::TEXT_ALIGN_CENTER as i32,
        );
        GuiSetStyle(
            GuiControl::DROPDOWNBOX,
            GuiControlProperty::TEXT_PADDING as i32,
            0,
        );

        if (GuiDropdownBox(
            Rectangle {
                x: 25.0,
                y: 25.0,
                width: 125.0,
                height: 30.0,
            },
            "ONE;TWO;THREE",
            &mut params.dropdownBox000Active,
            params.dropDown000EditMode,
        )) != 0
        {
            params.dropDown000EditMode = !params.dropDown000EditMode
        };

        // Second GUI column
        GuiSetStyle(
            GuiControl::LISTVIEW,
            GuiListViewProperty::LIST_ITEMS_BORDER_NORMAL as i32,
            1,
        );
        GuiListView(
            Rectangle {
                x: 165.0,
                y: 25.0,
                width: 140.0,
                height: 124.0,
            },
            Some("Charmander;Bulbasaur;#18#Squirtel;Pikachu;Eevee;Pidgey"),
            &mut params.listViewScrollIndex,
            &mut params.listViewActive,
        );
        GuiListViewEx(
            Rectangle {
                x: 165.0,
                y: 162.0,
                width: 140.0,
                height: 184.0,
            },
            params.listViewExList.as_slice(),
            // &mut 8,
            &mut params.listViewExScrollIndex,
            &mut params.listViewExActive,
            &mut params.listViewExFocus,
        );
        GuiSetStyle(
            GuiControl::LISTVIEW,
            GuiListViewProperty::LIST_ITEMS_BORDER_NORMAL as i32,
            0,
        );

        GuiToggle(
            Rectangle {
                x: 165.0,
                y: 400.0,
                width: 140.0,
                height: 25.0,
            },
            Some("#1#ONE"),
            &mut params.toggleActive,
        );
        GuiToggleGroup(
            Rectangle {
                x: 165.0,
                y: 360.0,
                width: 140.0,
                height: 24.0,
            },
            "#1#ONE\n#3#TWO\n#8#THREE\n#23#",
            &mut params.toggleGroupActive,
        );
        //GuiDisable();
        GuiSetStyle(
            GuiControl::SLIDER,
            GuiSliderProperty::SLIDER_PADDING as i32,
            2,
        );
        GuiToggleSlider(
            Rectangle {
                x: 165.0,
                y: 480.0,
                width: 140.0,
                height: 30.0,
            },
            "ON;OFF",
            &mut params.toggleSliderActive,
        );
        GuiSetStyle(
            GuiControl::SLIDER,
            GuiSliderProperty::SLIDER_PADDING as i32,
            0,
        );

        // Third GUI column
        GuiPanel(
            Rectangle {
                x: 320.0,
                y: 25.0,
                width: 225.0,
                height: 140.0,
            },
            Some("Panel Info"),
        );
        GuiColorPicker(
            Rectangle {
                x: 320.0,
                y: 185.0,
                width: 196.0,
                height: 192.0,
            },
            Some(""), // TODO: None,
            &mut params.colorPickerValue,
        );

        //GuiDisable();
        GuiSlider(
            Rectangle {
                x: 355.0,
                y: 400.0,
                width: 165.0,
                height: 20.0,
            },
            Some("TEST"),
            Some(format!("{:2.2}", params.sliderValue).as_str()),
            &mut params.sliderValue,
            -50.0,
            100.0,
        );
        GuiSliderBar(
            Rectangle {
                x: 320.0,
                y: 430.0,
                width: 200.0,
                height: 20.0,
            },
            Some(""), // TODO: None,
            Some(format!("{}", params.sliderBarValue).as_str()),
            &mut params.sliderBarValue,
            0.0,
            100.0,
        );

        GuiProgressBar(
            Rectangle {
                x: 320.0,
                y: 460.0,
                width: 200.0,
                height: 20.0,
            },
            Some(""), // TODO: None,
            Some(format!("{}", params.progressValue * 100.0).as_str()),
            &mut params.progressValue,
            0.0,
            1.0,
        );
        GuiEnable();

        // NOTE: View rectangle could be used to perform some scissor test
        let mut view = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        };
        GuiScrollPanel(
            Rectangle {
                x: 560.0,
                y: 25.0,
                width: 102.0,
                height: 354.0,
            },
            Some(""), // TODO: None,
            Rectangle {
                x: 560.0,
                y: 25.0,
                width: 300.0,
                height: 1200.0,
            },
            &mut params.viewScroll,
            &mut view,
        );

        let mut mouseCell = Vector2 { x: 0.0, y: 0.0 };
        GuiGrid(
            Rectangle {
                x: 560.0,
                y: 25.0 + 180.0 + 195.0,
                width: 100.0,
                height: 120.0,
            },
            Some(""), // TODO: None,
            20.0,
            3,
            &mut mouseCell,
        );

        GuiColorBarAlpha(
            Rectangle {
                x: 320.0,
                y: 490.0,
                width: 200.0,
                height: 30.0,
            },
            Some(""), // TODO: None,
            &mut params.alphaValue,
        );

        GuiSetStyle(
            GuiControl::DEFAULT,
            GuiDefaultProperty::TEXT_ALIGNMENT_VERTICAL as i32,
            GuiTextAlignmentVertical::TEXT_ALIGN_TOP as i32,
        ); // WARNING: Word-wrap does not work as expected in case of no-top alignment
        GuiSetStyle(
            GuiControl::DEFAULT,
            GuiDefaultProperty::TEXT_WRAP_MODE as i32,
            GuiTextWrapMode::TEXT_WRAP_WORD as i32,
        ); // WARNING: If wrap mode enabled, text editing is not supported
        if (GuiTextBox(
            Rectangle {
                x: 678.0,
                y: 25.0,
                width: 258.0,
                height: 492.0,
            },
            &mut params.textBoxMultiText,
            1024,
            params.textBoxMultiEditMode,
        )) != 0
        {
            params.textBoxMultiEditMode = !params.textBoxMultiEditMode
        };
        GuiSetStyle(
            GuiControl::DEFAULT,
            GuiDefaultProperty::TEXT_WRAP_MODE as i32,
            GuiTextWrapMode::TEXT_WRAP_NONE as i32,
        );
        GuiSetStyle(
            GuiControl::DEFAULT,
            GuiDefaultProperty::TEXT_ALIGNMENT_VERTICAL as i32,
            GuiTextAlignmentVertical::TEXT_ALIGN_MIDDLE as i32,
        );

        GuiSetStyle(
            GuiControl::DEFAULT,
            GuiControlProperty::TEXT_ALIGNMENT as i32,
            GuiTextAlignment::TEXT_ALIGN_LEFT as i32,
        );
        GuiStatusBar(
            Rectangle {
                x: 0.0,
                y: GetScreenHeight() as f32 - 20.0,
                width: GetScreenWidth() as f32,
                height: 20.0,
            },
            Some("This is a status bar"),
        );
        GuiSetStyle(
            GuiControl::DEFAULT,
            GuiControlProperty::TEXT_ALIGNMENT as i32,
            GuiTextAlignment::TEXT_ALIGN_CENTER as i32,
        );
        // GuiSetStyle(GuiControl::STATUSBAR, TEXT_INDENTATION, 20);

        if params.showMessageBox {
            DrawRectangle(
                0,
                0,
                GetScreenWidth(),
                GetScreenHeight(),
                color::Color::WHITE,
            ); //TODO: Fade(RAYWHITE, 0.8f));
            let result = GuiMessageBox(
                Rectangle {
                    x: GetScreenWidth() as f32 / 2.0 - 125.0,
                    y: GetScreenHeight() as f32 / 2.0 - 50.0,
                    width: 250.0,
                    height: 100.0,
                },
                GuiIconText(GuiIconName::ICON_EXIT as i32, Some("Close Window")).as_str(),
                "Do you really want to exit?",
                "Yes;No",
            );

            if ((result == 0) || (result == 2)) {
                params.showMessageBox = false
            } else if (result == 1) {
                params.exitWindow = true
            };
        }

        if (params.showTextInputBox) {
            DrawRectangle(
                0,
                0,
                GetScreenWidth(),
                GetScreenHeight(),
                color::Color::WHITE,
            ); //TODO Fade(RAYWHITE, 0.8f));
            let result = GuiTextInputBox(
                Rectangle {
                    x: GetScreenWidth() as f32 / 2.0 - 120.0,
                    y: GetScreenHeight() as f32 / 2.0 - 60.0,
                    width: 240.0,
                    height: 140.0,
                },
                GuiIconText(GuiIconName::ICON_FILE_SAVE as i32, Some("Save file as...")).as_str(),
                "Introduce output file name:",
                "Ok;Cancel",
                &mut params.textInput,
                // 255,
                &mut false,
            );

            if result == 1 {
                // TODO: Validate textInput value and save

                // TODO TextCopy(textInputFileName, textInput);
            }

            if ((result == 0) || (result == 1) || (result == 2)) {
                params.showTextInputBox = false;
                // TODO TextCopy(textInput, "\0");
            }
        }
        //----------------------------------------------------------------------------------

        EndDrawing();
    }
    CloseWindow();
}
