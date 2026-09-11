//! Material Symbols Rounded icon set, bundled and ready to use.
//!
//! Each variant of the [`Icon`] enum maps to a named Material Symbols
//! codepoint. Use [`icon`] to build a sized `Text` widget already wired to
//! the embedded Material Symbols font, or pass an [`Icon`] to any
//! icon-bearing component (e.g. `icon_button`, `fab`).
//!
//! The font is loaded via [`crate::fonts::load_fonts!`] — no extra setup
//! beyond calling that macro in `main()` alongside
//! [`crate::fonts::use_default_pairing`].
//!
//! ```no_run
//! use rustheme::prelude::*;
//! use rustheme::icons::Icon;
//!
//! fn view<'a, Message: Clone + 'a>(_s: &()) -> Element<'a, Message, RustTheme> {
//!     rustheme::icons::icon(Icon::Search, 24.0).into()
//! }
//! ```

use iced::Font;
use crate::theme::RustTheme;

/// A Material Symbols Rounded icon, identified by its ligature name.
/// Each variant carries the private-use codepoint Google assigns to that
/// ligature, so rendering uses the codepoint directly (faster and more
/// reliable than ligature substitution, which iced's text shaper doesn't
/// do).
///
/// The set covers the icons RustTheme components and the gallery use.
/// Add new icons by looking up the codepoint at
/// <https://fonts.google.com/icons> (the codepoint is shown in the icon
/// detail panel) and adding a variant here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Icon {
    /// `search` — magnifying glass.
    Search,
    /// `home` — house.
    Home,
    /// `settings` — gear.
    Settings,
    /// `person` — user silhouette.
    Person,
    /// `close` — ×.
    Close,
    /// `check` — ✓.
    Check,
    /// `add` — +.
    Add,
    /// `remove` — −.
    Remove,
    /// `delete` — trash can.
    Delete,
    /// `edit` — pencil.
    Edit,
    /// `favorite` — filled heart.
    Favorite,
    /// `favorite_border` — outlined heart.
    FavoriteBorder,
    /// `mail` — envelope.
    Mail,
    /// `notifications` — bell.
    Notifications,
    /// `menu` — hamburger.
    Menu,
    /// `arrow_back` — ←.
    ArrowBack,
    /// `arrow_forward` — →.
    ArrowForward,
    /// `chevron_right` — ›.
    ChevronRight,
    /// `chevron_left` — ‹.
    ChevronLeft,
    /// `expand_more` — ⌄.
    ExpandMore,
    /// `expand_less` — ⌃.
    ExpandLess,
    /// `refresh` — circular arrows.
    Refresh,
    /// `save` — floppy disk.
    Save,
    /// `cancel` — × in circle.
    Cancel,
    /// `done` — ✓ in circle.
    Done,
    /// `info` — ⓘ.
    Info,
    /// `warning` — ⚠.
    Warning,
    /// `error` — filled ⚠.
    Error,
    /// `visibility` — eye.
    Visibility,
    /// `visibility_off` — eye with slash.
    VisibilityOff,
    /// `star` — filled star.
    Star,
    /// `star_border` — outlined star.
    StarBorder,
    /// `more_vert` — ⋮.
    MoreVert,
    /// `more_horiz` — ⋯.
    MoreHoriz,
    /// `share` — share nodes.
    Share,
    /// `content_copy` — stacked sheets.
    ContentCopy,
    /// `download` — arrow into tray.
    Download,
    /// `upload` — arrow out of tray.
    Upload,
    /// `lock` — padlock.
    Lock,
    /// `help` — ?.
    Help,
    /// `folder` — closed folder.
    Folder,
    /// `folder_open` — open folder.
    FolderOpen,
    /// `description` — document/file page.
    Description,
    /// `terminal` — command prompt `>_`.
    Terminal,
    /// `play_arrow` — ▶.
    PlayArrow,
    /// `stop` — ■.
    Stop,
    /// `memory` — chip/RAM.
    Memory,
    /// `storage` — database/server stack.
    Storage,
}

