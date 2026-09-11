//! RustTheme Gallery: a scrollable showcase of every component in the
//! `rustheme` crate, with a light/dark theme toggle in the app bar.

use rustheme::prelude::*;
use rustheme::icons::Icon;
use rustheme_core::{hex, system_font, system_monospace_font, FontWeight};

mod create;
use create::ButtonConfig;

fn main() -> iced::Result {
    // Configuring a custom global font is done once, here, before any UI is
    // built — see `RustTheme::builder()`'s docs for why font is process-wide
    // while the palette stays per-`RustTheme`-instance. This gallery uses
    // Roboto as the single main font throughout (body text and headings
    // alike): `use_roboto()` points the theme at that family name for both
    // roles, and `load_fonts!` actually loads the bundled font bytes into
    // iced (naming a font without loading its bytes would silently fall
    // back to a generic font, since Roboto doesn't ship with every OS).
    rustheme::fonts::use_roboto();

    rustheme::load_fonts!(iced::application(Gallery::title, Gallery::update, Gallery::view))
        .theme(Gallery::theme)
        .subscription(Gallery::subscription)
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
    PrimaryPressed,
    ThemePresetSelected(usize),
    RadiusSelected(usize),
    SpacingSelected(usize),
    TextToneSelected(usize),
    SegmentSelected(usize),
    BottomNavigationSelected(usize),
    EmptyStateAction,
    SearchChanged(String),
    SearchCleared,
    NotificationsToggled(bool),
    OpenConfirmDialog,
    CancelConfirmDialog,
    ConfirmDialogConfirmed,
    ToastDismissed(u64),
    ShowToast,
    OpenContextMenu,
    CloseContextMenu,
    ContextMenuSelected(usize),
    SidebarSelected(usize),
    SidebarToggleCollapse,
    NavSelected(usize),
    CreatePresetSelected(usize),
    CreateSwatchSelected(usize),
    CreateSwatchReset,
    SpinnerTick,
    TreeToggled(usize),
    TreeSelected(usize),
    ColorPickerSwatchSelected(usize),
    ColorPickerHexChanged(String),
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
const THEME_PRESETS: [&str; 3] = ["Teal", "Indigo", "Coral"];
const RADIUS_PRESETS: [&str; 3] = ["Compact", "Balanced", "Soft"];
const SPACING_PRESETS: [&str; 3] = ["Compact", "Normal", "Extra Space"];
const TEXT_TONES: [&str; 2] = ["Standard", "High contrast"];
const SEGMENT_LABELS: [&str; 3] = ["Day", "Week", "Month"];

const NAV_ITEMS: [sidebar_nav::SidebarNavItem<'static>; 8] = [
    sidebar_nav::SidebarNavItem { label: "Theme", icon: Icon::Settings },
    sidebar_nav::SidebarNavItem { label: "Buttons", icon: Icon::Add },
    sidebar_nav::SidebarNavItem { label: "Form Controls", icon: Icon::Edit },
    sidebar_nav::SidebarNavItem { label: "Navigation", icon: Icon::Menu },
    sidebar_nav::SidebarNavItem { label: "Feedback", icon: Icon::Notifications },
    sidebar_nav::SidebarNavItem { label: "Data Display", icon: Icon::Info },
    sidebar_nav::SidebarNavItem { label: "Layout", icon: Icon::Home },
    sidebar_nav::SidebarNavItem { label: "Advanced", icon: Icon::Terminal },
];

const CODE_SAMPLE: &str = "fn fibonacci(n: u32) -> u64 {\n    match n {\n        0 => 0,\n        1 => 1,\n        _ => fibonacci(n - 1) + fibonacci(n - 2),\n    }\n}\n\nfn main() {\n    for i in 0..10 {\n        println!(\"fib({i}) = {}\", fibonacci(i));\n    }\n}\n";

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
    theme_preset: usize,
    radius_preset: usize,
    spacing_preset: usize,
    text_tone: usize,
    segment_selected: usize,
    bottom_navigation_selected: usize,
    empty_state_action_count: u8,
    search_query: String,
    notifications_enabled: bool,
    confirm_dialog_open: bool,
    last_confirm_result: Option<&'static str>,
    toasts: Vec<toast::Toast>,
    next_toast_id: u64,
    context_menu_open: bool,
    last_context_menu_selection: Option<usize>,
    sidebar_selected: usize,
    sidebar_collapsed: bool,
    nav_selected: usize,
    create_preset: usize,
    create_swatch: Option<usize>,
    spinner_angle: f32,
    tree_expanded: std::collections::HashSet<usize>,
    tree_selected: Option<usize>,
    color_picker_selected: Option<iced::Color>,
    color_picker_hex: String,
    code_highlighter: code_viewer::Highlighter,
    tree_nodes: Vec<tree_view::Node>,
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
            theme_preset: 0,
            radius_preset: 1,
            spacing_preset: 1,
            text_tone: 0,
            segment_selected: 0,
            bottom_navigation_selected: 0,
            empty_state_action_count: 0,
            search_query: String::new(),
            notifications_enabled: true,
            confirm_dialog_open: false,
            last_confirm_result: None,
            toasts: Vec::new(),
            next_toast_id: 0,
            context_menu_open: false,
            last_context_menu_selection: None,
            sidebar_selected: 0,
            sidebar_collapsed: false,
            nav_selected: 0,
            create_preset: 0,
            create_swatch: None,
            spinner_angle: 0.0,
            tree_expanded: std::collections::HashSet::from([0, 1]),
            tree_selected: Some(4),
            color_picker_selected: Some(color_picker::DEFAULT_SWATCHES[2].color),
            color_picker_hex: "#006A6A".to_string(),
            code_highlighter: code_viewer::Highlighter::default(),
            tree_nodes: vec![
                tree_view::Node::folder(0, None, "src"),
                tree_view::Node::file(1, Some(0), "main.rs"),
                tree_view::Node::folder(2, Some(0), "components"),
                tree_view::Node::file(3, Some(2), "button.rs"),
                tree_view::Node::file(4, Some(2), "card.rs"),
                tree_view::Node::folder(5, None, "assets"),
                tree_view::Node::file(6, Some(5), "logo.svg"),
                tree_view::Node::file(7, None, "Cargo.toml"),
            ],
        }
    }
}

