//! Integration between DOM rect access and mouse event handling

use wasm_bindgen::JsCast;
use web_sys::{Element, MouseEvent};

use crate::dom_rect::{CoordinateConverter, DomRectUtils, ElementRect};
use flow_rs_core::{Position, Viewport};

/// Mouse event coordinate converter that integrates DOM rect access with viewport transformations
pub struct MouseEventConverter {
    rect_utils: DomRectUtils,
    element_rect: Option<ElementRect>,
}

impl MouseEventConverter {
    /// Create a new mouse event converter
    pub fn new() -> Self {
        Self {
            rect_utils: DomRectUtils::new(),
            element_rect: None,
        }
    }

    /// Initialize with a canvas element
    pub fn with_canvas(
        &mut self,
        canvas_element: &Element,
    ) -> Result<(), crate::dom_rect::DomRectError> {
        self.element_rect = Some(self.rect_utils.get_bounding_client_rect(canvas_element)?);
        Ok(())
    }

    /// Convert mouse event to canvas coordinates
    pub fn mouse_to_canvas(
        &self,
        event: &MouseEvent,
    ) -> Result<Position, crate::dom_rect::DomRectError> {
        let element_rect =
            self.element_rect
                .as_ref()
                .ok_or(crate::dom_rect::DomRectError::DomAccessFailed(
                    "Canvas element not initialized".to_string(),
                ))?;

        Ok(Position::new(
            event.client_x() as f64 - element_rect.x,
            event.client_y() as f64 - element_rect.y,
        ))
    }

    /// Convert mouse event to world coordinates using viewport
    pub fn mouse_to_world(
        &self,
        event: &MouseEvent,
        viewport: &Viewport,
    ) -> Result<Position, crate::dom_rect::DomRectError> {
        let canvas_pos = self.mouse_to_canvas(event)?;
        self.canvas_to_world(canvas_pos, viewport)
    }

    /// Convert canvas coordinates to world coordinates
    pub fn canvas_to_world(
        &self,
        canvas_pos: Position,
        _viewport: &Viewport,
    ) -> Result<Position, crate::dom_rect::DomRectError> {
        let element_rect =
            self.element_rect
                .as_ref()
                .ok_or(crate::dom_rect::DomRectError::DomAccessFailed(
                    "Canvas element not initialized".to_string(),
                ))?;

        let converter = CoordinateConverter::new(element_rect.clone());
        Ok(converter.element_to_viewport(canvas_pos))
    }

    /// Convert world coordinates to canvas coordinates
    pub fn world_to_canvas(
        &self,
        world_pos: Position,
        _viewport: &Viewport,
    ) -> Result<Position, crate::dom_rect::DomRectError> {
        let element_rect =
            self.element_rect
                .as_ref()
                .ok_or(crate::dom_rect::DomRectError::DomAccessFailed(
                    "Canvas element not initialized".to_string(),
                ))?;

        let converter = CoordinateConverter::new(element_rect.clone());
        Ok(converter.viewport_to_element(world_pos))
    }

    /// Get the current canvas element rect
    pub fn get_canvas_rect(&self) -> Option<&ElementRect> {
        self.element_rect.as_ref()
    }

    /// Update canvas element rect (useful for resize events)
    pub fn update_canvas(
        &mut self,
        canvas_element: &Element,
    ) -> Result<(), crate::dom_rect::DomRectError> {
        self.element_rect = Some(self.rect_utils.get_bounding_client_rect(canvas_element)?);
        Ok(())
    }
}

/// Utility functions for mouse event handling
pub mod utils {
    use super::*;
    use crate::events::utils;

    /// Convert mouse event to canvas coordinates using DOM rect
    pub fn mouse_event_to_canvas_coords(
        event: &MouseEvent,
        canvas_element: &Element,
        rect_utils: &mut DomRectUtils,
    ) -> Result<Position, crate::dom_rect::DomRectError> {
        let canvas_rect = rect_utils.get_bounding_client_rect(canvas_element)?;
        Ok(Position::new(
            event.client_x() as f64 - canvas_rect.x,
            event.client_y() as f64 - canvas_rect.y,
        ))
    }

    /// Convert mouse event to world coordinates using DOM rect and viewport
    pub fn mouse_event_to_world_coords(
        event: &MouseEvent,
        canvas_element: &Element,
        rect_utils: &mut DomRectUtils,
        viewport: &Viewport,
    ) -> Result<Position, crate::dom_rect::DomRectError> {
        let canvas_pos = mouse_event_to_canvas_coords(event, canvas_element, rect_utils)?;
        Ok(utils::canvas_to_world(
            canvas_pos,
            viewport.offset,
            viewport.zoom,
        ))
    }