impl Icon {
    /// The Material Symbols codepoint (private-use area) for this icon.
    pub const fn codepoint(self) -> char {
        match self {
            Self::Search => '\u{e8b6}',
            Self::Home => '\u{e88a}',
            Self::Settings => '\u{e8b8}',
            Self::Person => '\u{e7fd}',
            Self::Close => '\u{e5cd}',
            Self::Check => '\u{e5ca}',
            Self::Add => '\u{e145}',
            Self::Remove => '\u{e15b}',
            Self::Delete => '\u{e872}',
            Self::Edit => '\u{e3c9}',
            Self::Favorite => '\u{e87d}',
            Self::FavoriteBorder => '\u{e87e}',
            Self::Mail => '\u{e158}',
            Self::Notifications => '\u{e7f4}',
            Self::Menu => '\u{e5d2}',
            Self::ArrowBack => '\u{e5c4}',
            Self::ArrowForward => '\u{e5c8}',
            Self::ChevronRight => '\u{e5cc}',
            Self::ChevronLeft => '\u{e5cb}',
            Self::ExpandMore => '\u{e5cf}',
            Self::ExpandLess => '\u{e5ce}',
            Self::Refresh => '\u{e5d5}',
            Self::Save => '\u{e161}',
            Self::Cancel => '\u{e5c9}',
            Self::Done => '\u{e876}',
            Self::Info => '\u{e88e}',
            Self::Warning => '\u{e002}',
            Self::Error => '\u{e000}',
            Self::Visibility => '\u{e8f4}',
            Self::VisibilityOff => '\u{e8f5}',
            Self::Star => '\u{e838}',
            Self::StarBorder => '\u{e83a}',
            Self::MoreVert => '\u{e5d4}',
            Self::MoreHoriz => '\u{e5d3}',
            Self::Share => '\u{e80d}',
            Self::ContentCopy => '\u{e14d}',
            Self::Download => '\u{e2c0}',
            Self::Upload => '\u{e2c6}',
            Self::Lock => '\u{e897}',
            Self::Help => '\u{e8fd}',
            Self::Folder => '\u{e2c7}',
            Self::FolderOpen => '\u{e2c8}',
            Self::Description => '\u{e873}',
            Self::Terminal => '\u{eb8e}',
            Self::PlayArrow => '\u{e037}',
            Self::Stop => '\u{e047}',
            Self::Memory => '\u{e322}',
            Self::Storage => '\u{e1db}',
        }
    }

    /// The single-char `&'static str` for this icon — for passing to
    /// components that take `&'static str` (e.g. `fab`, `speed_dial`).
    pub const fn as_str(self) -> &'static str {
        // Safety: every codepoint above is a single valid Unicode scalar,
        // so `from_char` produces a 1-char str with no interior nulls.
        // This avoids allocating a String per call.
        match self {
            Self::Search => "\u{e8b6}",
            Self::Home => "\u{e88a}",
            Self::Settings => "\u{e8b8}",
            Self::Person => "\u{e7fd}",
            Self::Close => "\u{e5cd}",
            Self::Check => "\u{e5ca}",
            Self::Add => "\u{e145}",
            Self::Remove => "\u{e15b}",
            Self::Delete => "\u{e872}",
            Self::Edit => "\u{e3c9}",
            Self::Favorite => "\u{e87d}",
            Self::FavoriteBorder => "\u{e87e}",
            Self::Mail => "\u{e158}",
            Self::Notifications => "\u{e7f4}",
            Self::Menu => "\u{e5d2}",
            Self::ArrowBack => "\u{e5c4}",
            Self::ArrowForward => "\u{e5c8}",
            Self::ChevronRight => "\u{e5cc}",
            Self::ChevronLeft => "\u{e5cb}",
            Self::ExpandMore => "\u{e5cf}",
            Self::ExpandLess => "\u{e5ce}",
            Self::Refresh => "\u{e5d5}",
            Self::Save => "\u{e161}",
            Self::Cancel => "\u{e5c9}",
            Self::Done => "\u{e876}",
            Self::Info => "\u{e88e}",
            Self::Warning => "\u{e002}",
            Self::Error => "\u{e000}",
            Self::Visibility => "\u{e8f4}",
            Self::VisibilityOff => "\u{e8f5}",
            Self::Star => "\u{e838}",
            Self::StarBorder => "\u{e83a}",
            Self::MoreVert => "\u{e5d4}",
            Self::MoreHoriz => "\u{e5d3}",
            Self::Share => "\u{e80d}",
            Self::ContentCopy => "\u{e14d}",
            Self::Download => "\u{e2c0}",
            Self::Upload => "\u{e2c6}",
            Self::Lock => "\u{e897}",
            Self::Help => "\u{e8fd}",
            Self::Folder => "\u{e2c7}",
            Self::FolderOpen => "\u{e2c8}",
            Self::Description => "\u{e873}",
            Self::Terminal => "\u{eb8e}",
            Self::PlayArrow => "\u{e037}",
            Self::Stop => "\u{e047}",
            Self::Memory => "\u{e322}",
            Self::Storage => "\u{e1db}",
        }
    }
}

/// The embedded Material Symbols Rounded font, for use in
/// `.font(...)` calls on text widgets that render icons.
pub fn font() -> Font {
    crate::fonts::material_symbols_font()
}

/// Builds a `Text` widget rendering the given [`Icon`] at `size` px,
/// already wired to the Material Symbols Rounded font. Center-aligned for
/// use inside buttons, icon buttons, and other containers.
pub fn icon<'a>(icon: Icon, size: f32) -> iced::widget::Text<'a, RustTheme, iced::Renderer> {
    iced::widget::text(glyph(icon))
        .font(font())
        .size(size)
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center)
}

/// Returns the single-char `&'static str` for an icon — useful when an
/// icon glyph needs to be composed into a row/column or passed to a
/// component that takes `impl IntoFragment` or `&'static str`.
pub fn glyph(icon: Icon) -> &'static str {
    icon.as_str()
}