impl Gallery {
    fn title(&self) -> String {
        "RustTheme Gallery".to_string()
    }

    /// Builds the app-wide [`RustThemeConfig`] once, from the Theme Studio's
    /// current selections layered with the Create section's selected design
    /// style. This is the single source of truth the gallery uses for both
    /// light and dark mode — every component in the app (not just the
    /// Create section's own previews) renders from this one `RustTheme`, so
    /// picking a Create preset restyles the whole gallery.
    fn theme_config(&self) -> RustThemeConfig {
        // The Create section's selected design style is the app-wide
        // theme: when a style other than "Default" is picked, its accent
        // color, radius, and material (border/shadow finish) replace the
        // Theme Studio's own selections in both light and dark mode, so
        // every component in the gallery reflects the chosen style —
        // "Default" (index 0) leaves the Theme Studio's own picks in
        // place. Per-mode setters (`light_primary_color`/
        // `dark_primary_color`) always win over the shared setter, so the
        // override goes through the same per-mode setters the Theme
        // Studio itself uses, not the shared ones.
        //
        // Deliberately NOT overridden here: `create_config.text_color`.
        // That field is the *button label's* contrast color (chosen for
        // legibility against the button's own fill, e.g. black-on-yellow
        // for Brutalism) — feeding it into the app-wide text color made
        // ordinary body text pick a color meant for a button label, which
        // could land at or near the page background and render blank.
        // Global text color stays driven by the Theme Studio's own text
        // tone regardless of which Create preset is active.
        let (primary, radius, material) = if self.create_preset != 0 {
            let create_config = self.create_config();
            // The preset's `background` can be translucent (e.g.
            // Glassmorphism's alpha-0.22 white) — fine for the button
            // preview, which composites over the page, but `primary.main`
            // is relied on as an opaque solid everywhere else (chip fills,
            // contrast-text derivation, etc.), so alpha must be stripped
            // here or those components wash out to near-invisible.
            let opaque_primary = iced::Color { a: 1.0, ..create_config.background };
            (opaque_primary, create_config.radius.min(28.0), Some(create_config.app_material()))
        } else {
            let light_primary = hex(["#006A6A", "#3F51B5", "#B23A48"][self.theme_preset.min(2)]);
            let radius = [4.0, 12.0, 20.0][self.radius_preset.min(2)];
            (light_primary, radius, None)
        };
        let dark_primary = if self.create_preset != 0 {
            primary
        } else {
            hex(["#4FD8D5", "#B7C4FF", "#FFB1B8"][self.theme_preset.min(2)])
        };
        let light_text = hex(["#1B1B1B", "#000000"][self.text_tone.min(1)]);
        let dark_text = hex(["#F2F2F2", "#FFFFFF"][self.text_tone.min(1)]);

        let spacing = [Spacing::compact(), Spacing::normal(), Spacing::spacious()][self.spacing_preset.min(2)];

        let mut config = RustThemeConfig::new()
            .radius(radius)
            .spacing(spacing)
            .light_primary_color(primary)
            .dark_primary_color(dark_primary)
            .light_text_color(light_text)
            .dark_text_color(dark_text);

        if let Some(material) = material {
            config = config.material(material);
        }

        config
    }

