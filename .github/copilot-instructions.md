# RustTheme

RustTheme is a MUI-inspired theming framework and component library for the
[Iced](https://iced.rs) GUI toolkit (Rust). It provides a design-token system
(palette, typography, spacing, shape), a `RustTheme` type that implements
iced 0.13's `Catalog` styling traits, and an ergonomic component library
(buttons, text fields, cards, dialogs, snackbars, etc.) pre-wired to named
style classes that mirror MUI's variant system.

## Workspace layout

```
rustheme/
  rustheme-core/        Design tokens only: Palette, Typography, Spacing,
                         Shape. No dependency on the full `iced` crate.
  rustheme/              The `RustTheme` type + iced `Catalog` impls
                         (src/style/) + the component library (src/components/).
  examples/gallery/      A runnable showcase app exercising every component.
```

`RustTheme` (the type widgets are styled against) lives in the `rustheme`
crate, not `rustheme-core`, because Rust's orphan rules require
`impl <iced trait> for RustTheme` to live in the same crate that defines
`RustTheme`. `rustheme-core` stays iced-agnostic and only exposes token
types and `iced_core::Color`.

## Adding a new component

1. **Style class** — add a named style function to `rustheme/src/style/<widget>.rs`
   (e.g. `pub fn primary() -> StyleFn<'static, RustTheme>`), returning a
   boxed `Fn(&RustTheme, Status) -> Style` (or `Fn(&RustTheme) -> Style` for
   status-less widgets). Read tokens off `theme.palette`, `theme.typography`,
   `theme.spacing`, `theme.shape` — never hardcode colors.
2. **Catalog impl** — if the widget doesn't have one yet, add
   `impl iced::widget::<widget>::Catalog for RustTheme` in
   `rustheme/src/style/mod.rs`, delegating `default()`/`style()` to your new
   style module.
3. **Component constructor** — add `rustheme/src/components/<name>.rs` with a
   free function returning the widget (or `Element`) pre-wired via
   `.style(|theme, status| crate::style::<widget>::<class>()(theme, status))`.
   Prefer multiple named variants matching MUI conventions (e.g.
   `primary`/`secondary`/`outlined`) over a single parameterized function.
4. **Re-export** — add the module to `rustheme/src/components/mod.rs` and to
   the flattened re-export list + `prelude` in `rustheme/src/lib.rs`.
5. **Gallery** — add a section to `examples/gallery/src/main.rs` demonstrating
   the new component's variants and any interactive state it needs.

Composite widgets with no native iced equivalent (Dialog, Snackbar, Menu,
AppBar, Tabs) are built by composing `container`, `row`, `column`, `stack`,
`mouse_area`, and existing components — see `rustheme/src/components/dialog.rs`
for the `stack`-based overlay pattern.

## Styling conventions

- Every color, radius, and spacing value styling code touches must come from
  `RustTheme`'s tokens (`palette`, `typography`, `spacing`, `shape`) — never
  a bare hex literal outside `rustheme-core/src/tokens/palette.rs`.
- Named style classes follow MUI's variant vocabulary where it applies:
  `primary`, `secondary`, `outlined`, `text`, `danger`/`error`, `success`.
- `RustTheme::light()` and `RustTheme::dark()` are the two supported theme
  modes; `Default` resolves to `light()`.

## Build & run

```
cargo build --workspace   # compile everything
cargo run -p gallery      # run the component showcase
make run / make build / make deploy
```
