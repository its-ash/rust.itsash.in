//! `Catalog` implementations wiring [`RustTheme`](crate::theme::RustTheme)
//! into every iced widget this crate styles.
//!
//! Each widget module in iced 0.13 defines its own `Catalog` trait with an
//! associated `Class<'a>` (a boxed style-producing closure) and a `style`
//! method. Implementing these traits for `RustTheme` is what lets widgets be
//! constructed as `Element<'a, Message, RustTheme, Renderer>` and styled via
//! `.class(rustheme::style::button::primary())`-style class constructors.
//!
//! Named style classes per widget (e.g. `button::primary`, `button::danger`)
//! live in the submodules below and are re-exported from `rustheme::style`.

pub mod button;
pub mod checkbox;
pub mod container;
pub mod menu;
pub mod pick_list;
pub mod progress_bar;
pub mod radio;
pub mod rule;
pub mod scrollable;
pub mod slider;
pub mod text;
pub mod text_input;
pub mod toggler;

use crate::theme::RustTheme;

impl iced::application::DefaultStyle for RustTheme {
    fn default_style(&self) -> iced::application::Appearance {
        iced::application::Appearance {
            background_color: self.palette.background.default,
            text_color: self.palette.text.primary,
        }
    }
}

impl iced::widget::button::Catalog for RustTheme {
    type Class<'a> = iced::widget::button::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        button::primary()
    }

    fn style(&self, class: &Self::Class<'_>, status: iced::widget::button::Status) -> iced::widget::button::Style {
        class(self, status)
    }
}

impl iced::widget::text_input::Catalog for RustTheme {
    type Class<'a> = iced::widget::text_input::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        text_input::outlined()
    }

    fn style(
        &self,
        class: &Self::Class<'_>,
        status: iced::widget::text_input::Status,
    ) -> iced::widget::text_input::Style {
        class(self, status)
    }
}

impl iced::widget::checkbox::Catalog for RustTheme {
    type Class<'a> = iced::widget::checkbox::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        checkbox::primary()
    }

    fn style(
        &self,
        class: &Self::Class<'_>,
        status: iced::widget::checkbox::Status,
    ) -> iced::widget::checkbox::Style {
        class(self, status)
    }
}

impl iced::widget::radio::Catalog for RustTheme {
    type Class<'a> = iced::widget::radio::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        radio::primary()
    }

    fn style(&self, class: &Self::Class<'_>, status: iced::widget::radio::Status) -> iced::widget::radio::Style {
        class(self, status)
    }
}

impl iced::widget::slider::Catalog for RustTheme {
    type Class<'a> = iced::widget::slider::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        slider::primary()
    }

    fn style(&self, class: &Self::Class<'_>, status: iced::widget::slider::Status) -> iced::widget::slider::Style {
        class(self, status)
    }
}

impl iced::widget::toggler::Catalog for RustTheme {
    type Class<'a> = iced::widget::toggler::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        toggler::primary()
    }

    fn style(&self, class: &Self::Class<'_>, status: iced::widget::toggler::Status) -> iced::widget::toggler::Style {
        class(self, status)
    }
}

impl iced::widget::progress_bar::Catalog for RustTheme {
    type Class<'a> = iced::widget::progress_bar::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        progress_bar::primary()
    }

    fn style(&self, class: &Self::Class<'_>) -> iced::widget::progress_bar::Style {
        class(self)
    }
}

impl iced::widget::text::Catalog for RustTheme {
    type Class<'a> = iced::widget::text::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        text::primary()
    }

    fn style(&self, class: &Self::Class<'_>) -> iced::widget::text::Style {
        class(self)
    }
}

impl iced::widget::container::Catalog for RustTheme {
    type Class<'a> = iced::widget::container::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        container::transparent()
    }

    fn style(&self, class: &Self::Class<'_>) -> iced::widget::container::Style {
        class(self)
    }
}

impl iced::widget::scrollable::Catalog for RustTheme {
    type Class<'a> = iced::widget::scrollable::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        scrollable::default()
    }

    fn style(
        &self,
        class: &Self::Class<'_>,
        status: iced::widget::scrollable::Status,
    ) -> iced::widget::scrollable::Style {
        class(self, status)
    }
}

impl iced::widget::overlay::menu::Catalog for RustTheme {
    type Class<'a> = iced::widget::overlay::menu::StyleFn<'a, Self>;

    fn default<'a>() -> <Self as iced::widget::overlay::menu::Catalog>::Class<'a> {
        menu::default()
    }

    fn style(
        &self,
        class: &<Self as iced::widget::overlay::menu::Catalog>::Class<'_>,
    ) -> iced::widget::overlay::menu::Style {
        class(self)
    }
}

impl iced::widget::rule::Catalog for RustTheme {
    type Class<'a> = iced::widget::rule::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        rule::default()
    }

    fn style(&self, class: &Self::Class<'_>) -> iced::widget::rule::Style {
        class(self)
    }
}

impl iced::widget::pick_list::Catalog for RustTheme {
    type Class<'a> = iced::widget::pick_list::StyleFn<'a, Self>;

    fn default<'a>() -> <Self as iced::widget::pick_list::Catalog>::Class<'a> {
        pick_list::outlined()
    }

    fn style(
        &self,
        class: &<Self as iced::widget::pick_list::Catalog>::Class<'_>,
        status: iced::widget::pick_list::Status,
    ) -> iced::widget::pick_list::Style {
        class(self, status)
    }
}
