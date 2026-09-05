//! RustTheme Gallery: a scrollable showcase of every component in the
//! `rustheme` crate, with a light/dark theme toggle in the app bar.

use rustheme::prelude::*;
use rustheme_core::{system_font, system_monospace_font, FontWeight};

fn main() -> iced::Result {
    // Configuring a custom global font is done once, here, before any UI is
    // built — see `RustTheme::builder()`'s docs for why font is process-wide
    // while the palette stays per-`RustTheme`-instance. Left as the default
    // system font for this gallery; uncomment to try a custom one (the named
    // font must be installed on the system, since RustTheme doesn't bundle
    // font files):
    //
    // let _ = RustTheme::builder().font("Inter").build();

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
    AccordionToggled,
    AutocompleteChanged(String),
    AutocompleteSelected(usize),
    BreadcrumbSelected(usize),
    OpenDrawer,
    CloseDrawer,
    OpenBackdrop,
    CloseBackdrop,
    PageSelected(usize),
    RatingChanged(u8),
    SpeedDialToggled,
    SpeedDialAction(&'static str),
    AlertDismissed,
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
const FRUIT_NAMES: [&str; 6] = ["Apple", "Apricot", "Avocado", "Banana", "Blueberry", "Cherry"];
const BREADCRUMB_ITEMS: [&str; 3] = ["Home", "Components", "Gallery"];

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
    accordion_expanded: bool,
    autocomplete_query: String,
    autocomplete_selected: Option<String>,
    last_breadcrumb: Option<usize>,
    drawer_open: bool,
    backdrop_open: bool,
    current_page: usize,
    rating_value: u8,
    speed_dial_open: bool,
    last_speed_dial_action: Option<&'static str>,
    alert_dismissed: bool,
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
            accordion_expanded: false,
            autocomplete_query: String::new(),
            autocomplete_selected: None,
            last_breadcrumb: None,
            drawer_open: false,
            backdrop_open: false,
            current_page: 1,
            rating_value: 3,
            speed_dial_open: false,
            last_speed_dial_action: None,
            alert_dismissed: false,
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
            Message::AccordionToggled => self.accordion_expanded = !self.accordion_expanded,
            Message::AutocompleteChanged(query) => {
                self.autocomplete_query = query;
                self.autocomplete_selected = None;
            }
            Message::AutocompleteSelected(index) => {
                if let Some(name) = self.autocomplete_matches().get(index) {
                    self.autocomplete_selected = Some((*name).to_string());
                    self.autocomplete_query = (*name).to_string();
                }
            }
            Message::BreadcrumbSelected(index) => self.last_breadcrumb = Some(index),
            Message::OpenDrawer => self.drawer_open = true,
            Message::CloseDrawer => self.drawer_open = false,
            Message::OpenBackdrop => self.backdrop_open = true,
            Message::CloseBackdrop => self.backdrop_open = false,
            Message::PageSelected(page) => self.current_page = page,
            Message::RatingChanged(value) => self.rating_value = value,
            Message::SpeedDialToggled => self.speed_dial_open = !self.speed_dial_open,
            Message::SpeedDialAction(label) => {
                self.last_speed_dial_action = Some(label);
                self.speed_dial_open = false;
            }
            Message::AlertDismissed => self.alert_dismissed = true,
        }
    }

    fn autocomplete_matches(&self) -> Vec<&'static str> {
        let query = self.autocomplete_query.to_lowercase();
        if query.is_empty() {
            return Vec::new();
        }
        FRUIT_NAMES
            .iter()
            .copied()
            .filter(|name| name.to_lowercase().contains(&query))
            .collect()
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
                    iced::widget::text(title.to_string()).size(18).font(system_font(FontWeight::Bold)),
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
                iced::widget::text(format!("{:.0}", self.slider_value)).font(system_monospace_font(FontWeight::Regular)),
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
                iced::widget::text("Above").font(system_font(FontWeight::Regular)),
                divider::horizontal(1),
                iced::widget::text("Below").font(system_font(FontWeight::Regular)),
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
                card::card(iced::widget::text("Card").font(system_font(FontWeight::Regular))).width(iced::Length::Fill),
                card::elevated(iced::widget::text("Elevated").font(system_font(FontWeight::Regular))).width(iced::Length::Fill),
                card::outlined(iced::widget::text("Outlined").font(system_font(FontWeight::Regular))).width(iced::Length::Fill),
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
                badge::count(iced::widget::text("Inbox").font(system_font(FontWeight::Regular)), 7),
                badge::dot(iced::widget::text("Alerts").font(system_font(FontWeight::Regular))),
            ]
            .spacing(24)
            .into(),
        );

        let list_section = section(
            "List",
            list::list(vec![
                list::item(iced::widget::text("Item one").font(system_font(FontWeight::Regular))).into(),
                list::item(iced::widget::text("Item two").font(system_font(FontWeight::Regular))).into(),
                list::item(iced::widget::text("Item three").font(system_font(FontWeight::Regular))).into(),
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
                iced::widget::text(format!("Active tab: {}", TAB_LABELS[self.active_tab])).font(system_font(FontWeight::Regular)),
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
                iced::widget::text(menu_status).font(system_font(FontWeight::Regular)),
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

        let icon_button_section = section(
            "Icon Button",
            iced::widget::row![
                icon_button::primary("★", Message::ToggleDarkMode),
                icon_button::secondary("♥", Message::ToggleDarkMode),
                icon_button::outlined("✎", Message::ToggleDarkMode),
                icon_button::text_icon("✕", Message::ToggleDarkMode),
            ]
            .spacing(8)
            .into(),
        );

        let fab_section = section(
            "Fab",
            iced::widget::row![
                fab::primary("+", Message::ToggleDarkMode),
                fab::secondary("✎", Message::ToggleDarkMode),
                fab::extended("+", "Create", Message::ToggleDarkMode),
            ]
            .spacing(16)
            .align_y(iced::alignment::Vertical::Center)
            .into(),
        );

        let alert_section = section(
            "Alert",
            if self.alert_dismissed {
                iced::widget::column![iced::widget::text("Alert dismissed.").font(system_font(FontWeight::Regular))].into()
            } else {
                iced::widget::column![
                    alert::success("Your changes have been saved.", None::<Message>),
                    alert::info("A new version is available.", None::<Message>),
                    alert::warning("Your session will expire soon.", None::<Message>),
                    alert::with_title(
                        alert::Severity::Error,
                        "Something went wrong",
                        "Please try again.",
                        Some(Message::AlertDismissed),
                    ),
                ]
                .spacing(8)
                .into()
            },
        );

        let accordion_section = section(
            "Accordion",
            accordion::accordion(
                "Click to expand",
                iced::widget::text("Hidden content revealed when expanded.").font(system_font(FontWeight::Regular)),
                self.accordion_expanded,
                Message::AccordionToggled,
            )
            .into(),
        );

        let breadcrumbs_section = section(
            "Breadcrumbs",
            iced::widget::column![
                breadcrumbs::breadcrumbs(&BREADCRUMB_ITEMS, Message::BreadcrumbSelected),
                iced::widget::text(match self.last_breadcrumb {
                    Some(i) => format!("Last clicked: {}", BREADCRUMB_ITEMS[i]),
                    None => "Nothing clicked yet".to_string(),
                }).font(system_font(FontWeight::Regular)),
            ]
            .spacing(8)
            .into(),
        );

        let pagination_section = section(
            "Pagination",
            pagination::pagination(self.current_page, 10, Message::PageSelected).into(),
        );

        let skeleton_section = section(
            "Skeleton",
            iced::widget::row![
                skeleton::text(120.0),
                skeleton::rectangular(80.0, 48.0),
                skeleton::circular(48.0),
            ]
            .spacing(16)
            .align_y(iced::alignment::Vertical::Center)
            .into(),
        );

        let stepper_section = section(
            "Stepper",
            stepper::stepper(&["Account", "Shipping", "Payment", "Review"], self.active_tab.min(3)).into(),
        );

        let rating_section = section(
            "Rating",
            iced::widget::row![
                rating::rating(self.rating_value, 5, Message::RatingChanged),
                iced::widget::text(format!("{}/5", self.rating_value)).font(system_monospace_font(FontWeight::Regular)),
            ]
            .spacing(12)
            .align_y(iced::alignment::Vertical::Center)
            .into(),
        );

        let table_section = section(
            "Table",
            table::table(
                &["Name", "Role", "Status"],
                &[
                    vec!["Ada".to_string(), "Engineer".to_string(), "Active".to_string()],
                    vec!["Grace".to_string(), "Designer".to_string(), "Active".to_string()],
                    vec!["Alan".to_string(), "Researcher".to_string(), "Away".to_string()],
                ],
            )
            .into(),
        );

        let layout_section = section(
            "Grid & Stack",
            iced::widget::column![
                layout::stack(
                    &theme,
                    layout::Direction::Row,
                    1.0,
                    vec![
                        iced::widget::text("Stack item A").font(system_font(FontWeight::Regular)).into(),
                        iced::widget::text("Stack item B").font(system_font(FontWeight::Regular)).into(),
                        iced::widget::text("Stack item C").font(system_font(FontWeight::Regular)).into(),
                    ],
                ),
                Element::from(layout::grid(
                    &theme,
                    3,
                    1.0,
                    vec![
                        card::outlined(iced::widget::text("1").font(system_font(FontWeight::Regular))).into(),
                        card::outlined(iced::widget::text("2").font(system_font(FontWeight::Regular))).into(),
                        card::outlined(iced::widget::text("3").font(system_font(FontWeight::Regular))).into(),
                        card::outlined(iced::widget::text("4").font(system_font(FontWeight::Regular))).into(),
                    ],
                )),
            ]
            .spacing(12)
            .into(),
        );

        let autocomplete_section = section(
            "Autocomplete",
            iced::widget::column![
                autocomplete::autocomplete(
                    "Search fruit…",
                    &self.autocomplete_query,
                    &self
                        .autocomplete_matches()
                        .into_iter()
                        .map(str::to_string)
                        .collect::<Vec<_>>(),
                    Message::AutocompleteChanged,
                    Message::AutocompleteSelected,
                ),
                iced::widget::text(match &self.autocomplete_selected {
                    Some(name) => format!("Selected: {name}"),
                    None => "Nothing selected yet".to_string(),
                }).font(system_font(FontWeight::Regular)),
            ]
            .spacing(8)
            .into(),
        );

        let timeline_section = section(
            "Timeline",
            timeline::timeline(vec![
                timeline::TimelineItem::new("Order placed", "Your order has been received."),
                timeline::TimelineItem::new("Shipped", "Your order is on its way."),
                timeline::TimelineItem::new("Delivered", "Your order has arrived."),
            ])
            .into(),
        );

        let drawer_backdrop_section = section(
            "Drawer & Backdrop",
            iced::widget::row![
                button::primary("Open drawer", Message::OpenDrawer),
                button::secondary("Open backdrop", Message::OpenBackdrop),
            ]
            .spacing(8)
            .into(),
        );

        let speed_dial_section = section(
            "Speed Dial",
            speed_dial::speed_dial(
                "+",
                self.speed_dial_open,
                vec![
                    speed_dial::SpeedDialAction::new("✎", Message::SpeedDialAction("Edit")),
                    speed_dial::SpeedDialAction::new("♥", Message::SpeedDialAction("Favorite")),
                    speed_dial::SpeedDialAction::new("✕", Message::SpeedDialAction("Delete")),
                ],
                Message::SpeedDialToggled,
            )
            .into(),
        );

        let content = iced::widget::scrollable(
            iced::widget::column![
                buttons_section,
                icon_button_section,
                fab_section,
                alert_section,
                text_field_section,
                checkbox_section,
                radio_section,
                switch_section,
                slider_section,
                select_section,
                autocomplete_section,
                divider_section,
                tooltip_section,
                card_section,
                chip_section,
                avatar_section,
                badge_section,
                list_section,
                table_section,
                progress_section,
                skeleton_section,
                tabs_section,
                stepper_section,
                accordion_section,
                breadcrumbs_section,
                pagination_section,
                rating_section,
                timeline_section,
                layout_section,
                menu_section,
                dialog_snackbar_section,
                drawer_backdrop_section,
                speed_dial_section,
            ]
            .spacing(20)
            .padding(20),
        );

        let base = iced::widget::column![bar, content];

        let with_dialog = dialog::dialog(
            base,
            self.dialog_open,
            "Confirm action",
            iced::widget::text("Are you sure you want to proceed?").font(system_font(FontWeight::Regular)),
            vec![
                button::text_button("Cancel", Message::CloseDialog).into(),
                button::primary("Confirm", Message::CloseDialog).into(),
            ],
        );

        let with_snackbar = snackbar::snackbar(
            with_dialog,
            self.snackbar_visible,
            "Action completed successfully",
            Message::DismissSnackbar,
        );

        let with_drawer = drawer::drawer(
            with_snackbar,
            self.drawer_open,
            drawer::Anchor::Left,
            iced::widget::column![
                iced::widget::text("Navigation").size(18).font(system_font(FontWeight::Bold)),
                divider::horizontal(1),
                iced::widget::text("Dashboard").font(system_font(FontWeight::Regular)),
                iced::widget::text("Settings").font(system_font(FontWeight::Regular)),
                iced::widget::text("Logout").font(system_font(FontWeight::Regular)),
            ]
            .spacing(12),
            Message::CloseDrawer,
        );

        backdrop::backdrop(
            with_drawer,
            self.backdrop_open,
            Some(
                iced::widget::column![
                    iced::widget::text("Loading…").size(16).font(system_font(FontWeight::Regular)),
                    button::text_button("Close", Message::CloseBackdrop),
                ]
                .spacing(12)
                .align_x(iced::alignment::Horizontal::Center)
                .into(),
            ),
        )
    }
}
