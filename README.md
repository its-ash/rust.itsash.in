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

## Configuring the theme

`RustTheme::light()` / `RustTheme::dark()` use RustTheme's own brand palette
by default (built from `--yale-blue`, `--stormy-teal`, `--graphite`,
`--alabaster-grey`, `--white`). To customize the palette and/or font
globally, build a `RustTheme` through `RustTheme::builder()` once at
startup:

```rust
use rustheme::prelude::*;

fn main() {
    // The palette is per-instance: build as many custom RustThemes as you
    // like (e.g. a light and dark variant of your own brand).
    let theme = RustTheme::builder()
        .palette(Palette::brand_dark()) // or your own `Palette { .. }`
        .font("Inter")                  // must be installed on the system
        .monospace_font("Fira Code")
        .build();

    // ...pass `theme` (or a closure returning it) to `iced::application`.
    let _ = theme;
}
```

The font family is set **process-wide** rather than per-`RustTheme`, and
should only be configured on the first `RustTheme` you build, before
constructing any UI: iced applies fonts at widget-construction time, not
lazily per theme instance the way it does for colors, so there's no way to
vary the font per-`RustTheme`-instance without threading a theme reference
through every component call. If you skip `.font(...)`, RustTheme falls back
to the OS-native system font per platform (SF Pro Text / Segoe UI /
Cantarell) — see `rustheme_core::set_global_font_family` for the underlying
mechanism.

### Heading vs. body font

The type scale distinguishes a **heading** role (`h1`..`h6`, `subtitle1`/`2`,
plus structural titles like the app bar and dialog) from the regular
**body/UI** role (everything else), so you can pair a display font with a
body font the way a real design system does:

```rust
use rustheme::prelude::*;

let theme = RustTheme::builder()
    .font("Source Sans Pro")     // body/UI text
    .heading_font("Playfair Display") // h1..h6, subtitle1/2, app bar, dialog titles
    .build();
```

`.font(...)` / `.heading_font(...)` only set the family **name** RustTheme
uses — the font's bytes still have to be loaded into iced separately (a
named font with no bytes loaded silently falls back to a generic font,
since neither of these fonts ships with any OS). RustTheme bundles exactly
this Playfair Display + Source Sans Pro pairing as ready-to-use font files,
so you don't have to source and embed them yourself:

```rust
fn main() -> iced::Result {
    rustheme::fonts::use_default_pairing(); // sets the family names
    rustheme::load_fonts!(                  // loads their bytes into iced
        iced::application("My App", update, view)
    )
    .run()
}
```

(`load_fonts!` is a macro rather than a function because `.font(bytes)`
lives on `iced::application(...)`'s builder type, which is generic over an
iced-internal trait that can't be named as a bound outside the `iced` crate
itself — the macro expands inline at the call site instead, where the
concrete builder type is already known.)

Both fonts are bundled under the SIL Open Font License in
`rustheme/assets/fonts/` (license texts included alongside the `.ttf`
files) and add about 850KB to your binary. If you don't call
`use_default_pairing()` / `load_fonts!`, none of that is used — the six
`include_bytes!`'d files are still compiled into the `rustheme` crate
either way, since Rust can't conditionally embed them at runtime, but
nothing loads or references them unless you opt in.

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
| Badge       | count, dot, standard; any semantic color; 4 corner positions |
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
