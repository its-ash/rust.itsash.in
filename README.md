# RustTheme

A MUI-inspired theming framework and component library for the [Iced](https://iced.rs)
GUI toolkit in Rust. RustTheme gives you a design-token system (palette,
typography, spacing, shape), a `RustTheme` type wired into iced 0.13's
`Catalog` styling traits, and an ergonomic component library — buttons, text
fields, cards, dialogs, snackbars, and more — pre-styled to named variants
that mirror Material UI's API surface.

## Install

RustTheme is not published to crates.io. Add it as a path or git dependency:

```toml
# Cargo.toml
[dependencies]
rustheme = { git = "https://github.com/YOUR_USERNAME/rustheme" }
# or, if vendored locally:
# rustheme = { path = "../rustheme/rustheme" }
```

## Quick example

```rust
use rustheme::prelude::*;

#[derive(Debug, Clone)]
enum Message {
    Save,
}

struct App {
    theme: RustTheme,
}

impl App {
    fn theme(&self) -> RustTheme {
        self.theme.clone()
    }

    fn view(&self) -> Element<'_, Message, RustTheme> {
        card::elevated(
            iced::widget::column![
                iced::widget::text("Settings"),
                button::primary("Save", Message::Save),
            ]
            .spacing(12),
        )
        .into()
    }
}
```

Run the full showcase:

```
cargo run -p gallery
```

## Components

| Component   | Variants                                              |
|-------------|--------------------------------------------------------|
| Button      | primary, secondary, outlined, text, danger, success    |
| TextField   | outlined, filled                                        |
| Checkbox    | primary                                                  |
| Radio       | primary                                                  |
| Switch      | primary (backed by iced's `Toggler`)                     |
| Slider      | primary                                                  |
| Select      | outlined (backed by iced's `PickList`)                   |
| Divider     | horizontal, vertical                                     |
| Tooltip     | default                                                  |
| Card        | card, elevated, outlined                                 |
| Chip        | filled, outlined                                         |
| Avatar      | initials                                                 |
| Badge       | count, dot                                               |
| List        | list, item                                               |
| Progress    | linear, circular                                         |
| AppBar      | app_bar                                                  |
| Tabs        | tabs                                                     |
| Menu        | menu (popover, composed via `stack`)                      |
| Dialog      | dialog (modal, composed via `stack`)                       |
| Snackbar    | snackbar (composed via `stack`, with dismiss action)        |
| IconButton  | primary, secondary, outlined, text                        |
| Fab         | primary, secondary, extended                               |
| Alert       | success, info, warning, error, with_title (dismissible)     |
| Accordion   | accordion (expand/collapse, no layout animation)             |
| Breadcrumbs | breadcrumbs (clickable trail, last segment static)           |
| Pagination  | pagination (page buttons + prev/next)                        |
| Skeleton    | text, rectangular, circular (static placeholder)             |
| Stepper     | stepper (active/completed/upcoming steps)                    |
| Backdrop    | scrim, backdrop (composed via `stack`)                        |
| Rating      | rating (clickable star row)                                   |
| Table       | table (header + rows)                                          |
| Drawer      | drawer (left/right anchored, composed via `stack`)              |
| Grid/Stack  | layout::stack, layout::grid (spacing-token layout helpers)      |
| Autocomplete| autocomplete (text field + filtered dropdown)                    |
| SpeedDial   | speed_dial (expandable Fab stack)                                  |
| Timeline    | timeline (vertical event sequence)                                  |

## Workspace layout

```
rustheme-core/   Design tokens: Palette, Typography, Spacing, Shape.
rustheme/        RustTheme type, iced Catalog impls, component library.
examples/gallery/  Runnable showcase of every component.
```

See `.github/copilot-instructions.md` (symlinked as `CLAUDE.md`) for
conventions on adding new components.

## License

MIT. See `LICENSE` (placeholder — add full license text before distribution).
