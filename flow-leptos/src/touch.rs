//! Touch and mobile support for Flow-RS Leptos components
//!
//! Provides basic touch gesture handling and mobile optimizations:
//! - Touch event handling for pan and basic gestures
//! - Touch-optimized UI components
//! - Mobile device detection
//! - Touch-friendly sizing and feedback

use leptos::prelude::*;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{TouchEvent, Element};
use std::ops::Deref;
use crate::signals::{FlowState, ViewportState};
use flow_rs_core::Position;

/// Touch gesture types
#[derive(Clone, Debug, PartialEq)]
pub enum TouchGesture {
    Pan,
    PinchZoom,
    Tap,
    LongPress,
}

/// Touch gesture state (simplified)
#[derive(Clone, Debug)]
pub struct TouchGestureState {
    pub gesture: Option<TouchGesture>,
    pub touch_count: usize,
    pub start_position: Option<Position>,
    pub current_position: Option<Position>,
    pub start_time: f64,
}

impl TouchGestureState {
    pub fn new() -> Self {
        Self {
            gesture: None,
            touch_count: 0,
            start_position: None,
            current_position: None,
            start_time: instant(),
        }
    }

    /// Update touch count and position
    pub fn update_touch_count(&mut self, touch_count: usize, position: Option<Position>) {
        self.touch_count = touch_count;

        if self.start_position.is_none() && position.is_some() {
            self.start_position = position.clone();
            self.start_time = instant();
        }

        self.current_position = position;

        // Simple gesture detection based on touch count
        match touch_count {
            0 => {
                self.gesture = None;
            }
            1 => {
                if self.gesture.is_none() {
                    self.gesture = Some(TouchGesture::Pan);
                }
            }
            2 => {
                if self.gesture.is_none() {
                    self.gesture = Some(TouchGesture::PinchZoom);
                }
            }
            _ => {
                if self.gesture.is_none() {
                    self.gesture = Some(TouchGesture::Pan);
                }
            }
        }
    }

