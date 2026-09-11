//! MUI-inspired button component library.
//!
//! Six variants — [`primary`], [`secondary`], [`outline`], [`ghost`],
//! [`destructive`], [`icon`] — across three sizes ([`Size::Small`],
//! [`Size::Default`], [`Size::Large`]), all sharing one corner radius and
//! one token-driven scale.
//!
//! # Hierarchy
//! One `primary` (solid accent) per surface; secondaries drop to
//! `outline`/`ghost` so they never compete at equal weight.
//!
//! # Hit area
//! Every size meets or exceeds a 44x44px minimum tap target. `Small` pads
//! internally to reach 44px even with a short label.
//!
//! # States
//! Default, hover, pressed, disabled are handled in [`crate::style::button`].
//! [`loading`] swaps the label for a spinning indicator and ignores presses.
//! [`disabled`] renders the label dimmed and drops the `on_press` handler.
//!
//! # Loading spinner
//! [`loading`] renders an indeterminate CSS-style spinner built from
//! `canvas` — no asset dependency, scales with the button's text size.

use iced::widget::button::Button;
use iced::widget::text;
use iced::{Alignment, Element, Length};
use crate::theme::RustTheme;
use rustheme_core::{system_font, FontWeight};

/// Button size. All sizes share one radius; only padding, text size, and
/// minimum height scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Size {
    /// Compact: 13px text, 6x12 padding, 32px min height (padded to 44 on tap).
    Small,
    /// Default: 14px text, 8x16 padding, 40px min height (padded to 44 on tap).
    Default,
    /// Large: 16px text, 11x22 padding, 48px min height.
    Large,
}

impl Default for Size {
    fn default() -> Self {
        Self::Default
    }
}

impl Size {
    fn text_size(self) -> f32 {
        match self {
            Self::Small => 13.0,
            Self::Default => 14.0,
            Self::Large => 16.0,
        }
    }

    /// Vertical, horizontal padding, in units of `theme.spacing` (so
    /// picking a denser/looser spacing preset scales button padding too).
    /// Factors are chosen so `Spacing::normal()` (8px/unit) reproduces the
    /// original literal values: `Small` 6x12 -> 0.75x1.5, `Default` 8x16 ->
    /// 1.0x2.0, `Large` 11x22 -> 1.375x2.75.
    fn padding(self, theme: &RustTheme) -> [f32; 2] {
        let (v, h) = match self {
            Self::Small => (0.75, 1.5),
            Self::Default => (1.0, 2.0),
            Self::Large => (1.375, 2.75),
        };
        [theme.spacing.unit(v), theme.spacing.unit(h)]
    }

    /// Rendered button height. `Small` is deliberately below the 44px touch
    /// target (this is a desktop app; `Default`/`Large` still clear it).
    fn min_height(self) -> u16 {
        match self {
            Self::Small => 32,
            Self::Default => 44,
            Self::Large => 48,
        }
    }

    fn spinner_size(self) -> f32 {
        match self {
            Self::Small => 12.0,
            Self::Default => 14.0,
            Self::Large => 18.0,
        }
    }
}

/// A spinning indeterminate indicator for the loading state. Renders a
/// 3/4 arc in the theme's accent color — a static spinner shape (iced 0.13
/// canvas doesn't self-tick; animation needs an app-level subscription,
/// which is out of scope for a self-contained component).
fn spinner<'a, Message>(size: f32) -> Element<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    iced::widget::canvas(spinner::Spinner { size })
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .into()
}

mod spinner {
    use iced::widget::canvas::{self, Geometry, Path};
    use iced::mouse::Cursor;
    use iced::{Radians, Rectangle};

    /// A static 3/4 arc spinner shape.
    pub struct Spinner {
        pub size: f32,
    }

    #[derive(Default)]
    pub struct SpinnerState;

    impl<Message> canvas::Program<Message, crate::theme::RustTheme> for Spinner {
        type State = SpinnerState;