    /// Convert mouse event to world coordinates with scroll offset
    pub fn mouse_event_to_world_coords_with_scroll(
        event: &MouseEvent,
        canvas_element: &Element,
        rect_utils: &mut DomRectUtils,
        viewport: &Viewport,
        scroll_offset: Position,
    ) -> Result<Position, crate::dom_rect::DomRectError> {
        let canvas_pos = mouse_event_to_canvas_coords(event, canvas_element, rect_utils)?;
        let converter = CoordinateConverter::with_scroll(
            rect_utils.get_bounding_client_rect(canvas_element)?,
            scroll_offset,
        );
        Ok(converter.element_to_viewport(canvas_pos))
    }

    /// Convert mouse event to world coordinates with zoom
    pub fn mouse_event_to_world_coords_with_zoom(
        event: &MouseEvent,
        canvas_element: &Element,
        rect_utils: &mut DomRectUtils,
        viewport: &Viewport,
        zoom_factor: f64,
    ) -> Result<Position, crate::dom_rect::DomRectError> {
        let canvas_pos = mouse_event_to_canvas_coords(event, canvas_element, rect_utils)?;
        let converter = CoordinateConverter::with_zoom(
            rect_utils.get_bounding_client_rect(canvas_element)?,
            zoom_factor,
        );
        Ok(converter.element_to_viewport(canvas_pos))
    }

    /// Check if mouse event is within canvas bounds
    pub fn is_mouse_in_canvas(
        event: &MouseEvent,
        canvas_element: &Element,
        rect_utils: &mut DomRectUtils,
    ) -> Result<bool, crate::dom_rect::DomRectError> {
        let canvas_rect = rect_utils.get_bounding_client_rect(canvas_element)?;
        let mouse_x = event.client_x() as f64;
        let mouse_y = event.client_y() as f64;

        Ok(mouse_x >= canvas_rect.x
            && mouse_x <= canvas_rect.x + canvas_rect.width
            && mouse_y >= canvas_rect.y
            && mouse_y <= canvas_rect.y + canvas_rect.height)
    }

