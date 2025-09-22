//! Visual feedback state management for nodes

/// Visual feedback state for nodes
#[derive(Debug, Clone, PartialEq, Default)]
pub struct VisualFeedback {
    /// Node is currently selected
    selected: bool,
    /// Node is currently hovered
    hovered: bool,
    /// Node is highlighted (e.g., during connection)
    highlighted: bool,
    /// Animation state for transitions
    animation_progress: f64,
}

impl VisualFeedback {
    /// Create new visual feedback with default state
    pub fn new() -> Self {
        Self::default()
    }

    /// Create visual feedback with selection state
    pub fn selected() -> Self {
        Self {
            selected: true,
            ..Default::default()
        }
    }

    /// Check if node is selected
    pub fn is_selected(&self) -> bool {
        self.selected
    }

    /// Check if node is hovered
    pub fn is_hovered(&self) -> bool {
        self.hovered
    }

    /// Check if node is highlighted
    pub fn is_highlighted(&self) -> bool {
        self.highlighted
    }

    /// Get animation progress (0.0 to 1.0)
    pub fn animation_progress(&self) -> f64 {
        self.animation_progress
    }

    /// Set selection state
    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    /// Set hover state
    pub fn set_hovered(&mut self, hovered: bool) {
        self.hovered = hovered;
    }

    /// Set highlight state
    pub fn set_highlighted(&mut self, highlighted: bool) {
        self.highlighted = highlighted;
    }

    /// Set animation progress
    pub fn set_animation_progress(&mut self, progress: f64) {
        self.animation_progress = progress.clamp(0.0, 1.0);
    }

    /// Check if any visual state is active
    pub fn has_any_state(&self) -> bool {
        self.selected || self.hovered || self.highlighted
    }
}