    fn theme(&self) -> RustTheme {
        let config = self.theme_config();
        if self.dark_mode {
            config.dark()
        } else {
            config.light()
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
            Message::PrimaryPressed => {}
            Message::ThemePresetSelected(index) => self.theme_preset = index.min(2),
            Message::RadiusSelected(index) => self.radius_preset = index.min(2),
            Message::SpacingSelected(index) => self.spacing_preset = index.min(2),
            Message::TextToneSelected(index) => self.text_tone = index.min(1),
            Message::SegmentSelected(index) => self.segment_selected = index.min(2),
            Message::BottomNavigationSelected(index) => self.bottom_navigation_selected = index.min(2),
            Message::EmptyStateAction => {
                self.empty_state_action_count = self.empty_state_action_count.saturating_add(1)
            }
            Message::SearchChanged(value) => self.search_query = value,
            Message::SearchCleared => self.search_query.clear(),
            Message::NotificationsToggled(on) => self.notifications_enabled = on,
            Message::OpenConfirmDialog => self.confirm_dialog_open = true,
            Message::CancelConfirmDialog => {
                self.confirm_dialog_open = false;
                self.last_confirm_result = Some("Cancelled");
            }
            Message::ConfirmDialogConfirmed => {
                self.confirm_dialog_open = false;
                self.last_confirm_result = Some("Confirmed");
            }
            Message::ShowToast => {
                let id = self.next_toast_id;
                self.next_toast_id += 1;
                self.toasts.push(toast::Toast::new(id, alert::Severity::Success, "Changes saved successfully"));
            }
            Message::ToastDismissed(id) => self.toasts.retain(|t| t.id != id),
            Message::OpenContextMenu => self.context_menu_open = true,
            Message::CloseContextMenu => self.context_menu_open = false,
            Message::ContextMenuSelected(index) => {
                self.last_context_menu_selection = Some(index);
                self.context_menu_open = false;
            }
            Message::SidebarSelected(index) => self.sidebar_selected = index,
            Message::SidebarToggleCollapse => self.sidebar_collapsed = !self.sidebar_collapsed,
            Message::NavSelected(index) => self.nav_selected = index,
            Message::CreatePresetSelected(index) => self.create_preset = index,
            Message::CreateSwatchSelected(index) => self.create_swatch = Some(index),
            Message::CreateSwatchReset => self.create_swatch = None,
            Message::SpinnerTick => {
                self.spinner_angle = (self.spinner_angle + 0.12) % std::f32::consts::TAU;
            }
            Message::TreeToggled(id) => {
                if !self.tree_expanded.remove(&id) {
                    self.tree_expanded.insert(id);
                }
            }
            Message::TreeSelected(id) => self.tree_selected = Some(id),
            Message::ColorPickerSwatchSelected(index) => {
                let swatch = color_picker::DEFAULT_SWATCHES[index];
                self.color_picker_selected = Some(swatch.color);
                self.color_picker_hex = rustheme_core::to_hex(swatch.color);
            }
            Message::ColorPickerHexChanged(value) => {
                if let Some(color) = rustheme_core::parse_hex(&value) {
                    self.color_picker_selected = Some(color);
                }
                self.color_picker_hex = value;
            }
        }
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(16)).map(|_| Message::SpinnerTick)
    }

    /// The live button config for the Create section: the selected preset,
    /// with an accent swatch applied on top if the user picked one.
    fn create_config(&self) -> ButtonConfig {
        let presets = create::presets();
        let base = presets[self.create_preset.min(presets.len() - 1)].config;
        match self.create_swatch {
            Some(index) => base.with_accent(create::SWATCHES[index.min(create::SWATCHES.len() - 1)].1),
            None => base,
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
            &theme,
            "RustTheme Gallery",
            vec![button::text_button(
                &theme,
                if self.dark_mode { "Light mode" } else { "Dark mode" },
                Message::ToggleDarkMode,
            )
            .into()],
        );

        fn section<'a>(
            theme: &RustTheme,
            title: &str,
            content: Element<'a, Message, RustTheme>,
        ) -> iced::widget::Container<'a, Message, RustTheme> {
            card::card(
                theme,
                iced::widget::column![
                    // Section headings use the heading font role — Roboto
                    // Bold here, since `rustheme::fonts::use_roboto()` points
                    // both the heading and body roles at Roboto.
                    iced::widget::text(title.to_string())
                        .size(19)
                        .font(rustheme_core::heading_font(FontWeight::Bold)),
                    content,
                ]
                .spacing(12),
            )
            .width(iced::Length::Fill)
        }

        let theme_studio_section = section(
            &theme,
            "Theme Studio",
            iced::widget::column![
                iced::widget::text("Change global tokens and watch every component update. Backed by RustThemeConfig, which keeps independent overrides per light/dark mode.")
                    .font(system_font(FontWeight::Regular)),
                iced::widget::text("Primary color").size(12).font(system_font(FontWeight::Medium)),
                Element::from(segmented::segmented(&THEME_PRESETS, self.theme_preset, Message::ThemePresetSelected)),
                iced::widget::text("Global corner radius").size(12).font(system_font(FontWeight::Medium)),
                Element::from(segmented::segmented(&RADIUS_PRESETS, self.radius_preset, Message::RadiusSelected)),
                iced::widget::text("Global text color").size(12).font(system_font(FontWeight::Medium)),
                Element::from(segmented::segmented(&TEXT_TONES, self.text_tone, Message::TextToneSelected)),
                iced::widget::text("Spacing density").size(12).font(system_font(FontWeight::Medium)),
                Element::from(segmented::segmented(&SPACING_PRESETS, self.spacing_preset, Message::SpacingSelected)),
                iced::widget::row![
                    button::primary(&theme, "Primary action", Message::PrimaryPressed),
                    chip::filled(&theme, format!("{} radius", ["4px", "12px", "20px"][self.radius_preset])),
                    chip::filled(&theme, format!("{}px base spacing", [4.0, 8.0, 12.0][self.spacing_preset])),
                    card::outlined(&theme, iced::widget::text("Surface preview").font(system_font(FontWeight::Regular))),
                ]
                .spacing(10)
                .align_y(iced::alignment::Vertical::Center),
            ]
            .spacing(10)
            .into(),
        );

        let create_config = self.create_config();
        let preset_names: Vec<&str> = create::presets().iter().map(|p| p.name).collect();

        let create_section = section(
            &theme,
            "Create — Button Theme Studio",
            iced::widget::column![
                iced::widget::text("Pick a design style below and the whole preview updates to match it — no per-button theme selection. \"Default\" is RustTheme's own standard button; every other style is a full reconfiguration of border, shadow, height, and color.")
                    .font(system_font(FontWeight::Regular)),
                iced::widget::text("Style").size(12).font(system_font(FontWeight::Medium)),
                iced::widget::scrollable(
                    iced::widget::row(preset_names.iter().enumerate().map(|(index, name)| {
                        let active = index == self.create_preset;
                        iced::widget::button(
                            iced::widget::text(*name).size(12).font(system_font(FontWeight::Medium)),
                        )
                        .padding([8, 14])
                        .on_press(Message::CreatePresetSelected(index))
                        .style(move |theme: &RustTheme, status| {
                            let p = &theme.palette;
                            let background = if active {
                                Some(iced::Background::Color(p.primary.main))
                            } else if matches!(status, iced::widget::button::Status::Hovered) {
                                Some(iced::Background::Color(iced::Color { a: 0.08, ..p.primary.main }))
                            } else {
                                Some(iced::Background::Color(p.background.paper))
                            };
                            iced::widget::button::Style {
                                background,
                                text_color: if active { p.primary.contrast_text } else { p.text.primary },
                                border: iced::border::rounded(theme.shape.full),
                                shadow: iced::Shadow::default(),
                            }
                        })
                        .into()
                    }))
                    .spacing(6)
                    .width(iced::Length::Shrink)
                )
                .direction(iced::widget::scrollable::Direction::Horizontal(Default::default()))
                .width(iced::Length::Fill)
                .height(iced::Length::Fixed(50.0)),
                iced::widget::text("Accent color").size(12).font(system_font(FontWeight::Medium)),
                iced::widget::row(create::SWATCHES.iter().enumerate().map(|(index, (_name, color))| {
                    let selected = self.create_swatch == Some(index);
                    iced::widget::button(iced::widget::text(""))
                        .width(iced::Length::Fixed(28.0))
                        .height(iced::Length::Fixed(28.0))
                        .on_press(Message::CreateSwatchSelected(index))
                        .style(move |_theme: &RustTheme, _status| iced::widget::button::Style {
                            background: Some(iced::Background::Color(*color)),
                            text_color: iced::Color::TRANSPARENT,
                            border: iced::Border {
                                color: if selected { iced::Color::BLACK } else { iced::Color::TRANSPARENT },
                                width: if selected { 2.0 } else { 0.0 },
                                radius: 999.0.into(),
                            },
                            shadow: iced::Shadow::default(),
                        })
                        .into()
                }).chain(std::iter::once(
                    iced::widget::button(iced::widget::text("Reset").size(11).font(system_font(FontWeight::Medium)))
                        .padding([6, 10])
                        .on_press(Message::CreateSwatchReset)
                        .style(|theme: &RustTheme, _status| iced::widget::button::Style {
                            background: None,
                            text_color: theme.palette.text.secondary,
                            border: iced::Border { color: theme.palette.divider, width: 1.0, radius: 999.0.into() },
                            shadow: iced::Shadow::default(),
                        })
                        .into()
                )))
                .spacing(8)
                .align_y(iced::alignment::Vertical::Center),
                iced::widget::text("Preview — every button below shares the config above")
                    .size(11)
                    .font(system_font(FontWeight::Medium)),
                iced::widget::row![
                    iced::widget::button(
                        iced::widget::text("Primary action")
                            .size(14)
                            .font(system_font(FontWeight::Medium))
                            .align_x(iced::alignment::Alignment::Center)
                            .align_y(iced::alignment::Alignment::Center),
                    )
                    .padding([10, 20])
                    .height(iced::Length::Fixed(create_config.height))
                    .on_press(Message::PrimaryPressed)
                    .style(move |_theme: &RustTheme, status| create_config.style_fn()(_theme, status)),
                    iced::widget::button(
                        iced::widget::text("Secondary")
                            .size(14)
                            .font(system_font(FontWeight::Medium))
                            .align_x(iced::alignment::Alignment::Center)
                            .align_y(iced::alignment::Alignment::Center),
                    )
                    .padding([10, 20])
                    .height(iced::Length::Fixed(create_config.height))
                    .on_press(Message::PrimaryPressed)
                    .style(move |_theme: &RustTheme, status| create_config.style_fn()(_theme, status)),
                    iced::widget::button(
                        iced::widget::text("Disabled")
                            .size(14)
                            .font(system_font(FontWeight::Medium))
                            .align_x(iced::alignment::Alignment::Center)
                            .align_y(iced::alignment::Alignment::Center),
                    )
                    .padding([10, 20])
                    .height(iced::Length::Fixed(create_config.height))
                    .style(move |_theme: &RustTheme, status| create_config.style_fn()(_theme, status)),
                ]
                .spacing(12)
                .align_y(iced::alignment::Vertical::Center)
                .wrap(),
            ]
            .spacing(10)
            .into(),
        );

        let buttons_section = section(
            &theme,
            "Buttons",
            iced::widget::column![
                // Hierarchy row: ONE primary, secondaries drop in weight.
                iced::widget::column![
                    iced::widget::text("Hierarchy — one primary action")
                        .size(11)
                        .font(system_font(FontWeight::Medium)),
                    iced::widget::row![
                        button::primary(&theme, "Save", Message::PrimaryPressed),
                        button::outline(&theme, "Cancel", Message::PrimaryPressed),
                        button::ghost(&theme, "Discard", Message::PrimaryPressed),
                    ]
                    .spacing(8)
                    .wrap(),
                ]
                .spacing(4),
                // Variants across sizes.
                iced::widget::column![
                    iced::widget::text("Variants — small / default / large")
                        .size(11)
                        .font(system_font(FontWeight::Medium)),
                    iced::widget::row![
                        button::primary_with(
                            &theme,
                            "Primary",
                            Message::PrimaryPressed,
                            button::Size::Small,
                            false,
                        ),
                        button::primary_with(
                            &theme,
                            "Primary",
                            Message::PrimaryPressed,
                            button::Size::Default,
                            false,
                        ),
                        button::primary_with(
                            &theme,
                            "Primary",
                            Message::PrimaryPressed,
                            button::Size::Large,
                            false,
                        ),
                    ]
                    .spacing(8)
                    .wrap(),
                    iced::widget::row![
                        button::secondary_with(&theme, "Secondary", Message::PrimaryPressed, button::Size::Small, false),
                        button::secondary_with(&theme, "Secondary", Message::PrimaryPressed, button::Size::Default, false),
                        button::secondary_with(&theme, "Secondary", Message::PrimaryPressed, button::Size::Large, false),
                    ]
                    .spacing(8)
                    .wrap(),
                    iced::widget::row![
                        button::outline_with(&theme, "Outline", Message::PrimaryPressed, button::Size::Small, false),
                        button::outline_with(&theme, "Outline", Message::PrimaryPressed, button::Size::Default, false),
                        button::outline_with(&theme, "Outline", Message::PrimaryPressed, button::Size::Large, false),
                    ]
                    .spacing(8)
                    .wrap(),
                    iced::widget::row![
                        button::ghost_with(&theme, "Ghost", Message::PrimaryPressed, button::Size::Small, false),
                        button::ghost_with(&theme, "Ghost", Message::PrimaryPressed, button::Size::Default, false),
                        button::ghost_with(&theme, "Ghost", Message::PrimaryPressed, button::Size::Large, false),
                    ]
                    .spacing(8)
                    .wrap(),
                    iced::widget::row![
                        button::destructive_with(&theme, "Delete", Message::PrimaryPressed, button::Size::Small, false),
                        button::destructive_with(&theme, "Delete", Message::PrimaryPressed, button::Size::Default, false),
                        button::destructive_with(&theme, "Delete", Message::PrimaryPressed, button::Size::Large, false),
                    ]
                    .spacing(8)
                    .wrap(),
                ]
                .spacing(4),
                // Static loading examples do not mutate the entire gallery.
                iced::widget::column![
                    iced::widget::text("States — loading")
                        .size(11)
                        .font(system_font(FontWeight::Medium)),
                    iced::widget::row![
                        button::primary_with(
                            &theme,
                            "Loading",
                            Message::PrimaryPressed,
                            button::Size::Default,
                            true,
                        ),
                        button::secondary_with(
                            &theme,
                            "Loading",
                            Message::PrimaryPressed,
                            button::Size::Default,
                            true,
                        ),
                        button::outline_with(
                            &theme,
                            "Loading",
                            Message::PrimaryPressed,
                            button::Size::Default,
                            true,
                        ),
                    ]
                    .spacing(8)
                    .wrap(),
                ]
                .spacing(4),
            ]
            .spacing(14)
            .into(),
        );

        let text_field_section = section(
            &theme,
            "Text Fields",
            iced::widget::row![
                text_field::outlined(&theme, "Outlined", &self.text_value, Message::TextChanged),
                text_field::filled(&theme, "Filled", &self.text_value, Message::TextChanged),
            ]
            .spacing(12)
            .into(),
        );

        let checkbox_section = section(
            &theme,
            "Checkbox",
            checkbox::checkbox("Accept terms", self.checkbox_checked, Message::CheckboxToggled).into(),
        );

        let radio_section = section(
            &theme,
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
            &theme,
            "Switch",
            switch::switch(self.switch_on, Message::SwitchToggled).label("Enable notifications").into(),
        );

        let slider_section = section(
            &theme,
            "Slider",
            iced::widget::column![
                slider::slider(0.0..=100.0, self.slider_value, Message::SliderChanged),
                iced::widget::text(format!("{:.0}", self.slider_value)).font(system_monospace_font(FontWeight::Regular)),
            ]
            .spacing(6)
            .into(),
        );

        let select_section = section(
            &theme,
            "Select",
            select::select(&theme, &FRAMEWORKS[..], self.select_value, Message::SelectPicked).into(),
        );

        let divider_section = section(
            &theme,
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
            &theme,
            "Tooltip",
            tooltip::tooltip(&theme, button::outlined(&theme, "Hover me", Message::ToggleDarkMode), "This is a tooltip").into(),
        );

        let card_section = section(
            &theme,
            "Cards",
            iced::widget::row![
                card::card(&theme, iced::widget::text("Card").font(system_font(FontWeight::Regular))).width(iced::Length::Fill),
                card::elevated(&theme, iced::widget::text("Elevated").font(system_font(FontWeight::Regular))).width(iced::Length::Fill),
                card::outlined(&theme, iced::widget::text("Outlined").font(system_font(FontWeight::Regular))).width(iced::Length::Fill),
            ]
            .spacing(12)
            .into(),
        );

        let chip_section = section(
            &theme,
            "Chips",
            iced::widget::row![chip::filled(&theme, "Filled"), chip::outlined(&theme, "Outlined")]
                .spacing(8)
                .into(),
        );

        let avatar_section = section(&theme, "Avatar", avatar::initials("RT", 48.0).into());

        let badge_section = section(
            &theme,
            "Badge",
            iced::widget::column![
                iced::widget::row![
                    badge::count(
                        &theme,
                        icon_button::outlined(icons::glyph(Icon::Mail), Message::ToggleDarkMode),
                        7,
                        99,
                        badge::Color::Error,
                        badge::Position::TopRight,
                    ),
                    badge::count(
                        &theme,
                        icon_button::outlined(icons::glyph(Icon::Mail), Message::ToggleDarkMode),
                        128,
                        99,
                        badge::Color::Primary,
                        badge::Position::TopLeft,
                    ),
                    badge::dot(
                        icon_button::outlined(icons::glyph(Icon::Favorite), Message::ToggleDarkMode),
                        badge::Color::Success,
                        badge::Position::BottomRight,
                    ),
                    badge::dot(
                        icon_button::outlined(icons::glyph(Icon::Notifications), Message::ToggleDarkMode),
                        badge::Color::Warning,
                        badge::Position::BottomLeft,
                    ),
                ]
                .spacing(24)
                .align_y(iced::alignment::Vertical::Center),
                iced::widget::row![
                    badge::standard(&theme, "New", badge::Color::Info),
                    badge::standard(&theme, "Beta", badge::Color::Secondary),
                ]
                .spacing(8),
            ]
            .spacing(16)
            .into(),
        );

        let list_section = section(
            &theme,
            "List",
            list::list(vec![
                list::item(&theme, iced::widget::text("Item one").font(system_font(FontWeight::Regular))).into(),
                list::item(&theme, iced::widget::text("Item two").font(system_font(FontWeight::Regular))).into(),
                list::item(&theme, iced::widget::text("Item three").font(system_font(FontWeight::Regular))).into(),
            ])
            .into(),
        );

        let progress_section = section(
            &theme,
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
            &theme,
            "Tabs",
            iced::widget::column![
                tabs::tabs(&theme, &TAB_LABELS, self.active_tab, Message::TabSelected),
                iced::widget::text(format!("Active tab: {}", TAB_LABELS[self.active_tab])).font(system_font(FontWeight::Regular)),
            ]
            .spacing(8)
            .into(),
        );

        let segmented_section = section(
            &theme,
            "Segmented Control",
            iced::widget::column![
                segmented::segmented(&SEGMENT_LABELS, self.segment_selected, Message::SegmentSelected),
                iced::widget::text(format!("Selected: {}", SEGMENT_LABELS[self.segment_selected]))
                    .font(system_font(FontWeight::Regular)),
            ]
            .spacing(10)
            .into(),
        );

        let bottom_navigation_section = section(
            &theme,
            "Bottom Navigation",
            bottom_navigation::bottom_navigation(
                &[
                    bottom_navigation::BottomNavigationItem { label: "Home", icon: Icon::Home },
                    bottom_navigation::BottomNavigationItem { label: "Search", icon: Icon::Search },
                    bottom_navigation::BottomNavigationItem { label: "Settings", icon: Icon::Settings },
                ],
                self.bottom_navigation_selected,
                Message::BottomNavigationSelected,
            )
            .into(),
        );

        let empty_state_section = section(
            &theme,
            "Empty State",
            empty_state::empty_state(
                &theme,
                Icon::Mail,
                "No messages yet",
                if self.empty_state_action_count == 0 {
                    "Create a message to start a conversation."
                } else {
                    "Message composer opened from this empty state."
                },
                Some(("Create message", Message::EmptyStateAction)),
            )
            .into(),
        );

        let menu_status = match self.last_menu_selection {
            Some(index) => format!("Last selected: {}", MENU_ITEMS[index]),
            None => "Nothing selected yet".to_string(),
        };
        let menu_section = section(
            &theme,
            "Menu",
            iced::widget::column![
                menu::menu(
                    &theme,
                    button::outlined(&theme, "Open menu", Message::OpenMenu),
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
            &theme,
            "Dialog & Snackbar",
            iced::widget::row![
                button::primary(&theme, "Open dialog", Message::OpenDialog),
                button::secondary(&theme, "Show snackbar", Message::ShowSnackbar),
            ]
            .spacing(8)
            .into(),
        );

        let icon_button_section = section(
            &theme,
            "Icon Button",
            iced::widget::row![
                icon_button::icon_primary(icons::glyph(Icon::Star), Message::ToggleDarkMode),
                icon_button::primary(icons::glyph(Icon::Favorite), Message::ToggleDarkMode),
                icon_button::outlined(icons::glyph(Icon::Edit), Message::ToggleDarkMode),
                icon_button::text_icon(icons::glyph(Icon::Close), Message::ToggleDarkMode),
            ]
            .spacing(8)
            .into(),
        );

        let fab_section = section(
            &theme,
            "Fab",
            iced::widget::row![
                fab::primary(&theme, icons::glyph(Icon::Add), Message::ToggleDarkMode),
                fab::secondary(&theme, icons::glyph(Icon::Edit), Message::ToggleDarkMode),
                fab::extended(&theme, icons::glyph(Icon::Add), "Create", Message::ToggleDarkMode),
            ]
            .spacing(16)
            .align_y(iced::alignment::Vertical::Center)
            .into(),
        );

        let alert_section = section(
            &theme,
            "Alert",
            if self.alert_dismissed {
                iced::widget::column![iced::widget::text("Alert dismissed.").font(system_font(FontWeight::Regular))].into()
            } else {
                iced::widget::column![
                    alert::success(&theme, "Your changes have been saved.", None::<Message>),
                    alert::info(&theme, "A new version is available.", None::<Message>),
                    alert::warning(&theme, "Your session will expire soon.", None::<Message>),
                    alert::with_title(
                        &theme,
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
            &theme,
            "Accordion",
            accordion::accordion(
                &theme,
                "Click to expand",
                iced::widget::text("Hidden content revealed when expanded.").font(system_font(FontWeight::Regular)),
                self.accordion_expanded,
                Message::AccordionToggled,
            )
            .into(),
        );

        let breadcrumbs_section = section(
            &theme,
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
            &theme,
            "Pagination",
            pagination::pagination(self.current_page, 10, Message::PageSelected).into(),
        );

        let skeleton_section = section(
            &theme,
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
            &theme,
            "Stepper",
            stepper::stepper(&theme, &["Account", "Shipping", "Payment", "Review"], self.active_tab.min(3)).into(),
        );

        let rating_section = section(
            &theme,
            "Rating",
            iced::widget::row![
                rating::rating(&theme, self.rating_value, 5, Message::RatingChanged),
                iced::widget::text(format!("{}/5", self.rating_value)).font(system_monospace_font(FontWeight::Regular)),
            ]
            .spacing(12)
            .align_y(iced::alignment::Vertical::Center)
            .into(),
        );

        let table_section = section(
            &theme,
            "Table",
            table::table(
                &theme,
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
            &theme,
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
                        card::outlined(&theme, iced::widget::text("1").font(system_font(FontWeight::Regular))).into(),
                        card::outlined(&theme, iced::widget::text("2").font(system_font(FontWeight::Regular))).into(),
                        card::outlined(&theme, iced::widget::text("3").font(system_font(FontWeight::Regular))).into(),
                        card::outlined(&theme, iced::widget::text("4").font(system_font(FontWeight::Regular))).into(),
                    ],
                )),
            ]
            .spacing(12)
            .into(),
        );

        let autocomplete_section = section(
            &theme,
            "Autocomplete",
            iced::widget::column![
                autocomplete::autocomplete(
                    &theme,
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
            &theme,
            "Timeline",
            timeline::timeline(&theme, vec![
                timeline::TimelineItem::new("Order placed", "Your order has been received."),
                timeline::TimelineItem::new("Shipped", "Your order is on its way."),
                timeline::TimelineItem::new("Delivered", "Your order has arrived."),
            ])
            .into(),
        );

        let drawer_backdrop_section = section(
            &theme,
            "Drawer & Backdrop",
            iced::widget::row![
                button::primary(&theme, "Open drawer", Message::OpenDrawer),
                button::secondary(&theme, "Open backdrop", Message::OpenBackdrop),
            ]
            .spacing(8)
            .into(),
        );

        let speed_dial_section = section(
            &theme,
            "Speed Dial",
            speed_dial::speed_dial(
                &theme,
                icons::glyph(Icon::Add),
                self.speed_dial_open,
                vec![
                    speed_dial::SpeedDialAction::new(icons::glyph(Icon::Edit), Message::SpeedDialAction("Edit")),
                    speed_dial::SpeedDialAction::new(icons::glyph(Icon::Favorite), Message::SpeedDialAction("Favorite")),
                    speed_dial::SpeedDialAction::new(icons::glyph(Icon::Delete), Message::SpeedDialAction("Delete")),
                ],
                Message::SpeedDialToggled,
            )
            .into(),
        );

        let search_bar_section = section(
            &theme,
            "Search Bar",
            search_bar::search_bar(
                "Search components…",
                &self.search_query,
                Message::SearchChanged,
                Some(Message::SearchCleared),
            )
            .into(),
        );

        let stat_card_section = section(
            &theme,
            "Stat Card",
            iced::widget::row![
                stat_card::stat_card("Active Users", "12,483"),
                stat_card::with_trend("Revenue", "$48.2k", "+12.4% vs last month", stat_card::Trend::Up),
                stat_card::with_trend("Churn", "2.1%", "-0.4% vs last month", stat_card::Trend::Down),
            ]
            .spacing(12)
            .into(),
        );

        let settings_row_section = section(
            &theme,
            "Settings Row",
            card::card(
                &theme,
                list::list(vec![
                    settings_row::settings_row(
                        Icon::Notifications,
                        "Notifications",
                        Some("Get notified about updates"),
                        switch::switch(self.notifications_enabled, Message::NotificationsToggled),
                    )
                    .into(),
                    settings_row::settings_row(
                        Icon::Lock,
                        "Privacy",
                        Some("Manage your data and permissions"),
                        icons::icon(Icon::ChevronRight, 18.0),
                    )
                    .into(),
                ]),
            )
            .into(),
        );

        let toolbar_section = section(
            &theme,
            "Toolbar",
            card::outlined(
                &theme,
                toolbar::toolbar(
                    iced::widget::text("12 items selected").font(system_font(FontWeight::Medium)),
                    vec![
                        button::outline(&theme, "Archive", Message::PrimaryPressed).into(),
                        button::destructive(&theme, "Delete", Message::PrimaryPressed).into(),
                    ],
                ),
            )
            .into(),
        );

        let confirm_dialog_status = match self.last_confirm_result {
            Some(result) => format!("Last result: {result}"),
            None => "Nothing confirmed yet".to_string(),
        };
        let confirm_dialog_section = section(
            &theme,
            "Confirm Dialog",
            iced::widget::column![
                button::destructive(&theme, "Delete account", Message::OpenConfirmDialog),
                iced::widget::text(confirm_dialog_status).font(system_font(FontWeight::Regular)),
            ]
            .spacing(8)
            .into(),
        );

        let toast_section = section(
            &theme,
            "Toast",
            iced::widget::column![
                button::primary(&theme, "Show toast", Message::ShowToast),
                iced::widget::text(format!("{} toast(s) active", self.toasts.len())).font(system_font(FontWeight::Regular)),
            ]
            .spacing(8)
            .into(),
        );

        let context_menu_status = match self.last_context_menu_selection {
            Some(index) => format!("Last selected: {}", MENU_ITEMS[index]),
            None => "Right-click the box below".to_string(),
        };
        let context_menu_section = section(
            &theme,
            "Context Menu",
            iced::widget::column![
                context_menu::context_menu(
                    card::outlined(
                        &theme,
                        iced::widget::text("Right-click me").font(system_font(FontWeight::Regular)),
                    )
                    .width(iced::Length::Fixed(220.0)),
                    self.context_menu_open,
                    &MENU_ITEMS,
                    Message::OpenContextMenu,
                    Message::ContextMenuSelected,
                    Message::CloseContextMenu,
                ),
                iced::widget::text(context_menu_status).font(system_font(FontWeight::Regular)),
            ]
            .spacing(8)
            .into(),
        );

        let sidebar_nav_section = section(
            &theme,
            "Sidebar Nav",
            iced::widget::container(sidebar_nav::sidebar_nav(
                &[
                    sidebar_nav::SidebarNavItem { label: "Dashboard", icon: Icon::Home },
                    sidebar_nav::SidebarNavItem { label: "Search", icon: Icon::Search },
                    sidebar_nav::SidebarNavItem { label: "Settings", icon: Icon::Settings },
                ],
                self.sidebar_selected,
                self.sidebar_collapsed,
                Message::SidebarSelected,
                Message::SidebarToggleCollapse,
            ))
            .height(iced::Length::Fixed(220.0))
            .into(),
        );

        let key_value_list_section = section(
            &theme,
            "Key Value List",
            key_value_list::key_value_list(vec![
                key_value_list::KeyValue::new("Order ID", "#38291"),
                key_value_list::KeyValue::new("Status", "Shipped"),
                key_value_list::KeyValue::new("Total", "$128.50"),
            ])
            .into(),
        );

        let data_card_grid_section = section(
            &theme,
            "Data Card Grid",
            data_card_grid::data_card_grid(
                &theme,
                3,
                vec![
                    card::outlined(&theme, iced::widget::text("Card A").font(system_font(FontWeight::Regular))).into(),
                    card::outlined(&theme, iced::widget::text("Card B").font(system_font(FontWeight::Regular))).into(),
                    card::outlined(&theme, iced::widget::text("Card C").font(system_font(FontWeight::Regular))).into(),
                ],
                false,
                1,
                Icon::Mail,
                "No items",
                "Nothing to show yet.",
            ),
        );

        let tree_view_section = section(
            &theme,
            "Tree View",
            iced::widget::container(tree_view::tree_view(
                &self.tree_nodes,
                &self.tree_expanded,
                self.tree_selected,
                Message::TreeToggled,
                Message::TreeSelected,
            ))
            .width(iced::Length::Fixed(260.0))
            .into(),
        );

        let code_viewer_section = section(
            &theme,
            "Code Viewer",
            code_viewer::code_viewer(&self.code_highlighter, CODE_SAMPLE, "rust"),
        );

        let color_picker_section = section(
            &theme,
            "Color Picker",
            color_picker::color_picker(
                &color_picker::DEFAULT_SWATCHES,
                self.color_picker_selected,
                &self.color_picker_hex,
                Message::ColorPickerSwatchSelected,
                Message::ColorPickerHexChanged,
            ),
        );

        let spinner_section = section(
            &theme,
            "Spinner",
            iced::widget::row![
                spinner::spinner(&theme, self.spinner_angle, 28.0),
                spinner::spinner(&theme, self.spinner_angle * 1.6, 40.0),
                spinner::spinner_colored(self.spinner_angle * 0.7, 20.0, theme.palette.error.main),
            ]
            .spacing(20)
            .align_y(iced::alignment::Vertical::Center)
            .into(),
        );

        let comparison_table_section = section(
            &theme,
            "Comparison Table",
            comparison_table::comparison_table(
                &[
                    comparison_table::Plan::new("Free", "$0/mo", false),
                    comparison_table::Plan::new("Pro", "$12/mo", true),
                    comparison_table::Plan::new("Team", "$29/mo", false),
                ],
                &[
                    comparison_table::FeatureRow::new(
                        "Projects",
                        vec![
                            comparison_table::Cell::Text("3".into()),
                            comparison_table::Cell::Text("Unlimited".into()),
                            comparison_table::Cell::Text("Unlimited".into()),
                        ],
                    ),
                    comparison_table::FeatureRow::new(
                        "Priority support",
                        vec![comparison_table::Cell::Excluded, comparison_table::Cell::Included, comparison_table::Cell::Included],
                    ),
                    comparison_table::FeatureRow::new(
                        "Team seats",
                        vec![comparison_table::Cell::Excluded, comparison_table::Cell::Excluded, comparison_table::Cell::Included],
                    ),
                ],
            )
            .into(),
        );

        let category_sections: Vec<Element<'_, Message, RustTheme>> = match self.nav_selected {
            0 => vec![theme_studio_section.into(), create_section.into()],
            1 => vec![buttons_section.into(), icon_button_section.into(), fab_section.into()],
            2 => vec![
                text_field_section.into(),
                checkbox_section.into(),
                radio_section.into(),
                switch_section.into(),
                slider_section.into(),
                select_section.into(),
                autocomplete_section.into(),
            ],
            3 => vec![
                tabs_section.into(),
                segmented_section.into(),
                bottom_navigation_section.into(),
                breadcrumbs_section.into(),
                pagination_section.into(),
                stepper_section.into(),
                menu_section.into(),
                context_menu_section.into(),
                sidebar_nav_section.into(),
                search_bar_section.into(),
                accordion_section.into(),
            ],
            4 => vec![
                alert_section.into(),
                dialog_snackbar_section.into(),
                drawer_backdrop_section.into(),
                confirm_dialog_section.into(),
                toast_section.into(),
                empty_state_section.into(),
                skeleton_section.into(),
                progress_section.into(),
            ],
            5 => vec![
                card_section.into(),
                chip_section.into(),
                avatar_section.into(),
                badge_section.into(),
                list_section.into(),
                table_section.into(),
                rating_section.into(),
                timeline_section.into(),
                stat_card_section.into(),
                settings_row_section.into(),
                key_value_list_section.into(),
                data_card_grid_section.into(),
                comparison_table_section.into(),
            ],
            6 => vec![
                divider_section.into(),
                tooltip_section.into(),
                layout_section.into(),
                speed_dial_section.into(),
                toolbar_section.into(),
            ],
            _ => vec![
                tree_view_section.into(),
                code_viewer_section.into(),
                color_picker_section.into(),
                spinner_section.into(),
            ],
        };

        let content = iced::widget::scrollable(
            iced::widget::column(category_sections).spacing(20).padding(20),
        );

        let nav = sidebar_nav::sidebar_nav(
            &NAV_ITEMS,
            self.nav_selected,
            self.sidebar_collapsed,
            Message::NavSelected,
            Message::SidebarToggleCollapse,
        );

        let body = iced::widget::row![nav, content].height(iced::Length::Fill);

        let base = iced::widget::column![bar, body];

        let with_dialog = dialog::dialog(
            &theme,
            base,
            self.dialog_open,
            "Confirm action",
            iced::widget::text("Are you sure you want to proceed?").font(system_font(FontWeight::Regular)),
            vec![
                button::text_button(&theme, "Cancel", Message::CloseDialog).into(),
                button::primary(&theme, "Confirm", Message::CloseDialog).into(),
            ],
        );

        let with_snackbar = snackbar::snackbar(
            &theme,
            with_dialog,
            self.snackbar_visible,
            "Action completed successfully",
            Message::DismissSnackbar,
        );

        let with_drawer = drawer::drawer(
            &theme,
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

        let with_backdrop = backdrop::backdrop(
            with_drawer,
            self.backdrop_open,
            Some(
                iced::widget::column![
                    iced::widget::text("Loading…").size(16).font(system_font(FontWeight::Regular)),
                    button::text_button(&theme, "Close", Message::CloseBackdrop),
                ]
                .spacing(12)
                .align_x(iced::alignment::Horizontal::Center)
                .into(),
            ),
        );

        let with_confirm_dialog = confirm_dialog::destructive(
            &theme,
            with_backdrop,
            self.confirm_dialog_open,
            "Delete account",
            "This action is permanent and cannot be undone. Are you sure?",
            "Delete",
            Message::CancelConfirmDialog,
            Message::ConfirmDialogConfirmed,
        );

        toast::toast_stack(
            with_confirm_dialog,
            &self.toasts,
            toast::Position::BottomRight,
            Message::ToastDismissed,
        )
    }
}