    /// Get mouse position relative to canvas center
    pub fn mouse_relative_to_canvas_center(
        event: &MouseEvent,
        canvas_element: &Element,
        rect_utils: &mut DomRectUtils,
    ) -> Result<Position, crate::dom_rect::DomRectError> {
        let canvas_rect = rect_utils.get_bounding_client_rect(canvas_element)?;
        let canvas_center_x = canvas_rect.x + canvas_rect.width / 2.0;
        let canvas_center_y = canvas_rect.y + canvas_rect.height / 2.0;

        Ok(Position::new(
            event.client_x() as f64 - canvas_center_x,
            event.client_y() as f64 - canvas_center_y,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use js_sys::Object;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    fn create_mock_mouse_event(client_x: f64, client_y: f64) -> MouseEvent {
        let event = Object::new();
        js_sys::Reflect::set(&event, &"clientX".into(), &client_x.into()).unwrap();
        js_sys::Reflect::set(&event, &"clientY".into(), &client_y.into()).unwrap();
        js_sys::Reflect::set(
            &event,
            &"preventDefault".into(),
            &js_sys::Function::new_no_args(""),
        )
        .unwrap();
        event.unchecked_into()
    }

    fn create_mock_canvas_element(x: f64, y: f64, width: f64, height: f64) -> Element {
        let element = Object::new();
        js_sys::Reflect::set(&element, &"getBoundingClientRect".into(), &js_sys::Function::new_no_args(&format!(
            "return {{x: {}, y: {}, width: {}, height: {}, top: {}, right: {}, bottom: {}, left: {}}}",
            x, y, width, height, y, x + width, y + height, x
        ))).unwrap();
        element.unchecked_into()
    }

    #[wasm_bindgen_test]
    fn test_mouse_event_converter_creation() {
        let converter = MouseEventConverter::new();
        assert!(converter.get_canvas_rect().is_none());
    }

    #[wasm_bindgen_test]
    fn test_mouse_event_converter_with_canvas() {
        let mut converter = MouseEventConverter::new();
        let canvas_element = create_mock_canvas_element(100.0, 200.0, 800.0, 600.0);

        let result = converter.with_canvas(&canvas_element);
        assert!(result.is_ok());
        assert!(converter.get_canvas_rect().is_some());
    }

    #[wasm_bindgen_test]
    fn test_mouse_to_canvas_conversion() {
        let mut converter = MouseEventConverter::new();
        let canvas_element = create_mock_canvas_element(100.0, 200.0, 800.0, 600.0);
        let mouse_event = create_mock_mouse_event(250.0, 350.0);

        converter.with_canvas(&canvas_element).unwrap();
        let canvas_pos = converter.mouse_to_canvas(&mouse_event).unwrap();

        assert_eq!(canvas_pos.x, 150.0); // 250 - 100
        assert_eq!(canvas_pos.y, 150.0); // 350 - 200
    }

    #[wasm_bindgen_test]
    fn test_mouse_to_world_conversion() {
        let mut converter = MouseEventConverter::new();
        let canvas_element = create_mock_canvas_element(0.0, 0.0, 800.0, 600.0);
        let mouse_event = create_mock_mouse_event(400.0, 300.0);
        let viewport = Viewport::new(100.0, 50.0, 0.0, 0.0, 2.0);

        converter.with_canvas(&canvas_element).unwrap();
        let world_pos = converter.mouse_to_world(&mouse_event, &viewport).unwrap();

        // Expected: (400 - 100) / 2.0 + 100 = 250, (300 - 50) / 2.0 + 50 = 175
        assert_eq!(world_pos.x, 250.0);
        assert_eq!(world_pos.y, 175.0);
    }

    #[wasm_bindgen_test]
    fn test_utils_mouse_event_to_canvas_coords() {
        let canvas_element = create_mock_canvas_element(100.0, 200.0, 800.0, 600.0);
        let mouse_event = create_mock_mouse_event(250.0, 350.0);
        let mut rect_utils = DomRectUtils::new();

        let canvas_pos =
            utils::mouse_event_to_canvas_coords(&mouse_event, &canvas_element, &mut rect_utils)
                .unwrap();

        assert_eq!(canvas_pos.x, 150.0);
        assert_eq!(canvas_pos.y, 150.0);
    }

    #[wasm_bindgen_test]
    fn test_utils_mouse_event_to_world_coords() {
        let canvas_element = create_mock_canvas_element(0.0, 0.0, 800.0, 600.0);
        let mouse_event = create_mock_mouse_event(400.0, 300.0);
        let mut rect_utils = DomRectUtils::new();
        let viewport = Viewport::new(100.0, 50.0, 0.0, 0.0, 2.0);

        let world_pos = utils::mouse_event_to_world_coords(
            &mouse_event,
            &canvas_element,
            &mut rect_utils,
            &viewport,
        )
        .unwrap();

        // Expected: (400 / 2.0) + 100 = 300, (300 / 2.0) + 50 = 200
        assert_eq!(world_pos.x, 300.0);
        assert_eq!(world_pos.y, 200.0);
    }

    #[wasm_bindgen_test]
    fn test_utils_is_mouse_in_canvas() {
        let canvas_element = create_mock_canvas_element(100.0, 200.0, 800.0, 600.0);
        let mut rect_utils = DomRectUtils::new();

        // Mouse inside canvas
        let inside_event = create_mock_mouse_event(500.0, 500.0);
        let inside_result =
            utils::is_mouse_in_canvas(&inside_event, &canvas_element, &mut rect_utils).unwrap();
        assert!(inside_result);

        // Mouse outside canvas
        let outside_event = create_mock_mouse_event(50.0, 150.0);
        let outside_result =
            utils::is_mouse_in_canvas(&outside_event, &canvas_element, &mut rect_utils).unwrap();
        assert!(!outside_result);
    }

    #[wasm_bindgen_test]
    fn test_utils_mouse_relative_to_canvas_center() {
        let canvas_element = create_mock_canvas_element(100.0, 200.0, 800.0, 600.0);
        let mouse_event = create_mock_mouse_event(500.0, 500.0); // Center of canvas
        let mut rect_utils = DomRectUtils::new();

        let relative_pos =
            utils::mouse_relative_to_canvas_center(&mouse_event, &canvas_element, &mut rect_utils)
                .unwrap();

        // Canvas center is at (100 + 400, 200 + 300) = (500, 500)
        // Mouse is at (500, 500), so relative position should be (0, 0)
        assert_eq!(relative_pos.x, 0.0);
        assert_eq!(relative_pos.y, 0.0);
    }
}