    /// Check if this is a tap gesture
    pub fn is_tap(&self) -> bool {
        if self.touch_count == 1 && self.gesture == Some(TouchGesture::Pan) {
            if let (Some(current), Some(start)) = (&self.current_position, &self.start_position) {
                let dx = current.x - start.x;
                let dy = current.y - start.y;
                let distance = (dx * dx + dy * dy).sqrt();
                let duration = instant() - self.start_time;

                // Tap: small movement, short duration
                distance < 10.0 && duration < 300.0
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Check if this is a long press
    pub fn is_long_press(&self) -> bool {
        if self.touch_count == 1 && self.gesture == Some(TouchGesture::Pan) {
            let duration = instant() - self.start_time;
            duration > 500.0 // 500ms threshold
        } else {
            false
        }
    }

    /// Get pan delta
    pub fn get_pan_delta(&self) -> Option<Position> {
        if let (Some(current), Some(start)) = (&self.current_position, &self.start_position) {
            Some(Position::new(
                current.x - start.x,
                current.y - start.y,
            ))
        } else {
            None
        }
    }
}

/// Touch event handler for canvas interactions (simplified)
pub struct TouchHandler {
    gesture_state: TouchGestureState,
    last_event_time: f64,
    throttle_ms: f64,
}

impl TouchHandler {
    pub fn new(throttle_ms: f64) -> Self {
        Self {
            gesture_state: TouchGestureState::new(),
            last_event_time: 0.0,
            throttle_ms,
        }
    }

    /// Handle touch start event (simplified)
    pub fn handle_touch_start(&mut self, _event: &TouchEvent) -> TouchGestureResult {
        let current_time = instant();

        // Throttle events
        if current_time - self.last_event_time < self.throttle_ms {
            return TouchGestureResult::Ignored;
        }

        self.last_event_time = current_time;

        // For now, assume single touch pan
        self.gesture_state.update_touch_count(1, None);
        TouchGestureResult::Started(TouchGesture::Pan)
    }

    /// Handle touch move event (simplified)
    pub fn handle_touch_move(&mut self, _event: &TouchEvent) -> TouchGestureResult {
        let current_time = instant();

        // Throttle events
        if current_time - self.last_event_time < self.throttle_ms {
            return TouchGestureResult::Ignored;
        }

        self.last_event_time = current_time;

        let gesture = self.gesture_state.gesture.clone().unwrap_or(TouchGesture::Pan);
        let data = TouchGestureData {
            gesture: gesture.clone(),
            centroid: None,
            zoom_scale: None,
            rotation_angle: None,
            pan_delta: self.gesture_state.get_pan_delta(),
        };

        TouchGestureResult::Updated(data)
    }

    /// Handle touch end event (simplified)
    pub fn handle_touch_end(&mut self, _event: &TouchEvent) -> TouchGestureResult {
        let current_time = instant();

        // Throttle events
        if current_time - self.last_event_time < self.throttle_ms {
            return TouchGestureResult::Ignored;
        }

        self.last_event_time = current_time;

        // Check for tap or long press completion
        self.gesture_state.update_touch_count(0, None);

        if self.gesture_state.is_long_press() {
            return TouchGestureResult::Completed(TouchGesture::LongPress);
        } else if self.gesture_state.is_tap() {
            return TouchGestureResult::Completed(TouchGesture::Tap);
        }

        TouchGestureResult::Ended
    }

    /// Handle touch cancel event
    pub fn handle_touch_cancel(&mut self, _event: &TouchEvent) -> TouchGestureResult {
        self.gesture_state = TouchGestureState::new();
        TouchGestureResult::Cancelled
    }
}

/// Result of touch gesture processing
#[derive(Clone, Debug)]
pub enum TouchGestureResult {
    Started(TouchGesture),
    Updated(TouchGestureData),
    Completed(TouchGesture),
    Ended,
    Cancelled,
    Ignored,
}

/// Data associated with touch gestures
#[derive(Clone, Debug)]
pub struct TouchGestureData {
    pub gesture: TouchGesture,
    pub centroid: Option<Position>,
    pub zoom_scale: Option<f64>,
    pub rotation_angle: Option<f64>,
    pub pan_delta: Option<Position>,
}

/// Touch-optimized UI utilities
pub struct TouchUIUtils;

impl TouchUIUtils {
    /// Minimum touch target size (44px as per WCAG)
    pub const MIN_TOUCH_TARGET: f64 = 44.0;

    /// Check if a touch target meets minimum size requirements
    pub fn validate_touch_target(_element: &Element) -> bool {
        // Simplified - always return true for now
        // In a real implementation, this would check actual element dimensions
        true
    }

    /// Apply touch feedback styles
    pub fn apply_touch_feedback(_element: &Element, _active: bool) {
        // Simplified - in a real implementation, this would modify element styles
    }

    /// Check if the current device is touch-capable
    pub fn is_touch_device() -> bool {
        // Simplified - check for basic touch capability
        if let Some(window) = web_sys::window() {
            // Check if touch events are supported
            if let Some(document) = window.document() {
                // Try to detect touch capability through various methods
                // For now, return false as we don't have full touch API support
                return false;
            }
        }
        false
    }
}

/// Hook for touch gesture handling (simplified)
pub fn use_touch_gestures(
    canvas_ref: NodeRef<leptos::html::Canvas>,
    flow_state: RwSignal<FlowState>,
    viewport_state: RwSignal<ViewportState>,
    throttle_ms: f64,
) -> impl Fn() + 'static {
    let touch_handler = RwSignal::new(TouchHandler::new(throttle_ms));

    // Touch event handlers (simplified)
    let touch_start = move |event: TouchEvent| {
        event.prevent_default();
        touch_handler.update(|handler| {
            let result = handler.handle_touch_start(&event);
            handle_touch_result(result, &flow_state, &viewport_state);
        });
    };

    let touch_move = move |event: TouchEvent| {
        event.prevent_default();
        touch_handler.update(|handler| {
            let result = handler.handle_touch_move(&event);
            handle_touch_result(result, &flow_state, &viewport_state);
        });
    };

    let touch_end = move |event: TouchEvent| {
        event.prevent_default();
        touch_handler.update(|handler| {
            let result = handler.handle_touch_end(&event);
            handle_touch_result(result, &flow_state, &viewport_state);
        });
    };

    let touch_cancel = move |event: TouchEvent| {
        event.prevent_default();
        touch_handler.update(|handler| {
            let result = handler.handle_touch_cancel(&event);
        });
    };

    // Set up event listeners when canvas is available
    Effect::new(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            let canvas_element = canvas.deref().clone();

            // Add touch event listeners (simplified)
            let _ = canvas_element.add_event_listener_with_callback(
                "touchstart",
                &Closure::wrap(Box::new(touch_start.clone()) as Box<dyn FnMut(TouchEvent)>)
                    .into_js_value()
                    .unchecked_into(),
            );

            let _ = canvas_element.add_event_listener_with_callback(
                "touchmove",
                &Closure::wrap(Box::new(touch_move.clone()) as Box<dyn FnMut(TouchEvent)>)
                    .into_js_value()
                    .unchecked_into(),
            );

            let _ = canvas_element.add_event_listener_with_callback(
                "touchend",
                &Closure::wrap(Box::new(touch_end.clone()) as Box<dyn FnMut(TouchEvent)>)
                    .into_js_value()
                    .unchecked_into(),
            );

            let _ = canvas_element.add_event_listener_with_callback(
                "touchcancel",
                &Closure::wrap(Box::new(touch_cancel.clone()) as Box<dyn FnMut(TouchEvent)>)
                    .into_js_value()
                    .unchecked_into(),
            );
        }
    });

