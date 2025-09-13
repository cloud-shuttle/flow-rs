//! Comprehensive unit tests for core types
//!
//! This module tests all the fundamental types used throughout the Leptos Flow system,
//! including Position, Size, Rect, NodeId, EdgeId, and their operations.

use flow_core::types::*;

#[test]
fn test_position_creation() {
    let pos = Position::new(10.0, 20.0);
    assert_eq!(pos.x, 10.0);
    assert_eq!(pos.y, 20.0);
}

#[test]
fn test_position_zero() {
    let pos = Position::zero();
    assert_eq!(pos.x, 0.0);
    assert_eq!(pos.y, 0.0);
}

#[test]
fn test_position_distance() {
    let pos1 = Position::new(0.0, 0.0);
    let pos2 = Position::new(3.0, 4.0);

    assert_eq!(pos1.distance_to(pos2), 5.0);
    assert_eq!(pos2.distance_to(pos1), 5.0);
}

#[test]
fn test_position_distance_squared() {
    let pos1 = Position::new(0.0, 0.0);
    let pos2 = Position::new(3.0, 4.0);

    assert_eq!(pos1.distance_squared_to(pos2), 25.0);
    assert_eq!(pos2.distance_squared_to(pos1), 25.0);
}

#[test]
fn test_position_add() {
    let pos1 = Position::new(10.0, 20.0);
    let pos2 = Position::new(5.0, 15.0);
    let result = pos1.add(pos2);

    assert_eq!(result.x, 15.0);
    assert_eq!(result.y, 35.0);
}

#[test]
fn test_position_sub() {
    let pos1 = Position::new(10.0, 20.0);
    let pos2 = Position::new(5.0, 15.0);
    let result = pos1.sub(pos2);

    assert_eq!(result.x, 5.0);
    assert_eq!(result.y, 5.0);
}

#[test]
fn test_position_scale() {
    let pos = Position::new(10.0, 20.0);
    let scaled = pos.scale(2.0);

    assert_eq!(scaled.x, 20.0);
    assert_eq!(scaled.y, 40.0);
}

#[test]
fn test_position_normalize() {
    let pos = Position::new(3.0, 4.0);
    let normalized = pos.normalize();

    assert!((normalized.x - 0.6).abs() < 1e-10);
    assert!((normalized.y - 0.8).abs() < 1e-10);
}

#[test]
fn test_position_normalize_zero() {
    let pos = Position::zero();
    let normalized = pos.normalize();

    assert_eq!(normalized.x, 0.0);
    assert_eq!(normalized.y, 0.0);
}

#[test]
fn test_position_magnitude() {
    let pos = Position::new(3.0, 4.0);
    assert_eq!(pos.magnitude(), 5.0);

    let pos_zero = Position::zero();
    assert_eq!(pos_zero.magnitude(), 0.0);
}

#[test]
fn test_position_lerp() {
    let pos1 = Position::new(0.0, 0.0);
    let pos2 = Position::new(10.0, 20.0);

    let lerped = pos1.lerp(pos2, 0.5);
    assert_eq!(lerped.x, 5.0);
    assert_eq!(lerped.y, 10.0);

    let lerped_0 = pos1.lerp(pos2, 0.0);
    assert_eq!(lerped_0.x, 0.0);
    assert_eq!(lerped_0.y, 0.0);

    let lerped_1 = pos1.lerp(pos2, 1.0);
    assert_eq!(lerped_1.x, 10.0);
    assert_eq!(lerped_1.y, 20.0);
}

#[test]
fn test_size_creation() {
    let size = Size::new(100.0, 200.0);
    assert_eq!(size.width, 100.0);
    assert_eq!(size.height, 200.0);
}

#[test]
fn test_size_zero() {
    let size = Size::zero();
    assert_eq!(size.width, 0.0);
    assert_eq!(size.height, 0.0);
}

#[test]
fn test_size_area() {
    let size = Size::new(10.0, 20.0);
    assert_eq!(size.area(), 200.0);

    let size_zero = Size::zero();
    assert_eq!(size_zero.area(), 0.0);
}

