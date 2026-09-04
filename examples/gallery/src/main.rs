//! RustTheme Gallery: a scrollable showcase of every component in the
//! `rustheme` crate, with a light/dark theme toggle in the app bar.

use rustheme::prelude::*;

fn main() -> iced::Result {
    iced::application(Gallery::title, Gallery::update, Gallery::view)
        .theme(Gallery::theme)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    ToggleDarkMode,
    CheckboxToggled(bool),
    RadioSelected(Fruit),
    SwitchToggled(bool),
    SliderChanged(f32),
    SelectPicked(Framework),
    TabSelected(usize),
    OpenDialog,
    CloseDialog,
    ShowSnackbar,
    DismissSnackbar,
    OpenMenu,
    CloseMenu,
    MenuItemSelected(usize),
    TextChanged(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fruit {
    Apple,
    Banana,
    Cherry,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Framework {
    Iced,
    Egui,
    Druid,
}

impl std::fmt::Display for Framework {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Iced => "Iced",
            Self::Egui => "egui",
            Self::Druid => "Druid",
        })
    }
}

const FRAMEWORKS: [Framework; 3] = [Framework::Iced, Framework::Egui, Framework::Druid];
const TAB_LABELS: [&str; 3] = ["Overview", "Components", "Theming"];
const MENU_ITEMS: [&str; 3] = ["Profile", "Settings", "Sign out"];

struct Gallery {
    dark_mode: bool,
    checkbox_checked: bool,
    radio_selected: Option<Fruit>,
    switch_on: bool,
    slider_value: f32,
    select_value: Option<Framework>,
    active_tab: usize,
    dialog_open: bool,
    snackbar_visible: bool,
    menu_open: bool,
    text_value: String,
    last_menu_selection: Option<usize>,
}

impl Default for Gallery {
    fn default() -> Self {
        Self {
            dark_mode: false,
            checkbox_checked: true,
            radio_selected: Some(Fruit::Banana),
            switch_on: true,
            slider_value: 40.0,
            select_value: Some(Framework::Iced),
            active_tab: 0,
            dialog_open: false,
            snackbar_visible: false,
            menu_open: false,
            text_value: String::new(),
            last_menu_selection: None,
        }
    }
}

impl Gallery {
    fn title(&self) -> String {
        "RustTheme Gallery".to_string()
    }

    fn theme(&self) -> RustTheme {
        if self.dark_mode {
            RustTheme::dark()
        } else {
            RustTheme::light()
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleDarkMode => self.dark_mode = !self.dark_mode,
            Message::CheckboxToggled(checked) => self.checkbox_checked = checked,
            Message::RadioSelected(fruit) => self.radio_selected = Some(fruit),
            Message::SwitchToggled(on) => self.switch_on = on,
            Message::SliderChanged(value) => self.slider_value = value,
            Message::SelectPicked(framework) => self.select_value = Some(framework),
            Message::TabSelected(index) => self.active_tab = index,
            Message::OpenDialog => self.dialog_open = true,
            Message::CloseDialog => self.dialog_open = false,
            Message::ShowSnackbar => self.snackbar_visible = true,
            Message::DismissSnackbar => self.snackbar_visible = false,
            Message::OpenMenu => self.menu_open = true,
            Message::CloseMenu => self.menu_open = false,
            Message::MenuItemSelected(index) => {
                self.last_menu_selection = Some(index);
                self.menu_open = false;
            }
            Message::TextChanged(value) => self.text_value = value,
        }
    }

    fn view(&self) -> Element<'_, Message, RustTheme> {
        let theme = self.theme();

        let bar = app_bar::app_bar(
            "RustTheme Gallery",
            vec![button::text_button(
                if self.dark_mode { "Light mode" } else { "Dark mode" },
                Message::ToggleDarkMode,
            )
            .into()],
        );

