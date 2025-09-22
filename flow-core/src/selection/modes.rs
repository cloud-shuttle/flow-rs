//! Selection modes and related enums

/// Selection modes for different interaction patterns
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SelectionMode {
    /// Single node selection (default)
    #[default]
    Single,
    /// Multi-node selection with Ctrl+Click
    Multi,
    /// Rectangle selection drag
    Rectangle,
}

/// Direction for keyboard navigation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NavigationDirection {
    /// Move to next node
    Next,
    /// Move to previous node
    Previous,
}

/// Keyboard shortcuts for selection management
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyboardShortcut {
    /// Select all nodes (Ctrl+A)
    SelectAll,
    /// Clear selection (Escape)
    Escape,
    /// Navigate right (Right Arrow)
    ArrowRight,
    /// Navigate left (Left Arrow)
    ArrowLeft,
    /// Navigate up (Up Arrow)
    ArrowUp,
    /// Navigate down (Down Arrow)
    ArrowDown,
    /// Delete selected nodes (Delete)
    Delete,
}
