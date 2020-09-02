use druid::widget::{Align, Button, Flex, Label, Padding, TextBox};
use druid::{AppLauncher, Data, Lens, LocalizedString, Widget, WidgetExt, WindowDesc};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const HORIZTONAL_WIDGET_SPACING: f64 = 8.0;
const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Rakaly");

#[derive(Clone, Data, Lens)]
struct HelloState {
    steam_name: String,
    api_key: String,
}

pub fn run() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state = HelloState {
        steam_name: "".into(),
        api_key: "".into(),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<HelloState> {
    let watch_dir = if let Some(user_dirs) = directories::UserDirs::new() {
        user_dirs
            .document_dir()
            .map(|x| {
                x.join("Paradox Interactive")
                    .join("Europa Universalis IV")
                    .join("save games")
            })
            .map(|x| x.display().to_string())
            .unwrap_or_else(|| String::from("unknown"))
    } else {
        String::from("unknown")
    };

    let intro_text_1 = Label::new("When the \"Start\" button is pressed, Rakaly will");
    let intro_text_2 = Label::new("automatically start watching the following directory");
    let intro_text_3 = Label::new("for any changes:");
    let intro_text_4 = Label::new(watch_dir);
    let intro_text_5 = Label::new("Rakaly will upload the new files to the server.");
    let intro_text_6 = Label::new("To start the uploader on boot, click \"Enable on Startup\"");

    let steam_name_box = TextBox::new().expand_width().lens(HelloState::steam_name);

    let steam_name_row = Flex::row()
        .must_fill_main_axis(true)
        .with_child(Label::new("Steam Name:"))
        .with_spacer(HORIZTONAL_WIDGET_SPACING)
        .with_flex_child(steam_name_box, 1.0);

    let api_key_box = TextBox::new().expand_width().lens(HelloState::api_key);

    let api_key_row = Flex::row()
        .must_fill_main_axis(true)
        .with_child(Label::new("API Key:"))
        .with_spacer(HORIZTONAL_WIDGET_SPACING)
        .with_flex_child(api_key_box, 1.0);

    let immediate_btn_layout = Flex::row()
        .with_child(Button::new("Start").on_click(|_ctx, _data: &mut HelloState, _env| {}))
        .with_spacer(HORIZTONAL_WIDGET_SPACING)
        .with_child(Button::new("Stop").on_click(|_ctx, _data: &mut HelloState, _env| {}));

    let service_btn_layout = Flex::row()
        .with_child(
            Button::new("Enable on Startup").on_click(|_ctx, _data: &mut HelloState, _env| {}),
        )
        .with_spacer(HORIZTONAL_WIDGET_SPACING)
        .with_child(
            Button::new("Disable on Startup").on_click(|_ctx, _data: &mut HelloState, _env| {}),
        );

    let layout = Flex::column()
        .with_child(intro_text_1)
        .with_child(intro_text_2)
        .with_child(intro_text_3)
        .with_child(intro_text_4)
        .with_child(intro_text_5)
        .with_child(intro_text_6)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(steam_name_row)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(api_key_row)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(immediate_btn_layout)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(service_btn_layout)
        .expand_width();

    Padding::new(10.0, Align::left(layout))
}