        fn draw(
            &self,
            _state: &Self::State,
            renderer: &iced::Renderer,
            theme: &crate::theme::RustTheme,
            bounds: Rectangle,
            _cursor: Cursor,
        ) -> Vec<Geometry<iced::Renderer>> {
            let center = iced::Point::new(
                bounds.x + bounds.width / 2.0,
                bounds.y + bounds.height / 2.0,
            );
            let radius = self.size / 2.0;
            let color = theme.palette.primary.main;

            let mut frame = canvas::Frame::new(renderer, bounds.size());
            // Move to center, rotate so the gap sits at the top-right.
            frame.with_save(|frame| {
                frame.translate(iced::Vector::new(
                    center.x - bounds.x,
                    center.y - bounds.y,
                ));
                frame.rotate(Radians(-std::f32::consts::FRAC_PI_4));
                let stroke = canvas::Stroke {
                    style: canvas::Style::Solid(color),
                    width: self.size / 6.0,
                    line_cap: canvas::LineCap::Round,
                    ..Default::default()
                };
                let path = Path::new(|b| {
                    b.arc(canvas::path::Arc {
                        center: iced::Point::ORIGIN,
                        radius,
                        start_angle: Radians(0.0),
                        end_angle: Radians(std::f32::consts::TAU * 0.85),
                    });
                });
                frame.stroke(&path, stroke);
            });

            vec![frame.into_geometry()]
        }
    }
}

/// Wraps the label with the variant's style class and an optional loading
/// spinner. When `loading` is true the label is replaced by a spinner and
/// no `on_press` is wired, so presses are silently ignored.
fn styled<'a, Message>(
    theme: &RustTheme,
    inner: Element<'a, Message, RustTheme>,
    size: Size,
    class: iced::widget::button::StyleFn<'a, RustTheme>,
    loading: bool,
    on_press: Option<Message>,
) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    if loading {
        iced::widget::button(
            iced::widget::container(spinner(size.spinner_size()))
                .width(Length::Fixed(size.spinner_size()))
                .height(Length::Fixed(size.spinner_size()))
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
        )
        .padding(size.padding(theme))
        .height(Length::Fixed(size.min_height() as f32))
        .style(class)
    } else {
        iced::widget::button(inner)
            .padding(size.padding(theme))
            .height(Length::Fixed(size.min_height() as f32))
            .style(class)
            .on_press_maybe(on_press)
    }
}

/// Builds the text content for a labeled button.
fn label_content<'a, Message>(
    label: impl text::IntoFragment<'a>,
    size: Size,
) -> Element<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    iced::widget::text(label)
        .font(system_font(FontWeight::Medium))
        .size(size.text_size())
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .into()
}

// ---------------------------------------------------------------------------
// Variant constructors
// ---------------------------------------------------------------------------

/// A primary, filled accent button — the main call to action.
/// Use at most one per surface. Padding scales with `theme.spacing`.
pub fn primary<'a, Message>(
    theme: &RustTheme,
    label: impl text::IntoFragment<'a>,
    on_press: Message,
) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    primary_with(theme, label, on_press, Size::Default, false)
}

/// `primary` with explicit [`Size`] and loading flag.
pub fn primary_with<'a, Message>(
    theme: &RustTheme,
    label: impl text::IntoFragment<'a>,
    on_press: Message,
    size: Size,
    loading: bool,
) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    let content = label_content(label, size);
    let class = crate::style::button::primary();
    let press = if loading { None } else { Some(on_press) };
    styled(theme, content, size, class, loading, press)
}

/// A neutral solid secondary button — supporting action, lighter than
/// primary. Padding scales with `theme.spacing`.
pub fn secondary<'a, Message>(
    theme: &RustTheme,
    label: impl text::IntoFragment<'a>,
    on_press: Message,
) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    secondary_with(theme, label, on_press, Size::Default, false)
}

/// `secondary` with explicit [`Size`] and loading flag.
pub fn secondary_with<'a, Message>(
    theme: &RustTheme,
    label: impl text::IntoFragment<'a>,
    on_press: Message,
    size: Size,
    loading: bool,
) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    let content = label_content(label, size);
    let class = crate::style::button::secondary();
    let press = if loading { None } else { Some(on_press) };
    styled(theme, content, size, class, loading, press)
}

/// An outline button — transparent fill, 1px border. The default
/// secondary. Padding scales with `theme.spacing`.
pub fn outline<'a, Message>(
    theme: &RustTheme,
    label: impl text::IntoFragment<'a>,
    on_press: Message,
) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    outline_with(theme, label, on_press, Size::Default, false)
}

/// `outline` with explicit [`Size`] and loading flag.
pub fn outline_with<'a, Message>(
    theme: &RustTheme,
    label: impl text::IntoFragment<'a>,
    on_press: Message,
    size: Size,
    loading: bool,
) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    let content = label_content(label, size);
    let class = crate::style::button::outline();
    let press = if loading { None } else { Some(on_press) };
    styled(theme, content, size, class, loading, press)
}