        fn section<'a>(
            title: &str,
            content: Element<'a, Message, RustTheme>,
        ) -> iced::widget::Container<'a, Message, RustTheme> {
            card::card(
                iced::widget::column![
                    iced::widget::text(title.to_string()).size(18),
                    content,
                ]
                .spacing(12),
            )
        }

        let buttons_section = section(
            "Buttons",
            iced::widget::row![
                button::primary("Primary", Message::ToggleDarkMode),
                button::secondary("Secondary", Message::ToggleDarkMode),
                button::outlined("Outlined", Message::ToggleDarkMode),
                button::text_button("Text", Message::ToggleDarkMode),
                button::danger("Danger", Message::ToggleDarkMode),
                button::success("Success", Message::ToggleDarkMode),
            ]
            .spacing(8)
            .wrap()
            .into(),
        );

        let text_field_section = section(
            "Text Fields",
            iced::widget::row![
                text_field::outlined("Outlined", &self.text_value, Message::TextChanged),
                text_field::filled("Filled", &self.text_value, Message::TextChanged),
            ]
            .spacing(12)
            .into(),
        );

        let checkbox_section = section(
            "Checkbox",
            checkbox::checkbox("Accept terms", self.checkbox_checked, Message::CheckboxToggled).into(),
        );

        let radio_section = section(
            "Radio",
            iced::widget::row![
                radio::radio("Apple", Fruit::Apple, self.radio_selected, Message::RadioSelected),
                radio::radio("Banana", Fruit::Banana, self.radio_selected, Message::RadioSelected),
                radio::radio("Cherry", Fruit::Cherry, self.radio_selected, Message::RadioSelected),
            ]
            .spacing(16)
            .into(),
        );

        let switch_section = section(
            "Switch",
            switch::switch(self.switch_on, Message::SwitchToggled).label("Enable notifications").into(),
        );

        let slider_section = section(
            "Slider",
            iced::widget::column![
                slider::slider(0.0..=100.0, self.slider_value, Message::SliderChanged),
                iced::widget::text(format!("{:.0}", self.slider_value)),
            ]
            .spacing(6)
            .into(),
        );

        let select_section = section(
            "Select",
            select::select(&FRAMEWORKS[..], self.select_value, Message::SelectPicked).into(),
        );

        let divider_section = section(
            "Divider",
            iced::widget::column![
                iced::widget::text("Above"),
                divider::horizontal(1),
                iced::widget::text("Below"),
            ]
            .spacing(8)
            .into(),
        );

        let tooltip_section = section(
            "Tooltip",
            tooltip::tooltip(button::outlined("Hover me", Message::ToggleDarkMode), "This is a tooltip").into(),
        );

        let card_section = section(
            "Cards",
            iced::widget::row![
                card::card(iced::widget::text("Card")).width(iced::Length::Fill),
                card::elevated(iced::widget::text("Elevated")).width(iced::Length::Fill),
                card::outlined(iced::widget::text("Outlined")).width(iced::Length::Fill),
            ]
            .spacing(12)
            .into(),
        );

        let chip_section = section(
            "Chips",
            iced::widget::row![chip::filled("Filled"), chip::outlined("Outlined")]
                .spacing(8)
                .into(),
        );

        let avatar_section = section("Avatar", avatar::initials("RT", 48.0).into());

        let badge_section = section(
            "Badge",
            iced::widget::row![
                badge::count(iced::widget::text("Inbox"), 7),
                badge::dot(iced::widget::text("Alerts")),
            ]
            .spacing(24)
            .into(),
        );

        let list_section = section(
            "List",
            list::list(vec![
                list::item(iced::widget::text("Item one")).into(),
                list::item(iced::widget::text("Item two")).into(),
                list::item(iced::widget::text("Item three")).into(),
            ])
            .into(),
        );

        let progress_section = section(
            "Progress",
            iced::widget::row![
                progress::linear::<Message>(0.6).width(200),
                progress::circular::<Message>(&theme, 0.6, 48.0),
            ]
            .spacing(24)
            .align_y(iced::alignment::Vertical::Center)
            .into(),
        );

        let tabs_section = section(
            "Tabs",
            iced::widget::column![
                tabs::tabs(&TAB_LABELS, self.active_tab, Message::TabSelected),
                iced::widget::text(format!("Active tab: {}", TAB_LABELS[self.active_tab])),
            ]
            .spacing(8)
            .into(),
        );

        let menu_status = match self.last_menu_selection {
            Some(index) => format!("Last selected: {}", MENU_ITEMS[index]),
            None => "Nothing selected yet".to_string(),
        };
        let menu_section = section(
            "Menu",
            iced::widget::column![
                menu::menu(
                    button::outlined("Open menu", Message::OpenMenu),
                    self.menu_open,
                    &MENU_ITEMS,
                    Message::MenuItemSelected,
                    Message::CloseMenu,
                ),
                iced::widget::text(menu_status),
            ]
            .spacing(8)
            .into(),
        );

        let dialog_snackbar_section = section(
            "Dialog & Snackbar",
            iced::widget::row![
                button::primary("Open dialog", Message::OpenDialog),
                button::secondary("Show snackbar", Message::ShowSnackbar),
            ]
            .spacing(8)
            .into(),
        );

        let content = iced::widget::scrollable(
            iced::widget::column![
                buttons_section,
                text_field_section,
                checkbox_section,
                radio_section,
                switch_section,
                slider_section,
                select_section,
                divider_section,
                tooltip_section,
                card_section,
                chip_section,
                avatar_section,
                badge_section,
                list_section,
                progress_section,
                tabs_section,
                menu_section,
                dialog_snackbar_section,
            ]
            .spacing(20)
            .padding(20),
        );

        let base = iced::widget::column![bar, content];

        let with_dialog = dialog::dialog(
            base,
            self.dialog_open,
            "Confirm action",
            iced::widget::text("Are you sure you want to proceed?"),
            vec![
                button::text_button("Cancel", Message::CloseDialog).into(),
                button::primary("Confirm", Message::CloseDialog).into(),
            ],
        );

        snackbar::snackbar(
            with_dialog,
            self.snackbar_visible,
            "Action completed successfully",
            Message::DismissSnackbar,
        )
    }
}