#[test]
fn test_size_aspect_ratio() {
    let size = Size::new(16.0, 9.0);
    assert!((size.aspect_ratio() - 16.0/9.0).abs() < 1e-10);

    let size_square = Size::new(10.0, 10.0);
    assert_eq!(size_square.aspect_ratio(), 1.0);
}

#[test]
fn test_size_scale() {
    let size = Size::new(10.0, 20.0);
    let scaled = size.scale(2.0);

    assert_eq!(scaled.width, 20.0);
    assert_eq!(scaled.height, 40.0);
}

#[test]
fn test_size_contains() {
    let size = Size::new(100.0, 200.0);
    let pos_inside = Position::new(50.0, 100.0);
    let pos_outside = Position::new(150.0, 100.0);

    assert!(size.contains(pos_inside));
    assert!(!size.contains(pos_outside));
}

#[test]
fn test_rect_creation() {
    let rect = Rect::new(10.0, 20.0, 100.0, 200.0);
    assert_eq!(rect.x, 10.0);
    assert_eq!(rect.y, 20.0);
    assert_eq!(rect.width, 100.0);
    assert_eq!(rect.height, 200.0);
}

#[test]
fn test_rect_from_position_and_size() {
    let pos = Position::new(10.0, 20.0);
    let size = Size::new(100.0, 200.0);
    let rect = Rect::from_position_and_size(pos, size);

    assert_eq!(rect.x, 10.0);
    assert_eq!(rect.y, 20.0);
    assert_eq!(rect.width, 100.0);
    assert_eq!(rect.height, 200.0);
}

#[test]
fn test_rect_from_points() {
    let pos1 = Position::new(10.0, 20.0);
    let pos2 = Position::new(110.0, 220.0);
    let rect = Rect::from_points(pos1, pos2);

    assert_eq!(rect.x, 10.0);
    assert_eq!(rect.y, 20.0);
    assert_eq!(rect.width, 100.0);
    assert_eq!(rect.height, 200.0);
}

#[test]
fn test_rect_center() {
    let rect = Rect::new(0.0, 0.0, 100.0, 200.0);
    let center = rect.center();

    assert_eq!(center.x, 50.0);
    assert_eq!(center.y, 100.0);
}

#[test]
fn test_rect_top_left() {
    let rect = Rect::new(10.0, 20.0, 100.0, 200.0);
    let top_left = rect.top_left();

    assert_eq!(top_left.x, 10.0);
    assert_eq!(top_left.y, 20.0);
}

#[test]
fn test_rect_bottom_right() {
    let rect = Rect::new(10.0, 20.0, 100.0, 200.0);
    let bottom_right = rect.bottom_right();

    assert_eq!(bottom_right.x, 110.0);
    assert_eq!(bottom_right.y, 220.0);
}

#[test]
fn test_rect_contains_point() {
    let rect = Rect::new(10.0, 20.0, 100.0, 200.0);

    // Point inside
    let pos_inside = Position::new(50.0, 100.0);
    assert!(rect.contains_point(pos_inside));

    // Point outside
    let pos_outside = Position::new(150.0, 100.0);
    assert!(!rect.contains_point(pos_outside));

    // Point on edge
    let pos_edge = Position::new(10.0, 20.0);
    assert!(rect.contains_point(pos_edge));
}

#[test]
fn test_rect_intersects() {
    let rect1 = Rect::new(0.0, 0.0, 100.0, 100.0);
    let rect2 = Rect::new(50.0, 50.0, 100.0, 100.0);
    let rect3 = Rect::new(200.0, 200.0, 100.0, 100.0);

    assert!(rect1.intersects(rect2));
    assert!(rect2.intersects(rect1));
    assert!(!rect1.intersects(rect3));
    assert!(!rect3.intersects(rect1));
}

#[test]
fn test_rect_union() {
    let rect1 = Rect::new(0.0, 0.0, 100.0, 100.0);
    let rect2 = Rect::new(50.0, 50.0, 100.0, 100.0);
    let union = rect1.union(rect2);

    assert_eq!(union.x, 0.0);
    assert_eq!(union.y, 0.0);
    assert_eq!(union.width, 150.0);
    assert_eq!(union.height, 150.0);
}