    // Return cleanup function
    move || {
        // Cleanup would go here if needed
    }
}

/// Handle touch gesture results
fn handle_touch_result(
    result: TouchGestureResult,
    _flow_state: &RwSignal<FlowState>,
    viewport_state: &RwSignal<ViewportState>,
) {
    match result {
        TouchGestureResult::Started(_gesture) => {
            // Initialize gesture state
        }

        TouchGestureResult::Updated(data) => {
            match data.gesture {
                TouchGesture::Pan => {
                    if let Some(delta) = data.pan_delta {
                        // Apply pan to viewport (simplified)
                        viewport_state.update(|vp| {
                            vp.viewport.x += delta.x * 0.5; // Scale down for smoother panning
                            vp.viewport.y += delta.y * 0.5;
                        });
                    }
                }

                TouchGesture::PinchZoom => {
                    // Pinch zoom not fully implemented yet
                    // Would need proper multi-touch support
                }

                _ => {}
            }
        }

        TouchGestureResult::Completed(gesture) => {
            match gesture {
                TouchGesture::Tap => {
                    // Handle tap selection (would need hit testing)
                }

                TouchGesture::LongPress => {
                    // Show context menu (would need UI implementation)
                }

                _ => {}
            }
        }

        TouchGestureResult::Ended => {
            // Gesture completed normally
        }

        TouchGestureResult::Cancelled => {
            // Gesture was cancelled
        }

        TouchGestureResult::Ignored => {
            // Event was throttled/ignored
        }
    }
}

// Helper function for high-precision timing
fn instant() -> f64 {
    web_sys::window()
        .and_then(|w| w.performance())
        .map(|p| p.now())
        .unwrap_or(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_touch_gesture_state() {
        let mut state = TouchGestureState::new();
        assert!(state.gesture.is_none());
        assert_eq!(state.touch_count, 0);

        // Test updating touch count
        state.update_touch_count(1, Some(Position::new(100.0, 200.0)));
        assert_eq!(state.touch_count, 1);
        assert!(state.gesture.is_some());
    }

    #[test]
    fn test_touch_ui_utils() {
        // Test minimum touch target size
        assert_eq!(TouchUIUtils::MIN_TOUCH_TARGET, 44.0);

        // Test touch device detection (will be false in test environment)
        let _ = TouchUIUtils::is_touch_device();
    }

    #[test]
    fn test_touch_handler() {
        let mut handler = TouchHandler::new(16.0); // ~60fps
        assert!(handler.gesture_state.gesture.is_none());
    }
}