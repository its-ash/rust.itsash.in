# RustTheme Component & Theming Reference

Machine-readable reference for every public component, theme token, and
configuration API in RustTheme. Intended to be pasted into an LLM's context
(or read directly by a coding agent) so it can pick the correct component and
props without searching the source.

- Crate `rustheme-core`: design tokens only (`Palette`, `Typography`,
  `Spacing`, `Shape`, `Elevation`). No `iced` dependency.
- Crate `rustheme`: `RustTheme` + iced `Catalog` styles + the component
  library below. Import everything via `use rustheme::prelude::*;`.

Every component function returns (or builds into) `iced::Element<'_,
Message, RustTheme>`, styled automatically from whatever `RustTheme` is
active — you never pass colors/radii to a component call; you configure them
once on the theme (see [Theming](#theming) below) and every component
follows.

## Choosing a component — quick index

| Need | Use |
|---|---|
| Primary call-to-action button | [`button::primary`](#button) |
| Secondary/cancel button | [`button::outline`](#button) or [`button::ghost`](#button) |
| Irreversible/destructive action | [`button::destructive`](#button) |
| Icon-only button | [`icon_button::*`](#iconbutton) |
| Floating action button | [`fab::*`](#fab) |
| Expanding FAB with sub-actions | [`speed_dial::speed_dial`](#speeddial) |
| Text input | [`text_field::outlined`](#textfield) / [`text_field::filled`](#textfield) |
| Type-ahead search input | [`autocomplete::autocomplete`](#autocomplete) |
| On/off boolean | [`switch::switch`](#switch) |
| Multi-select boolean | [`checkbox::checkbox`](#checkbox) |
| Single choice from a small set | [`radio::radio`](#radio) or [`segmented::segmented`](#segmented) |
| Single choice from a dropdown | [`select::select`](#select) |
| Numeric range input | [`slider::slider`](#slider) |
| 1..5 star rating input | [`rating::rating`](#rating) |
| Page/section container | [`card::card`](#card) / [`card::elevated`](#card) / [`card::outlined`](#card) |
| Small status/label pill | [`chip::filled`](#chip) / [`chip::outlined`](#chip) |
| Status/label pill w/o interaction | [`badge::standard`](#badge) |
| Notification count/dot overlay | [`badge::count`](#badge) / [`badge::dot`](#badge) |
| User avatar | [`avatar::initials`](#avatar) |
| Severity banner (success/info/warn/error) | [`alert::*`](#alert) |
| Row of items with dividers | [`list::list`](#list) |
| Tabular data | [`table::table`](#table) |
| Tab navigation | [`tabs::tabs`](#tabs) |
| Bottom nav bar (mobile-style) | [`bottom_navigation::bottom_navigation`](#bottomnavigation) |
| Path/location trail | [`breadcrumbs::breadcrumbs`](#breadcrumbs) |
| Page number navigation | [`pagination::pagination`](#pagination) |
| Multi-step wizard progress | [`stepper::stepper`](#stepper) |
| Expand/collapse section | [`accordion::accordion`](#accordion) |
| Vertical event history | [`timeline::timeline`](#timeline) |
| Modal confirmation/form | [`dialog::dialog`](#dialog) |
| Sidebar panel | [`drawer::drawer`](#drawer) |
| Temporary bottom message | [`snackbar::snackbar`](#snackbar) |
| Popover option list | [`menu::menu`](#menu) |
| Full-screen dim + spinner | [`backdrop::backdrop`](#backdrop) |
| Hover-triggered label | [`tooltip::tooltip`](#tooltip) |
| Loading placeholder shape | [`skeleton::*`](#skeleton) |
| Loading bar / spinner | [`progress::linear`](#progress) / [`progress::circular`](#progress) |
| Top page header bar | [`app_bar::app_bar`](#appbar) |
| Empty list/search placeholder | [`empty_state::empty_state`](#emptystate) |
| Horizontal/vertical rule | [`divider::horizontal`](#divider) / [`divider::vertical`](#divider) |
| Row/column with themed gap | [`layout::stack`](#layout) |
| Wrapping grid | [`layout::grid`](#layout) |
| Icon glyph | [`icons::icon`](#icons) / [`icons::glyph`](#icons) |
| Search input | [`search_bar::search_bar`](#searchbar) |
| Section title + actions row | [`section_header::section_header_with`](#sectionheader) |
| Settings/preferences list row | [`settings_row::settings_row`](#settingsrow) |
| KPI/metric number card | [`stat_card::stat_card`](#statcard) / [`stat_card::with_trend`](#statcard) |
| Bulk-action row above a list/table | [`toolbar::toolbar`](#toolbar) |
| Yes/no confirmation prompt | [`confirm_dialog::confirm_dialog`](#confirmdialog) / [`confirm_dialog::destructive`](#confirmdialog) |
| Multi-item notification stack | [`toast::toast_stack`](#toast) |
| Right-click popover menu | [`context_menu::context_menu`](#contextmenu) |
| Persistent app sidebar navigation | [`sidebar_nav::sidebar_nav`](#sidebarnav) |
| Label/value detail list | [`key_value_list::key_value_list`](#keyvaluelist) |
| Card grid with loading/empty states | [`data_card_grid::data_card_grid`](#datacardgrid) |
| Pricing/feature comparison grid | [`comparison_table::comparison_table`](#comparisontable) |

---

## Theming

### Quick start

```rust
use rustheme::prelude::*;

// One line, no configuration — RustTheme's brand look:
let theme = RustTheme::light();   // or RustTheme::dark()
```

### Global configuration (recommended): `RustThemeConfig`

Use `RustThemeConfig` when you want **one settings object** that drives both
light and dark mode, with optional per-mode overrides — e.g. a theme
generated from user preferences, or a design-system entry point.

```rust
use rustheme::prelude::*;
use rustheme_core::hex;

let config = RustThemeConfig::new()
    .radius(10.0)                        // global corner radius, BOTH modes
    .primary_color(hex("#3F51B5"))       // global brand/accent color, BOTH modes
    .text_color(hex("#1B1B1B"))          // global text color, BOTH modes
    .surface_color(hex("#FFFFFF"))       // global background/card color, BOTH modes
    .divider_color(hex("#E0E0E0"))       // global divider/hairline color, BOTH modes
    // Per-mode overrides win over the shared setters above:
    .dark_primary_color(hex("#B7C4FF"))  // dark-mode-only accent
    .dark_text_color(hex("#F2F2F2"))     // dark-mode-only text color
    .dark_surface_color(hex("#1E1E1E"))  // dark-mode-only background
    .dark_radius(6.0);                   // dark-mode-only corner radius

let light_theme = config.light();
let dark_theme = config.dark();

// Pick per-frame based on user's mode toggle:
fn theme(is_dark: bool, config: &RustThemeConfig) -> RustTheme {
    if is_dark { config.dark() } else { config.light() }
}
```

`RustThemeConfig` builder methods (all take/return `Self` — call chain
order doesn't matter, only presence):

| Method | Type | Applies to | Overridden by |
|---|---|---|---|
| `.radius(f32)` | corner radius (px) | both modes | `.light_radius` / `.dark_radius` |
| `.primary_color(Color)` | brand/accent color | both modes | `.light_primary_color` / `.dark_primary_color` |
| `.text_color(Color)` | primary text color | both modes | `.light_text_color` / `.dark_text_color` |
| `.surface_color(Color)` | background + card color | both modes | `.light_surface_color` / `.dark_surface_color` |
| `.divider_color(Color)` | hairline/divider color | both modes | `.light_divider_color` / `.dark_divider_color` |
| `.light_radius(f32)` | corner radius, light mode only | light | — |
| `.light_primary_color(Color)` | accent, light mode only | light | — |
| `.light_text_color(Color)` | text, light mode only | light | — |
| `.light_surface_color(Color)` | background, light mode only | light | — |
| `.light_divider_color(Color)` | divider, light mode only | light | — |
| `.dark_radius(f32)` | corner radius, dark mode only | dark | — |
| `.dark_primary_color(Color)` | accent, dark mode only | dark | — |
| `.dark_text_color(Color)` | text, dark mode only | dark | — |
| `.dark_surface_color(Color)` | background, dark mode only | dark | — |
| `.dark_divider_color(Color)` | divider, dark mode only | dark | — |
| `.font(&'static str)` | process-wide UI font family name | both modes | — |
| `.monospace_font(&'static str)` | process-wide monospace font family name | both modes | — |
| `.heading_font(&'static str)` | process-wide heading font family name | both modes | — |

Then call `.light()` / `.dark()` to materialize a `RustTheme`. Precedence
per mode: **per-mode setter > shared setter > brand default**.

`Color` above is `iced::Color`; build one from hex with
`rustheme_core::hex("#RRGGBB")` or `hex("#RRGGBBAA")`.

### Single-theme configuration: `RustTheme::builder()`

Use `RustTheme::builder()` when you only need one `RustTheme` instance (e.g.
a fixed-mode app, or you're already branching light/dark yourself).

```rust
use rustheme::prelude::*;
use rustheme_core::hex;

let theme = RustTheme::builder()
    .palette(Palette::brand_dark())     // base palette (see Palette presets below)
    .primary_color(hex("#B7C4FF"))      // overrides palette's primary + derives light/dark/contrast
    .text_color(hex("#F2F2F2"))         // overrides palette's text.primary + derives secondary/disabled
    .surface_color(hex("#1E1E1E"))      // overrides palette's background.default + .paper
    .divider_color(hex("#3A3A3A"))      // overrides palette's divider
    .border_radius(20.0)                // uniform corner radius across every shape token
    .font("Inter")                      // process-wide UI font (see Fonts below)
    .build();
```

Builder methods:

| Method | Effect |
|---|---|
| `.palette(Palette)` | Sets the full base palette (see presets below) |
| `.primary_color(Color)` | Global brand/accent color; derives lighter/darker/contrast-text automatically |
| `.text_color(Color)` | Global primary text color; derives secondary/disabled tones against the surface |
| `.surface_color(Color)` | Global canvas (`background.default`) and card (`background.paper`) color |
| `.divider_color(Color)` | Global hairline/divider color |
| `.border_radius(f32)` | Uniform corner radius for every shape token (`none`/`full` keep semantic meaning) |
| `.typography(Typography)` | Custom type scale |
| `.spacing(Spacing)` | Custom base spacing unit (default 8px) |
| `.shape(Shape)` | Custom full shape scale (overridden by `.border_radius` if both set) |
| `.elevation(Elevation)` | Default resting elevation level (default `Level1`) |
| `.font(&'static str)` | Process-wide UI font family (see Fonts) |
| `.monospace_font(&'static str)` | Process-wide monospace font family |
| `.heading_font(&'static str)` | Process-wide heading font family (h1-h6, dialog/app-bar titles) |

### Palette presets

| Preset | Description |
|---|---|
| `Palette::brand_light()` / `Palette::brand_dark()` | RustTheme's default look: native OS surface/text colors per platform (macOS/Windows/Linux) + a deep-teal brand accent. `RustTheme::light()`/`::dark()` use these. |
| `Palette::light()` / `Palette::dark()` | A Material 3 -inspired vibrant palette (purple primary, teal secondary) instead of the native/brand look. |

All presets pass WCAG AA contrast assertions at construction time (panics
if a custom `Palette` you hand-build fails contrast — use
`rustheme_core::assert_aa` / `assert_aa_large` to check your own colors).

### Design tokens reference

Read these off `theme.palette` / `.typography` / `.spacing` / `.shape` /
`.elevation` inside a custom style function — never hardcode a color or
radius in component code.

**`Palette`** (`theme.palette`):
- `primary`, `secondary`, `error`, `warning`, `info`, `success`: each a
  `ColorSet { main, light, dark, contrast_text }`
- `background: BackgroundColors { default, paper }`
- `text: TextColors { primary, secondary, disabled }`
- `divider: Color`

**`Shape`** (`theme.shape`) — M3 10-level corner radius scale:
`none` (0), `extra_small` (4), `small` (8), `medium`/`border_radius` (12,
default), `large` (16), `large_increased` (20), `extra_large` (28),
`extra_large_increased` (32), `extra_extra_large` (48), `full` (fully
rounded, use `dimension / 2.0` at the call site for pills/circles).

**`Spacing`** (`theme.spacing`) — one base unit (default 8px);
`theme.spacing.unit(factor)` returns `factor * base`.

**`Typography`** (`theme.typography`) — type scale + font weights; read via
`rustheme_core::system_font(FontWeight)` / `system_monospace_font(...)` /
`heading_font(...)` helpers rather than the struct directly in most
component code.

**`Elevation`** (`theme.elevation`) — `Level0`..`Level5`, M3 shadow levels.

### Fonts

Font family is **process-wide** (iced applies fonts at widget-construction
time), so set it once in `main()` before building any UI — never per
`RustTheme` instance beyond the first:

```rust
fn main() -> iced::Result {
    rustheme::fonts::use_default_pairing(); // Playfair Display (headings) + Roboto (body)
    // or: rustheme::fonts::use_roboto();   // Roboto everywhere — body AND headings, no display pairing
    rustheme::load_fonts!(iced::application(App::title, App::update, App::view))
        .theme(App::theme)
        .run()
}
```

`use_roboto()` is the right call when you want one consistent main font
throughout the app rather than a heading/body pairing — it still requires
`load_fonts!` to load Roboto's bytes.

Or set custom families via `RustTheme::builder().font(...)` /
`.heading_font(...)` / `.monospace_font(...)` (or the equivalent
`RustThemeConfig` methods) — also once, before any UI is built.

---

## Components

Import via `use rustheme::prelude::*;` (brings every module below into
scope, plus `RustTheme`, `RustThemeConfig`, `Palette`, `FontFamily`, and
iced's `Element`) — or `use rustheme::<name>;` for one module.

### Button

Six variants × three sizes. **One `primary` per surface** — secondaries
drop to `outline`/`ghost` so they never compete at equal visual weight.
Every size meets the 44×44px minimum tap target.

```rust
button::primary(label: impl text::IntoFragment, on_press: Message) -> Button<Message, RustTheme>
button::secondary(label, on_press) -> Button<Message, RustTheme>
button::outline(label, on_press) -> Button<Message, RustTheme>       // default secondary choice
button::ghost(label, on_press) -> Button<Message, RustTheme>          // tertiary/inline only
button::destructive(label, on_press) -> Button<Message, RustTheme>    // irreversible actions only
button::icon(glyph, on_press) -> Button<Message, RustTheme>           // icon-only, neutral

// Explicit size + loading state (label swaps for a spinner; on_press is ignored while loading):
button::primary_with(label, on_press, size: button::Size, loading: bool) -> Button<Message, RustTheme>
// same _with pattern for secondary_with / outline_with / ghost_with / destructive_with / icon_with

button::icon_primary(glyph, on_press) -> Button<Message, RustTheme>   // the one accent-filled icon action

// Legacy aliases (kept for compatibility): outlined() = outline, text_button() = ghost,
// danger() = destructive, success() = primary (green solid variant retired)
```

`button::Size`: `Small` (13px text, 32px min height) | `Default` (14px,
40px, `#[default]`) | `Large` (16px, 48px).

### Card

```rust
card::card(content: impl Into<Element>) -> Container       // subtle shadow
card::elevated(content) -> Container                        // stronger shadow
card::outlined(content) -> Container                         // bordered, no fill
```

### Chip

```rust
chip::filled(label: impl Into<String>) -> Container    // filled pill
chip::outlined(label) -> Container                       // outlined pill
```

### Badge

Overlays a marker on a corner of any element.

```rust
badge::Color // enum: Primary | Secondary | Error | Warning | Info | Success
badge::Position // enum: TopRight | TopLeft | BottomRight | BottomLeft

badge::count(content: impl Into<Element>, count: u32, max: u32, color: badge::Color, position: badge::Position) -> Element
badge::simple_count(content, count: u32) -> Element         // top-right, error color, max 99
badge::dot(content, color: badge::Color, position: badge::Position) -> Element
badge::simple_dot(content) -> Element                          // top-right, error color
badge::standard(label: impl Into<String>, color: badge::Color) -> Container  // standalone pill, no anchor
```

`count == 0` renders no badge; counts `> max` render as `"{max}+"`.

### Avatar

```rust
avatar::initials(initials: impl Into<String>, diameter: f32) -> Container   // up to 2 chars
```

### Alert

```rust
alert::Severity // enum: Success | Info | Warning | Error

alert::success(body: impl Into<String>, on_dismiss: Option<Message>) -> Container
alert::info(body, on_dismiss) -> Container
alert::warning(body, on_dismiss) -> Container
alert::error(body, on_dismiss) -> Container
alert::with_title(severity: alert::Severity, title: impl Into<String>, body: impl Into<String>, on_dismiss: Option<Message>) -> Container
alert::severity_color(theme: &RustTheme, severity: alert::Severity) -> ColorSet
```

### TextField

```rust
text_field::outlined(placeholder: &str, value: &str, on_input: impl Fn(String) -> Message) -> TextInput
text_field::filled(placeholder, value, on_input) -> TextInput
```

### Autocomplete

Caller supplies pre-filtered `suggestions` (matching logic is left to your
`update` function — keeps this component generic).

```rust
autocomplete::autocomplete(
    placeholder: &str,
    query: &str,
    suggestions: &[String],
    on_input: impl Fn(String) -> Message,
    on_select: impl Fn(usize) -> Message,
) -> Column
```

### Checkbox

```rust
checkbox::checkbox(label: impl Into<String>, is_checked: bool, on_toggle: impl Fn(bool) -> Message) -> Checkbox
```

### Radio

```rust
radio::radio<V: Copy + Eq>(label: impl Into<String>, value: V, selected: Option<V>, on_click: impl FnOnce(V) -> Message) -> Radio
```

### Switch

```rust
switch::switch(is_toggled: bool, on_toggle: impl Fn(bool) -> Message) -> Toggler
// chain .label("text") to add a text label
```

### Slider

```rust
slider::slider(range: RangeInclusive<f32>, value: f32, on_change: impl Fn(f32) -> Message) -> Slider<f32, Message, RustTheme>
```

### Select

Backed by iced's `PickList`.

```rust
select::select<T: ToString + PartialEq + Clone>(options: impl Borrow<[T]>, selected: Option<T>, on_selected: impl Fn(T) -> Message) -> PickList
```

### Segmented

```rust
segmented::segmented(labels: &[&str], selected: usize, on_select: impl Fn(usize) -> Message) -> Container
```

### Rating

```rust
rating::rating(value: u8, max: u8, on_rate: impl Fn(u8) -> Message) -> Row   // stars <= value render filled
```

### List

```rust
list::item(content: impl Into<Element>) -> Container
list::list(items: Vec<Element>) -> Column       // items separated by dividers
```

### Table

```rust
table::table(headers: &[&str], rows: &[Vec<String>]) -> Column   // striped body rows
```

### Tabs

Rounded "pill card" tabs (not outlined/underlined) inside a subtly-tinted container.

```rust
tabs::tabs(items: &[&str], selected: usize, on_select: impl Fn(usize) -> Message) -> Column
```

### BottomNavigation

```rust
bottom_navigation::BottomNavigationItem<'a> { label: &'a str, icon: icons::Icon }

bottom_navigation::bottom_navigation(items: &[BottomNavigationItem], selected: usize, on_select: impl Fn(usize) -> Message) -> Container
```

### Breadcrumbs

```rust
breadcrumbs::breadcrumbs(items: &[&str], on_select: impl Fn(usize) -> Message) -> Row  // last segment is non-interactive
```

### Pagination

```rust
pagination::pagination(page: usize /* 1-based */, total: usize, on_select: impl Fn(usize) -> Message) -> Row
```

### Stepper

```rust
stepper::stepper(labels: &[&str], active: usize /* 0-based */) -> Row   // steps before `active` render completed
```

### Accordion

```rust
accordion::accordion(title: impl Into<String>, body: impl Into<Element>, is_expanded: bool, on_toggle: Message) -> Column
```

### Timeline

```rust
timeline::TimelineItem { title: String, body: String }
timeline::TimelineItem::new(title: impl Into<String>, body: impl Into<String>) -> Self

timeline::timeline(items: Vec<TimelineItem>) -> Column
```

### Dialog

Modal overlay; wraps your whole page content and conditionally shows on top
via `stack`.

```rust
dialog::dialog(
    base_content: impl Into<Element>,   // your entire page/view
    is_open: bool,
    title: impl Into<String>,
    body: impl Into<Element>,
    actions: Vec<Element>,               // e.g. [Cancel button, Confirm button]
) -> Element
```

### Drawer

```rust
drawer::Anchor // enum: Left | Right

drawer::drawer(base_content: impl Into<Element>, is_open: bool, anchor: drawer::Anchor, content: impl Into<Element>, on_dismiss: Message) -> Element
```

### Snackbar

Always renders on an inverted high-contrast surface (readable in both light
and dark mode regardless of the active theme).

```rust
snackbar::snackbar(base_content: impl Into<Element>, is_visible: bool, message: impl Into<String>, on_dismiss: Message) -> Element
```

### Menu

```rust
menu::menu(base_content: impl Into<Element>, is_open: bool, items: &[&str], on_select: impl Fn(usize) -> Message, on_dismiss: Message) -> Element
```

### Backdrop

```rust
backdrop::scrim() -> Element                             // the dimming layer alone
backdrop::backdrop(base_content: impl Into<Element>, is_open: bool, content: Option<Element>) -> Element   // e.g. centered spinner
```

### SpeedDial

```rust
speed_dial::SpeedDialAction<Message> { glyph: &'static str, on_press: Message }
speed_dial::SpeedDialAction::new(glyph: &'static str, on_press: Message) -> Self

speed_dial::speed_dial(main_glyph: impl text::IntoFragment, is_open: bool, actions: Vec<SpeedDialAction<Message>>, on_toggle: Message) -> Column
```

### Fab

```rust
fab::primary(glyph: impl text::IntoFragment, on_press: Message) -> Button
fab::secondary(glyph, on_press) -> Button
fab::extended(glyph, label: impl Into<String>, on_press: Message) -> Button   // pill shape w/ label
```

### IconButton

All variants enforce a 44×44px minimum hit area.

```rust
icon_button::primary(glyph: impl text::IntoFragment, on_press: Message) -> Button   // neutral filled
icon_button::secondary(glyph, on_press) -> Button                                    // alias of primary
icon_button::outlined(glyph, on_press) -> Button                                     // bordered
icon_button::text_icon(glyph, on_press) -> Button                                     // borderless, lightest
icon_button::icon_primary(glyph, on_press) -> Button                                  // accent-filled, use once per surface
icon_button::destructive(glyph, on_press) -> Button                                   // error-red
```

### AppBar

```rust
app_bar::app_bar(title: impl Into<String>, actions: Vec<Element>) -> Container
```

### EmptyState

```rust
empty_state::empty_state(icon: icons::Icon, title: impl Into<String>, description: impl Into<String>, action: Option<(&str, Message)>) -> Container
```

### Tooltip

```rust
tooltip::tooltip(content: impl Into<Element>, label: impl text::IntoFragment) -> Tooltip   // shown above content on hover
```

### Skeleton

Static muted-fill placeholders (no shimmer animation — would require an
iced `Subscription`).

```rust
skeleton::text(width: f32) -> Container
skeleton::rectangular(width: f32, height: f32) -> Container
skeleton::circular(diameter: f32) -> Container
```

### Progress

```rust
progress::linear(value: f32 /* 0.0..=1.0 */) -> ProgressBar
progress::circular(theme: &RustTheme, value: f32, diameter: f32) -> Element
```

### Divider

```rust
divider::horizontal(height: impl Into<Pixels>) -> Rule
divider::vertical(width: impl Into<Pixels>) -> Rule
```

### Layout

Pure spacing helpers — wrap iced's `row`/`column` with theme-consistent gaps.

```rust
layout::Direction // enum: Row | Column

layout::stack(theme: &RustTheme, direction: layout::Direction, spacing_factor: f32, children: Vec<Element>) -> Element
layout::grid(theme: &RustTheme, columns: usize, spacing_factor: f32, children: Vec<Element>) -> Column
```

`spacing_factor` is multiplied by `theme.spacing`'s base unit (default 8px).

### Icons

Material Symbols Rounded, bundled — requires `rustheme::load_fonts!` in
`main()` (see [Fonts](#fonts)).

```rust
icons::Icon // enum, one variant per bundled glyph: Search, Home, Settings, Person, Close,
            // Check, Add, Remove, Delete, Edit, Favorite, FavoriteBorder, Mail, Notifications,
            // Menu, ArrowBack, ArrowForward, ChevronRight, ChevronLeft, ExpandMore, ExpandLess,
            // Refresh, Save, Cancel, Done, Info, Warning, Error, Visibility, VisibilityOff,
            // Star, StarBorder, MoreVert, MoreHoriz, Share, ContentCopy, Download, Upload,
            // Lock, Help

icons::font() -> iced::Font                              // for manual .font(...) calls
icons::icon(icon: icons::Icon, size: f32) -> Text          // ready-to-render sized glyph
icons::glyph(icon: icons::Icon) -> &'static str            // raw char, for button/fab glyph params
```

To add a new icon: look up its codepoint at fonts.google.com/icons and add
an `Icon` variant.

---

## Composite patterns

Everything above this section is a single-widget primitive. The components
below are **pre-assembled combinations of those primitives** — a settings
row, a search bar, a stat card, etc. — for the screens every app rebuilds by
hand. Reach for these first when the shape of what you're building matches
one; drop down to the primitives above when it doesn't.

### SearchBar

```rust
search_bar::search_bar(
    placeholder: &str,
    value: &str,
    on_input: impl Fn(String) -> Message,
    on_clear: Option<Message>,   // shows a trailing "×" button when Some and value is non-empty
) -> Container
```

### SectionHeader

```rust
section_header::section_header(title: impl Into<String>) -> Column   // title only

section_header::section_header_with(
    title: impl Into<String>,
    subtitle: Option<impl Into<String>>,
    actions: Vec<Element>,   // right-aligned on the title row (e.g. a "See all" button)
) -> Column
```

### SettingsRow

```rust
settings_row::settings_row(
    icon: icons::Icon,
    title: impl Into<String>,
    subtitle: Option<impl Into<String>>,
    trailing: impl Into<Element>,   // e.g. switch::switch(...), icons::icon(Icon::ChevronRight, 18.0)
) -> Container

settings_row::settings_row_compact(   // same, no leading icon column
    title: impl Into<String>,
    subtitle: Option<impl Into<String>>,
    trailing: impl Into<Element>,
) -> Container
```

Stack rows inside `list::list(vec![...])` for dividers between them, or wrap
a group in `card::card(...)`.

### StatCard

```rust
stat_card::Trend // enum: Up (success tint) | Down (error tint) | Neutral (secondary text tint)

stat_card::stat_card(label: impl Into<String>, value: impl Into<String>) -> Container

stat_card::with_trend(
    label: impl Into<String>,
    value: impl Into<String>,
    trend_label: impl Into<String>,   // e.g. "+12.4% vs last month"
    trend: stat_card::Trend,
) -> Container
```

### Toolbar

```rust
toolbar::toolbar(
    leading: impl Into<Element>,     // e.g. a title or selection count
    trailing: Vec<Element>,           // right-packed action buttons
) -> Container
```

### ConfirmDialog

Wraps `dialog::dialog` with the standard Cancel + confirm button row
pre-wired, so a yes/no prompt is one call.

```rust
confirm_dialog::confirm_dialog(
    base_content: impl Into<Element>,   // your entire page/view
    is_open: bool,
    title: impl Into<String>,
    message: impl text::IntoFragment,
    confirm_label: impl text::IntoFragment,
    on_cancel: Message,
    on_confirm: Message,
) -> Element   // confirm button renders as button::primary

confirm_dialog::destructive(/* same signature */) -> Element   // confirm button renders as button::destructive
```

### Toast

Multi-item, auto-stacking notification queue — the upgrade to `snackbar`
(which shows only one message at a time) for screens where several
notifications can be in flight at once.

```rust
toast::Toast { id: u64, severity: alert::Severity, message: String }
toast::Toast::new(id: u64, severity: alert::Severity, message: impl Into<String>) -> Self

toast::Position // enum: TopRight | BottomRight (default) | BottomLeft | TopLeft

toast::toast_stack(
    base_content: impl Into<Element>,   // your entire page/view
    toasts: &[toast::Toast],             // keep this list in your app state; push/retain to add/dismiss
    position: toast::Position,
    on_dismiss: impl Fn(u64) -> Message,  // maps a clicked toast's id to a dismiss Message
) -> Element
```

### ContextMenu

Right-click-triggered popover menu, anchored to the wrapped `target`'s
corner (not the literal cursor position — iced 0.13 doesn't expose click
coordinates to a plain component).

```rust
context_menu::context_menu(
    target: impl Into<Element>,
    is_open: bool,
    items: &[&str],
    on_open: Message,                       // fires on right-press of target
    on_select: impl Fn(usize) -> Message,
    on_dismiss: Message,                     // fires when the scrim is clicked
) -> Element
```

### SidebarNav

A persistent left-hand navigation column with a collapse toggle — the
desktop-app equivalent of `bottom_navigation`. Unlike `drawer` (a transient
overlay), this renders inline as a fixed layout column beside your page
content.

```rust
sidebar_nav::SidebarNavItem<'a> { label: &'a str, icon: icons::Icon }

sidebar_nav::sidebar_nav(
    items: &[SidebarNavItem],
    selected: usize,
    collapsed: bool,        // true = 56px icon-only rail, false = 220px icon+label
    on_select: impl Fn(usize) -> Message,
    on_toggle_collapse: Message,
) -> Container
```

### KeyValueList

Two-column label/value detail list (e.g. an order summary), distinct from
`table::table` which is for multi-row tabular data rather than one record's
properties.

```rust
key_value_list::KeyValue { label: String, value: String }
key_value_list::KeyValue::new(label: impl Into<String>, value: impl Into<String>) -> Self

key_value_list::key_value_list(pairs: Vec<KeyValue>) -> Column
```

### DataCardGrid

Wraps `layout::grid` with loading (skeleton placeholders) and empty-state
handling baked in, so a typical "fetch a list, render cards" screen doesn't
need to hand-roll those branches.

```rust
data_card_grid::data_card_grid(
    theme: &RustTheme,
    columns: usize,
    items: Vec<Element>,
    loading: bool,               // when true, renders columns * skeleton_rows skeleton cards instead of items
    skeleton_rows: usize,
    empty_icon: icons::Icon,     // shown when !loading && items.is_empty()
    empty_title: impl Into<String>,
    empty_description: impl Into<String>,
) -> Element
```

### ComparisonTable

Pricing/feature comparison grid: plan headers across the top (one
optionally highlighted as recommended), feature rows down the side.

```rust
comparison_table::Plan { name: String, price: String, highlighted: bool }
comparison_table::Plan::new(name: impl Into<String>, price: impl Into<String>, highlighted: bool) -> Self

comparison_table::Cell // enum: Included (green check) | Excluded (muted dash) | Text(String)

comparison_table::FeatureRow { label: String, cells: Vec<Cell> }
comparison_table::FeatureRow::new(label: impl Into<String>, cells: Vec<Cell>) -> Self

comparison_table::comparison_table(plans: &[Plan], rows: &[FeatureRow]) -> Column
```

---

## Full example: configurable app with light/dark toggle

```rust
use rustheme::prelude::*;
use rustheme_core::hex;

struct App {
    dark_mode: bool,
    config: RustThemeConfig,
}

impl Default for App {
    fn default() -> Self {
        Self {
            dark_mode: false,
            config: RustThemeConfig::new()
                .radius(10.0)
                .primary_color(hex("#3F51B5"))
                .dark_primary_color(hex("#B7C4FF")),
        }
    }
}

impl App {
    fn theme(&self) -> RustTheme {
        if self.dark_mode { self.config.dark() } else { self.config.light() }
    }

    fn view(&self) -> Element<'_, Message, RustTheme> {
        card::card(
            iced::widget::column![
                button::primary("Save", Message::Save),
                button::outline("Cancel", Message::Cancel),
            ]
            .spacing(8),
        )
        .into()
    }
}
```