#[test]
fn test_rect_expand() {
    let rect = Rect::new(10.0, 20.0, 100.0, 200.0);
    let expanded = rect.expand(5.0);

    assert_eq!(expanded.x, 5.0);
    assert_eq!(expanded.y, 15.0);
    assert_eq!(expanded.width, 110.0);
    assert_eq!(expanded.height, 210.0);
}

#[test]
fn test_rect_shrink() {
    let rect = Rect::new(10.0, 20.0, 100.0, 200.0);
    let shrunk = rect.shrink(5.0);

    assert_eq!(shrunk.x, 15.0);
    assert_eq!(shrunk.y, 25.0);
    assert_eq!(shrunk.width, 90.0);
    assert_eq!(shrunk.height, 190.0);
}

#[test]
fn test_node_id_creation() {
    let id = NodeId::new("test-node");
    assert_eq!(id.as_str(), "test-node");
}

#[test]
fn test_node_id_from_string() {
    let id = NodeId::from("test-node".to_string());
    assert_eq!(id.as_str(), "test-node");
}

#[test]
fn test_node_id_from_str() {
    let id = NodeId::from("test-node");
    assert_eq!(id.as_str(), "test-node");
}

#[test]
fn test_node_id_equality() {
    let id1 = NodeId::new("test-node");
    let id2 = NodeId::new("test-node");
    let id3 = NodeId::new("different-node");

    assert_eq!(id1, id2);
    assert_ne!(id1, id3);
}

#[test]
fn test_edge_id_creation() {
    let id = EdgeId::new("test-edge");
    assert_eq!(id.as_str(), "test-edge");
}

#[test]
fn test_edge_id_from_string() {
    let id = EdgeId::from("test-edge".to_string());
    assert_eq!(id.as_str(), "test-edge");
}

#[test]
fn test_edge_id_from_str() {
    let id = EdgeId::from("test-edge");
    assert_eq!(id.as_str(), "test-edge");
}

#[test]
fn test_edge_id_equality() {
    let id1 = EdgeId::new("test-edge");
    let id2 = EdgeId::new("test-edge");
    let id3 = EdgeId::new("different-edge");

    assert_eq!(id1, id2);
    assert_ne!(id1, id3);
}

#[test]
fn test_viewport_creation() {
    let viewport = Viewport::new(Position::new(100.0, 200.0), 1.5);
    assert_eq!(viewport.offset.x, 100.0);
    assert_eq!(viewport.offset.y, 200.0);
    assert_eq!(viewport.zoom, 1.5);
}

#[test]
fn test_viewport_default() {
    let viewport = Viewport::default();
    assert_eq!(viewport.offset.x, 0.0);
    assert_eq!(viewport.offset.y, 0.0);
    assert_eq!(viewport.zoom, 1.0);
}

#[test]
fn test_viewport_intersects_rect() {
    let viewport = Viewport::new(Position::new(0.0, 0.0), 1.0);
    let rect_inside = Rect::new(50.0, 50.0, 100.0, 100.0);
    let rect_outside = Rect::new(1000.0, 1000.0, 100.0, 100.0);

    assert!(viewport.intersects_rect(rect_inside));
    assert!(!viewport.intersects_rect(rect_outside));
}

#[test]
fn test_viewport_world_to_screen() {
    let viewport = Viewport::new(Position::new(100.0, 200.0), 2.0);
    let world_pos = Position::new(50.0, 75.0);
    let screen_pos = viewport.world_to_screen(world_pos);

    assert_eq!(screen_pos.x, 0.0); // (50 - 100) * 2
    assert_eq!(screen_pos.y, -250.0); // (75 - 200) * 2
}

#[test]
fn test_viewport_screen_to_world() {
    let viewport = Viewport::new(Position::new(100.0, 200.0), 2.0);
    let screen_pos = Position::new(0.0, -250.0);
    let world_pos = viewport.screen_to_world(screen_pos);

    assert_eq!(world_pos.x, 100.0); // 0 / 2 + 100
    assert_eq!(world_pos.y, 75.0); // -250 / 2 + 200
}

