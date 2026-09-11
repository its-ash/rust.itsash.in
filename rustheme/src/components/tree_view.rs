//! `TreeView`: an expandable/collapsible file/folder tree — the shared
//! navigation primitive behind an Editor's file explorer, a File Server's
//! remote listing, or a System Resource Explorer's process tree.
//!
//! Fully controlled: the caller owns the tree data (as a flat `Vec<Node>`,
//! not a recursive struct — recursive ownership doesn't compose well with
//! iced's borrow-per-frame `view()`) plus which node ids are expanded and
//! which is selected. [`flatten`] walks the tree once per `view()` call to
//! produce the visible (respecting collapsed ancestors), indented row list.

use crate::components::icons::{self, Icon};
use crate::theme::RustTheme;
use iced::widget::{button, column, row, text};
use iced::{Background, Border, Element, Length, Shadow};
use rustheme_core::{system_font, FontWeight};
use std::collections::HashSet;

/// One entry in a tree, addressed by a caller-assigned `id` (stable across
/// re-renders, used for expansion/selection tracking — an index into the
/// backing `Vec` works fine if the tree is static for the session).
#[derive(Debug, Clone)]
pub struct Node {
    pub id: usize,
    pub parent: Option<usize>,
    pub label: String,
    pub kind: NodeKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    Folder,
    File,
}

impl Node {
    pub fn folder(id: usize, parent: Option<usize>, label: impl Into<String>) -> Self {
        Self { id, parent, label: label.into(), kind: NodeKind::Folder }
    }

    pub fn file(id: usize, parent: Option<usize>, label: impl Into<String>) -> Self {
        Self { id, parent, label: label.into(), kind: NodeKind::File }
    }
}

/// Renders `nodes` as an indented tree. `expanded` holds the ids of
/// currently-open folders (a folder not in the set renders collapsed, its
/// children hidden); `selected` highlights one row. `on_toggle` fires when
/// a folder's disclosure arrow (or the folder row itself) is pressed;
/// `on_select` fires when any row (folder or file) is pressed, before the
/// implicit toggle — wire both to distinguish "select" from "expand" if the
/// caller wants them decoupled, or route both to the same message.
pub fn tree_view<'a, Message>(
    nodes: &'a [Node],
    expanded: &HashSet<usize>,
    selected: Option<usize>,
    on_toggle: impl Fn(usize) -> Message + 'a,
    on_select: impl Fn(usize) -> Message + 'a,
) -> Element<'a, Message, RustTheme>
where
    Message: Clone + 'a,
{
    let visible = flatten(nodes, expanded);

    column(visible.into_iter().map(|(node, depth)| {
        let is_folder = node.kind == NodeKind::Folder;
        let is_open = expanded.contains(&node.id);
        let is_selected = selected == Some(node.id);

        let disclosure: Element<'a, Message, RustTheme> = if is_folder {
            icons::icon(if is_open { Icon::ExpandMore } else { Icon::ChevronRight }, 16.0).into()
        } else {
            iced::widget::Space::new(Length::Fixed(16.0), Length::Fixed(16.0)).into()
        };

        let type_icon = if is_folder {
            icons::icon(if is_open { Icon::FolderOpen } else { Icon::Folder }, 17.0)
        } else {
            icons::icon(Icon::Description, 17.0)
        };

        let label = text(node.label.clone()).size(13).font(system_font(FontWeight::Regular));

        let content = row![disclosure, type_icon, label]
            .spacing(6)
            .align_y(iced::alignment::Vertical::Center);

        let id = node.id;
        let toggle_msg = on_toggle(id);
        let select_msg = on_select(id);

        button(row![
            iced::widget::Space::new(Length::Fixed(depth as f32 * 16.0), Length::Shrink),
            content,
        ])
        .padding([6, 8])
        .width(Length::Fill)
        .on_press(if is_folder { toggle_msg } else { select_msg })
        .style(move |theme: &RustTheme, status| {
            let p = &theme.palette;
            let background = if is_selected {
                Some(Background::Color(iced::Color { a: 0.16, ..p.primary.main }))
            } else if matches!(status, iced::widget::button::Status::Hovered) {
                Some(Background::Color(iced::Color { a: 0.06, ..p.primary.main }))
            } else {
                None
            };
            button::Style {
                background,
                text_color: if is_selected { p.primary.main } else { p.text.primary },
                border: Border { color: iced::Color::TRANSPARENT, width: 0.0, radius: theme.shape.small.into() },
                shadow: Shadow::default(),
            }
        })
        .into()
    }))
    .spacing(1)
    .into()
}

/// Walks `nodes` depth-first from the roots (entries with `parent: None`),
/// skipping the children of any folder not present in `expanded`, and
/// returns the visible rows paired with their indent depth.
fn flatten<'a>(nodes: &'a [Node], expanded: &HashSet<usize>) -> Vec<(&'a Node, usize)> {
    fn walk<'a>(
        nodes: &'a [Node],
        expanded: &HashSet<usize>,
        parent: Option<usize>,
        depth: usize,
        out: &mut Vec<(&'a Node, usize)>,
    ) {
        for node in nodes.iter().filter(|n| n.parent == parent) {
            out.push((node, depth));
            if node.kind == NodeKind::Folder && expanded.contains(&node.id) {
                walk(nodes, expanded, Some(node.id), depth + 1, out);
            }
        }
    }

    let mut out = Vec::new();
    walk(nodes, expanded, None, 0, &mut out);
    out
}