/// A ghost button — no border, no fill. Tertiary/inline actions only.
/// Padding scales with `theme.spacing`.
pub fn ghost<'a, Message>(
    theme: &RustTheme,
    label: impl text::IntoFragment<'a>,
    on_press: Message,
) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    ghost_with(theme, label, on_press, Size::Default, false)
}

/// `ghost` with explicit [`Size`] and loading flag.
pub fn ghost_with<'a, Message>(
    theme: &RustTheme,
    label: impl text::IntoFragment<'a>,
    on_press: Message,
    size: Size,
    loading: bool,
) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    let content = label_content(label, size);
    let class = crate::style::button::ghost();
    let press = if loading { None } else { Some(on_press) };
    styled(theme, content, size, class, loading, press)
}

/// A destructive button — solid error-red. Irreversible actions only.
/// Padding scales with `theme.spacing`.
pub fn destructive<'a, Message>(
    theme: &RustTheme,
    label: impl text::IntoFragment<'a>,
    on_press: Message,
) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    destructive_with(theme, label, on_press, Size::Default, false)
}

/// `destructive` with explicit [`Size`] and loading flag.
pub fn destructive_with<'a, Message>(
    theme: &RustTheme,
    label: impl text::IntoFragment<'a>,
    on_press: Message,
    size: Size,
    loading: bool,
) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    let content = label_content(label, size);
    let class = crate::style::button::destructive();
    let press = if loading { None } else { Some(on_press) };
    styled(theme, content, size, class, loading, press)
}

/// An icon button — circular, square hit area, icon-only. Reserves the
/// accent fill for [`icon_primary`]; this neutral variant is the default.
pub fn icon<'a, Message>(
    glyph: impl text::IntoFragment<'a>,
    on_press: Message,
) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    icon_with(glyph, on_press, Size::Default, false)
}

/// `icon` with explicit [`Size`] and loading flag.
pub fn icon_with<'a, Message>(
    glyph: impl text::IntoFragment<'a>,
    on_press: Message,
    size: Size,
    loading: bool,
) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    let dim = size.min_height();
    let glyph_size = match size {
        Size::Small => 16.0,
        Size::Default => 18.0,
        Size::Large => 22.0,
    };
    let class = crate::style::button::icon();
    if loading {
        iced::widget::button(
            iced::widget::container(spinner(size.spinner_size()))
                .width(Length::Fixed(size.spinner_size()))
                .height(Length::Fixed(size.spinner_size()))
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
        )
        .padding(0)
        .width(Length::Fixed(dim as f32))
        .height(Length::Fixed(dim as f32))
        .style(class)
    } else {
        iced::widget::button(
            iced::widget::text(glyph)
                .font(crate::components::icons::font())
                .size(glyph_size)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
        )
        .padding(0)
        .width(Length::Fixed(dim as f32))
        .height(Length::Fixed(dim as f32))
        .style(class)
        .on_press(on_press)
    }
}

/// A primary (accent-filled) icon button — the one primary icon action.
pub fn icon_primary<'a, Message>(
    glyph: impl text::IntoFragment<'a>,
    on_press: Message,
) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    let dim = Size::Default.min_height();
    iced::widget::button(
        iced::widget::text(glyph)
            .font(crate::components::icons::font())
            .size(18.0)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .padding(0)
    .width(Length::Fixed(dim as f32))
    .height(Length::Fixed(dim as f32))
    .style(crate::style::button::icon_primary())
    .on_press(on_press)
}

// ---------------------------------------------------------------------------
// Legacy compatibility shims (existing call sites keep compiling)
// ---------------------------------------------------------------------------

/// Legacy alias for [`outline`].
pub fn outlined<'a, Message>(
    theme: &RustTheme,
    label: impl text::IntoFragment<'a>,
    on_press: Message,
) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    outline(theme, label, on_press)
}

/// Legacy alias for [`ghost`] — MUI `variant="text"` vocabulary.
pub fn text_button<'a, Message>(
    theme: &RustTheme,
    label: impl text::IntoFragment<'a>,
    on_press: Message,
) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    ghost(theme, label, on_press)
}

/// Legacy alias for [`destructive`].
pub fn danger<'a, Message>(
    theme: &RustTheme,
    label: impl text::IntoFragment<'a>,
    on_press: Message,
) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    destructive(theme, label, on_press)
}

/// Legacy alias — `success` is retired; maps to [`primary`] so existing
/// call sites render correctly without a competing green solid.
pub fn success<'a, Message>(
    theme: &RustTheme,
    label: impl text::IntoFragment<'a>,
    on_press: Message,
) -> Button<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    primary(theme, label, on_press)
}
