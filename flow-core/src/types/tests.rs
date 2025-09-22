//! Tests for core types

// use super::geometry::{Position, Rect, Size}; // Unused imports
// use super::identifiers::{EdgeId, GroupId, NodeId}; // Unused imports
// use super::viewport::Viewport; // Unused imports

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Position, Size, Rect, Viewport, NodeId, EdgeId, GroupId};

    #[test]
    fn test_position_operations() {
        let p1 = Position::new(1.0, 2.0);
        let p2 = Position::new(3.0, 4.0);

        assert_eq!(p1 + p2, Position::new(4.0, 6.0));
        assert_eq!(p2 - p1, Position::new(2.0, 2.0));
        assert_eq!(p1 * 2.0, Position::new(2.0, 4.0));
        assert_eq!(p1.distance_to(p2), (8.0_f64).sqrt());
    }


    #[test]
    fn test_viewport_transformations() {
        let viewport = Viewport::new(10.0, 10.0, 100.0, 100.0, 2.0);

        let flow_pos = Position::new(60.0, 60.0);
        let screen_pos = viewport.flow_to_screen(flow_pos);
        assert_eq!(screen_pos, Position::new(100.0, 100.0));

        let back_to_flow = viewport.screen_to_flow(screen_pos);
        assert_eq!(back_to_flow, flow_pos);
    }

    // Targeted unit tests for mathematical operations (mutation testing coverage)

    #[test]
    fn test_position_distance_calculation() {
        // Test basic distance calculation
        let p1 = Position::new(0.0, 0.0);
        let p2 = Position::new(3.0, 4.0);
        let distance = p1.distance_to(p2);
        assert_eq!(distance, 5.0); // 3-4-5 triangle

        // Test distance squared
        let distance_sq = p1.distance_squared_to(p2);
        assert_eq!(distance_sq, 25.0);

        // Test with negative coordinates
        let p3 = Position::new(-3.0, -4.0);
        let distance_neg = p1.distance_to(p3);
        assert_eq!(distance_neg, 5.0);

        // Test with decimal coordinates
        let p4 = Position::new(1.5, 2.0);
        let p5 = Position::new(4.5, 6.0);
        let distance_decimal = p4.distance_to(p5);
        assert_eq!(distance_decimal, 5.0);
    }

    #[test]
    fn test_position_arithmetic_operations() {
        let p1 = Position::new(10.0, 20.0);
        let p2 = Position::new(5.0, 15.0);

        // Test addition
        let sum = p1.add(p2);
        assert_eq!(sum, Position::new(15.0, 35.0));

        // Test subtraction
        let diff = p1.sub(p2);
        assert_eq!(diff, Position::new(5.0, 5.0));

        // Test scaling
        let scaled = p1.scale(2.0);
        assert_eq!(scaled, Position::new(20.0, 40.0));

        // Test operator overloading
        assert_eq!(p1 + p2, sum);
        assert_eq!(p1 - p2, diff);
        assert_eq!(p1 * 2.0, scaled);
    }

    #[test]
    fn test_position_validity() {
        // Test valid positions
        let valid_pos = Position::new(10.0, 20.0);
        assert!(valid_pos.is_valid());

        // Test with zero
        let zero_pos = Position::zero();
        assert!(zero_pos.is_valid());

        // Test with negative values
        let neg_pos = Position::new(-10.0, -20.0);
        assert!(neg_pos.is_valid());

        // Test with very large values
        let large_pos = Position::new(1e10, 1e10);
        assert!(large_pos.is_valid());

        // Test with very small values
        let small_pos = Position::new(1e-10, 1e-10);
        assert!(small_pos.is_valid());
    }

    #[test]
    fn test_size_operations() {
        let size1 = Size::new(10.0, 20.0);
        let size2 = Size::new(5.0, 15.0);

        // Test area calculation
        assert_eq!(size1.area(), 200.0);
        assert_eq!(size2.area(), 75.0);

        // Test scaling
        let scaled = size1.scale(2.0);
        assert_eq!(scaled, Size::new(20.0, 40.0));

        // Test square creation
        let square = Size::square(5.0);
        assert_eq!(square, Size::new(5.0, 5.0));

        // Test zero size
        let zero = Size::zero();
        assert_eq!(zero, Size::new(0.0, 0.0));
        assert_eq!(zero.area(), 0.0);
    }

    #[test]
    fn test_size_validity() {
        // Test valid sizes
        let valid_size = Size::new(10.0, 20.0);
        assert!(valid_size.is_valid());

        // Test with zero dimensions
        let zero_size = Size::zero();
        assert!(zero_size.is_valid());

        // Test with very small dimensions
        let small_size = Size::new(1e-10, 1e-10);
        assert!(small_size.is_valid());

        // Test with very large dimensions
        let large_size = Size::new(1e10, 1e10);
        assert!(large_size.is_valid());
    }

    #[test]
    fn test_rect_operations() {
        let rect1 = Rect::new(0.0, 0.0, 10.0, 10.0);
        let rect2 = Rect::new(5.0, 5.0, 10.0, 10.0);

        // Test point containment
        assert!(rect1.contains_point(Position::new(5.0, 5.0)));
        assert!(rect1.contains_point(Position::new(0.0, 0.0)));
        assert!(rect1.contains_point(Position::new(10.0, 10.0)));
        assert!(!rect1.contains_point(Position::new(15.0, 15.0)));

        // Test intersection
        assert!(rect1.intersects(&rect2));
        assert!(rect2.intersects(&rect1));

        // Test union
        let union = rect1.union(rect2);
        assert_eq!(union, Rect::new(0.0, 0.0, 15.0, 15.0));

        // Test intersection
        let intersection = rect1.intersection(rect2);
        assert_eq!(intersection, Some(Rect::new(5.0, 5.0, 5.0, 5.0)));

        // Test non-intersecting rectangles
        let rect3 = Rect::new(20.0, 20.0, 10.0, 10.0);
        assert!(!rect1.intersects(&rect3));
        assert_eq!(rect1.intersection(rect3), None);
    }

    #[test]
    fn test_rect_creation() {
        let pos = Position::new(10.0, 20.0);
        let size = Size::new(30.0, 40.0);
        let rect = Rect::from_pos_size(pos, size);

        assert_eq!(rect.x, 10.0);
        assert_eq!(rect.y, 20.0);
        assert_eq!(rect.width, 30.0);
        assert_eq!(rect.height, 40.0);

        // Test position and size getters
        assert_eq!(rect.position(), pos);
        assert_eq!(rect.size(), size);

        // Test center calculation
        let center = rect.center();
        assert_eq!(center, Position::new(25.0, 40.0));
    }

    #[test]
    fn test_viewport_operations() {
        let viewport = Viewport::new(10.0, 20.0, 100.0, 200.0, 2.0);

        // Test bounds
        let bounds = viewport.bounds();
        assert_eq!(bounds, Rect::new(10.0, 20.0, 100.0, 200.0));

        // Test center
        let center = viewport.center();
        assert_eq!(center, Position::new(60.0, 120.0));

        // Test point containment
        assert!(viewport.contains_point(Position::new(60.0, 120.0)));
        assert!(!viewport.contains_point(Position::new(200.0, 300.0)));

        // Test coordinate transformations
        let flow_pos = Position::new(60.0, 120.0);
        let screen_pos = viewport.flow_to_screen(flow_pos);
        assert_eq!(screen_pos, Position::new(100.0, 200.0));

        let back_to_flow = viewport.screen_to_flow(screen_pos);
        assert_eq!(back_to_flow, flow_pos);
    }

    #[test]
    fn test_viewport_modifications() {
        let viewport = Viewport::new(0.0, 0.0, 100.0, 100.0, 1.0);

        // Test panning
        let panned = viewport.pan(Position::new(10.0, 20.0));
        assert_eq!(panned.offset, Position::new(10.0, 20.0));

        // Test zooming
        let zoomed = viewport.zoom(2.0);
        assert_eq!(zoomed.zoom, 2.0);

        // Test setting zoom
        let set_zoom = viewport.set_zoom(1.5);
        assert_eq!(set_zoom.zoom, 1.5);

        // Test setting position
        let set_pos = viewport.set_position(50.0, 60.0);
        assert_eq!(set_pos.x, 50.0);
        assert_eq!(set_pos.y, 60.0);

        // Test setting size
        let set_size = viewport.set_size(200.0, 300.0);
        assert_eq!(set_size.width, 200.0);
        assert_eq!(set_size.height, 300.0);
    }

    #[test]
    fn test_identifier_creation() {
        // Test NodeId
        let node_id = NodeId::new("test_node");
        assert_eq!(node_id.to_string(), "test_node");

        let node_id_from_str = NodeId::from("test_node_str");
        assert_eq!(node_id_from_str.to_string(), "test_node_str");

        // Test EdgeId
        let edge_id = EdgeId::new("test_edge");
        assert_eq!(edge_id.to_string(), "test_edge");

        let edge_id_from_str = EdgeId::from("test_edge_str");
        assert_eq!(edge_id_from_str.to_string(), "test_edge_str");

        // Test GroupId
        let group_id = GroupId::new("test_group");
        assert_eq!(group_id.to_string(), "test_group");

        let group_id_from_str = GroupId::from("test_group_str");
        assert_eq!(group_id_from_str.to_string(), "test_group_str");
    }

    #[test]
    fn test_identifier_generation() {
        // Test that generated IDs are unique
        let node_id1 = NodeId::generate();
        let node_id2 = NodeId::generate();
        assert_ne!(node_id1, node_id2);

        let edge_id1 = EdgeId::generate();
        let edge_id2 = EdgeId::generate();
        assert_ne!(edge_id1, edge_id2);

        let group_id1 = GroupId::generate();
        let group_id2 = GroupId::generate();
        assert_ne!(group_id1, group_id2);
    }

    // Edge case tests for mathematical operations

    #[test]
    fn test_position_edge_cases() {
        // Test with very small values
        let tiny_pos = Position::new(1e-20, 1e-20);
        assert!(tiny_pos.is_valid());
        assert_eq!(tiny_pos.distance_to(Position::zero()), 1e-20 * (2.0_f64).sqrt());

        // Test with very large values
        let huge_pos = Position::new(1e20, 1e20);
        assert!(huge_pos.is_valid());
        assert_eq!(huge_pos.distance_to(Position::zero()), 1e20 * (2.0_f64).sqrt());

        // Test scaling with very small factors
        let normal_pos = Position::new(100.0, 100.0);
        let scaled_tiny = normal_pos.scale(1e-10);
        assert_eq!(scaled_tiny, Position::new(1e-8, 1e-8));

        // Test scaling with very large factors
        let scaled_huge = normal_pos.scale(1e10);
        assert_eq!(scaled_huge, Position::new(1e12, 1e12));
    }

    #[test]
    fn test_size_edge_cases() {
        // Test with very small sizes
        let tiny_size = Size::new(1e-10, 1e-10);
        assert!(tiny_size.is_valid());
        let expected_area = 1e-20;
        let actual_area = tiny_size.area();
        assert!(
            (actual_area - expected_area).abs() < f64::EPSILON,
            "Expected area {} but got {}",
            expected_area,
            actual_area
        );

        // Test with very large sizes
        let huge_size = Size::new(1e10, 1e10);
        assert!(huge_size.is_valid());
        assert_eq!(huge_size.area(), 1e20);

        // Test scaling with very small factors
        let normal_size = Size::new(100.0, 100.0);
        let scaled_tiny = normal_size.scale(1e-10);
        assert_eq!(scaled_tiny, Size::new(1e-8, 1e-8));

        // Test scaling with very large factors
        let scaled_huge = normal_size.scale(1e10);
        assert_eq!(scaled_huge, Size::new(1e12, 1e12));
    }

    #[test]
    fn test_viewport_edge_cases() {
        // Test with zero zoom
        let viewport_zero = Viewport::new(0.0, 0.0, 100.0, 100.0, 0.0);
        let flow_pos = Position::new(50.0, 50.0);
        let screen_pos = viewport_zero.flow_to_screen(flow_pos);
        assert_eq!(screen_pos, Position::new(0.0, 0.0));

        // Test with very large zoom
        let viewport_huge = Viewport::new(0.0, 0.0, 100.0, 100.0, 1e10);
        let screen_pos_huge = viewport_huge.flow_to_screen(flow_pos);
        assert!(screen_pos_huge.x > 1e9);
        assert!(screen_pos_huge.y > 1e9);

        // Test with negative offset - fix calculation logic
        let viewport_neg = Viewport::new(-100.0, -100.0, 100.0, 100.0, 1.0);
        let screen_pos_neg = viewport_neg.flow_to_screen(Position::new(0.0, 0.0));
        // flow_to_screen: (flow_pos.x - viewport.x) * zoom = (0.0 - (-100.0)) * 1.0 = 100.0
        assert_eq!(screen_pos_neg, Position::new(100.0, 100.0));

        // Test with very small viewport
        let viewport_tiny = Viewport::new(0.0, 0.0, 1e-10, 1e-10, 1.0);
        let bounds = viewport_tiny.bounds();
        assert_eq!(
            bounds,
            Rect::from_pos_size(Position::new(0.0, 0.0), Size::new(1e-10, 1e-10))
        );
    }

    #[test]
    fn test_arithmetic_edge_cases() {
        // Test addition with zero
        let p1 = Position::new(10.0, 20.0);
        let p_zero = Position::zero();
        assert_eq!(p1.add(p_zero), p1);
        assert_eq!(p_zero.add(p1), p1);

        // Test subtraction with zero
        assert_eq!(p1.sub(p_zero), p1);
        assert_eq!(p_zero.sub(p1), Position::new(-10.0, -20.0));

        // Test scaling with zero
        assert_eq!(p1.scale(0.0), Position::zero());

        // Test scaling with one
        assert_eq!(p1.scale(1.0), p1);

        // Test division by zero (should panic or handle gracefully)
        // Note: This depends on implementation - might need to adjust based on actual behavior

        // Test with very small scaling factors
        let scaled_tiny = p1.scale(1e-10);
        assert_eq!(scaled_tiny, Position::new(1e-9, 2e-9));

        // Test with very large scaling factors
        let scaled_huge = p1.scale(1e10);
        assert_eq!(scaled_huge, Position::new(1e11, 2e11));
    }

    #[test]
    fn test_distance_edge_cases() {
        // Test distance between identical points
        let p1 = Position::new(10.0, 20.0);
        assert_eq!(p1.distance_to(p1), 0.0);
        assert_eq!(p1.distance_squared_to(p1), 0.0);

        // Test distance with very close points
        let p2 = Position::new(10.0 + 1e-10, 20.0 + 1e-10);
        let distance = p1.distance_to(p2);
        assert!(distance > 0.0);
        assert!(distance < 1e-9);

        // Test distance with very far points
        let p3 = Position::new(1e10, 1e10);
        let p4 = Position::new(-1e10, -1e10);
        let distance_far = p3.distance_to(p4);
        assert!(distance_far > 1e10);

        // Test distance squared with very close points
        let distance_sq = p1.distance_squared_to(p2);
        assert!(distance_sq > 0.0);
        assert!(distance_sq < 1e-18);
    }
}
