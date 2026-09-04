/// Shape tokens controlling corner rounding across components.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shape {
    pub border_radius: f32,
}

impl Default for Shape {
    fn default() -> Self {
        Self { border_radius: 4.0 }
    }
}
