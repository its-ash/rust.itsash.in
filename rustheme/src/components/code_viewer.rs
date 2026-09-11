//! `CodeViewer`: a read-only, syntax-highlighted code block with line
//! numbers — the shared primitive behind an Editor's file preview, a
//! Database Studio's SQL/result panel, or any "show me this file" surface.
//!
//! Highlighting is done once per `view()` call via `syntect`'s bundled
//! TextMate grammars/themes (no network fetch, no user-supplied grammar
//! files needed) and rendered as [`iced::widget::rich_text`] spans — one
//! span per syntect token, colored from the syntect theme rather than
//! `RustTheme`'s palette, since a code theme (e.g. "base16-ocean.dark") is
//! its own deliberately-fixed color system independent of the app chrome.
//!
//! `SyntaxSet`/`ThemeSet` loading parses embedded grammar/theme dumps and
//! is not free (single-digit milliseconds, one-time) — call
//! [`Highlighter::new`] once (e.g. stored in your app state) and reuse it
//! across renders rather than rebuilding it per `view()`.

use crate::theme::RustTheme;
use iced::widget::{column, container, row, rich_text, span, text};
use iced::{Color, Element, Length};
use rustheme_core::{system_monospace_font, FontWeight};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::parsing::{SyntaxReference, SyntaxSet};

/// Owns the parsed syntax/theme definitions. Expensive to build (parses
/// syntect's bundled dumps), cheap to use — construct once and keep it in
/// your application state.
pub struct Highlighter {
    syntax_set: SyntaxSet,
    theme: Theme,
}

impl Highlighter {
    /// Loads the default bundled syntax set and the given syntect theme by
    /// name (e.g. `"base16-ocean.dark"`, `"InspiredGitHub"`,
    /// `"Solarized (light)"` — see `ThemeSet::load_defaults()` for the full
    /// list). Falls back to the first available theme if `theme_name`
    /// isn't found.
    pub fn new(theme_name: &str) -> Self {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme_set = ThemeSet::load_defaults();
        let theme = theme_set
            .themes
            .get(theme_name)
            .or_else(|| theme_set.themes.values().next())
            .expect("syntect bundles at least one default theme")
            .clone();
        Self { syntax_set, theme }
    }

    fn syntax_for(&self, language_or_extension: &str) -> &SyntaxReference {
        self.syntax_set
            .find_syntax_by_token(language_or_extension)
            .or_else(|| self.syntax_set.find_syntax_by_extension(language_or_extension))
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text())
    }

    /// The theme's own background color, for the caller to paint behind
    /// the code block so highlighted tokens read correctly against it.
    pub fn background(&self) -> Color {
        self.theme
            .settings
            .background
            .map(|c| Color::from_rgba8(c.r, c.g, c.b, c.a as f32 / 255.0))
            .unwrap_or(Color::from_rgb8(0x1e, 0x1e, 0x1e))
    }
}

impl Default for Highlighter {
    /// Uses `"base16-eighties.dark"`, a legible default that works well in
    /// both light- and dark-mode host apps since the code block paints its
    /// own background regardless of `RustTheme`'s mode.
    fn default() -> Self {
        Self::new("base16-eighties.dark")
    }
}

/// Renders `source` as a highlighted, line-numbered, read-only code block.
/// `language` is a syntect token or file extension (e.g. `"rust"`, `"rs"`,
/// `"json"`, `"toml"`, `"sh"`) — unrecognized values fall back to plain
/// text rather than erroring.
pub fn code_viewer<'a, Message>(highlighter: &Highlighter, source: &'a str, language: &str) -> Element<'a, Message, RustTheme>
where
    Message: Clone + 'static,
{
    let syntax = highlighter.syntax_for(language);
    let mut highlight = HighlightLines::new(syntax, &highlighter.theme);
    let bg = highlighter.background();

    let mut lines_col = column![].spacing(0);

    for (index, line) in source.lines().enumerate() {
        // syntect's `LinesWithEndings`-free API wants a trailing newline to
        // correctly close line-spanning scopes (e.g. block comments); `source.lines()`
        // strips it, so it's added back before highlighting.
        let with_newline = format!("{line}\n");
        let ranges = highlight
            .highlight_line(&with_newline, &highlighter.syntax_set)
            .unwrap_or_default();

        let spans: Vec<_> = ranges
            .iter()
            .map(|(style, text)| {
                let c = style.foreground;
                span(text.trim_end_matches('\n').to_string())
                    .color(Color::from_rgba8(c.r, c.g, c.b, c.a as f32 / 255.0))
            })
            .collect();

        let line_number = text(format!("{:>4}", index + 1))
            .size(13)
            .font(system_monospace_font(FontWeight::Regular))
            .color(Color { a: 0.4, ..Color::WHITE });

        lines_col = lines_col.push(
            row![
                line_number,
                rich_text::<Message, RustTheme, iced::Renderer>(spans)
                    .size(13)
                    .font(system_monospace_font(FontWeight::Regular))
                    .width(Length::Fill),
            ]
            .spacing(14)
            .padding([1, 0]),
        );
    }

    container(iced::widget::scrollable(lines_col.padding([10, 14])))
        .width(Length::Fill)
        .style(move |theme: &RustTheme| iced::widget::container::Style {
            background: Some(iced::Background::Color(bg)),
            border: iced::Border { color: theme.palette.divider, width: 1.0, radius: theme.shape.small.into() },
            ..Default::default()
        })
        .into()
}