#[test]
fn test_viewport_zoom_in() {
    let mut viewport = Viewport::default();
    viewport.zoom_in(1.5);

    assert_eq!(viewport.zoom, 1.5);
}

#[test]
fn test_viewport_zoom_out() {
    let mut viewport = Viewport::new(Position::zero(), 2.0);
    viewport.zoom_out(0.5);

    assert_eq!(viewport.zoom, 1.0);
}

#[test]
fn test_viewport_pan() {
    let mut viewport = Viewport::default();
    viewport.pan(Position::new(100.0, 200.0));

    assert_eq!(viewport.offset.x, 100.0);
    assert_eq!(viewport.offset.y, 200.0);
}

#[test]
fn test_viewport_fit_to_rect() {
    let mut viewport = Viewport::default();
    let rect = Rect::new(0.0, 0.0, 1000.0, 1000.0);
    let screen_size = Size::new(800.0, 600.0);

    viewport.fit_to_rect(rect, screen_size);

    // Should zoom to fit the rect in the screen
    assert!(viewport.zoom < 1.0);
    assert!(viewport.zoom > 0.0);
}

#[test]
fn test_position_display() {
    let pos = Position::new(10.5, 20.25);
    let display = format!("{}", pos);
    assert!(display.contains("10.5"));
    assert!(display.contains("20.25"));
}

#[test]
fn test_size_display() {
    let size = Size::new(100.5, 200.25);
    let display = format!("{}", size);
    assert!(display.contains("100.5"));
    assert!(display.contains("200.25"));
}

#[test]
fn test_rect_display() {
    let rect = Rect::new(10.0, 20.0, 100.0, 200.0);
    let display = format!("{}", rect);
    assert!(display.contains("10"));
    assert!(display.contains("20"));
    assert!(display.contains("100"));
    assert!(display.contains("200"));
}

#[test]
fn test_node_id_display() {
    let id = NodeId::new("test-node");
    let display = format!("{}", id);
    assert_eq!(display, "test-node");
}

#[test]
fn test_edge_id_display() {
    let id = EdgeId::new("test-edge");
    let display = format!("{}", id);
    assert_eq!(display, "test-edge");
}

#[test]
fn test_viewport_display() {
    let viewport = Viewport::new(Position::new(100.0, 200.0), 1.5);
    let display = format!("{}", viewport);
    assert!(display.contains("100"));
    assert!(display.contains("200"));
    assert!(display.contains("1.5"));
}

// Property-based tests for complex operations
#[test]
fn test_position_operations_commutativity() {
    let pos1 = Position::new(10.0, 20.0);
    let pos2 = Position::new(5.0, 15.0);

    // Addition should be commutative
    assert_eq!(pos1.add(pos2), pos2.add(pos1));

    // Distance should be symmetric
    assert_eq!(pos1.distance_to(pos2), pos2.distance_to(pos1));
    assert_eq!(pos1.distance_squared_to(pos2), pos2.distance_squared_to(pos1));
}

#[test]
fn test_rect_operations_consistency() {
    let rect = Rect::new(10.0, 20.0, 100.0, 200.0);
    let expanded = rect.expand(5.0);
    let shrunk_back = expanded.shrink(5.0);

    // Expanding and shrinking by the same amount should return to original
    assert_eq!(rect.x, shrunk_back.x);
    assert_eq!(rect.y, shrunk_back.y);
    assert_eq!(rect.width, shrunk_back.width);
    assert_eq!(rect.height, shrunk_back.height);
}

#[test]
fn test_viewport_coordinate_conversion_roundtrip() {
    let viewport = Viewport::new(Position::new(100.0, 200.0), 2.0);
    let original_world = Position::new(50.0, 75.0);

    let screen = viewport.world_to_screen(original_world);
    let back_to_world = viewport.screen_to_world(screen);

    assert!((original_world.x - back_to_world.x).abs() < 1e-10);
    assert!((original_world.y - back_to_world.y).abs() < 1e-10);
}
