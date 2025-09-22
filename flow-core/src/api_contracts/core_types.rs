//! Core Types API Contracts

use crate::types::{Position, Rect, Size, Viewport};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_api_contract() {
        // Test Position creation and basic operations
        let pos1 = Position::new(10.0, 20.0);
        let pos2 = Position::new(5.0, 15.0);

        // Test basic arithmetic operations
        let sum = pos1 + pos2;
        assert_eq!(sum.x, 15.0);
        assert_eq!(sum.y, 35.0);

        let diff = pos1 - pos2;
        assert_eq!(diff.x, 5.0);
        assert_eq!(diff.y, 5.0);

        // Test distance calculations
        let distance = pos1.distance_to(pos2);
        assert!((distance - 7.0710678118654755).abs() < 1e-10);

        // Test validation
        assert!(pos1.is_valid());
        assert!(!Position::new(f64::NAN, 0.0).is_valid());
        assert!(!Position::new(f64::INFINITY, 0.0).is_valid());

        // Test display formatting
        assert_eq!(format!("{}", pos1), "(10, 20)");

        // Test default
        assert_eq!(Position::default(), Position::zero());
    }

    #[test]
    fn test_size_api_contract() {
        let size = Size::new(100.0, 200.0);

        // Test basic properties
        assert_eq!(size.width, 100.0);
        assert_eq!(size.height, 200.0);

        // Test validation
        assert!(size.is_valid());
        assert!(!Size::new(-10.0, 20.0).is_valid());
        assert!(!Size::new(f64::NAN, 20.0).is_valid());

        // Test default
        assert_eq!(Size::default(), Size::new(100.0, 50.0));
    }

    #[test]
    fn test_rect_api_contract() {
        let rect = Rect::new(10.0, 20.0, 100.0, 200.0);

        // Test basic properties
        assert_eq!(rect.x, 10.0);
        assert_eq!(rect.y, 20.0);
        assert_eq!(rect.width, 100.0);
        assert_eq!(rect.height, 200.0);

        // Test computed properties
        assert_eq!(rect.x + rect.width, 110.0);
        assert_eq!(rect.y + rect.height, 220.0);
        assert_eq!(rect.center(), Position::new(60.0, 120.0));

        // Test point containment
        assert!(rect.contains_point(Position::new(50.0, 100.0)));
        assert!(!rect.contains_point(Position::new(5.0, 15.0)));

        // Test intersection
        let other = Rect::new(50.0, 50.0, 100.0, 100.0);
        assert!(rect.intersects(&other));

        let non_intersecting = Rect::new(200.0, 300.0, 50.0, 50.0);
        assert!(!rect.intersects(&non_intersecting));
    }

    #[test]
    fn test_viewport_api_contract() {
        let viewport = Viewport::new(0.0, 0.0, 800.0, 600.0, 1.0);

        // Test basic properties
        assert_eq!(viewport.x, 0.0);
        assert_eq!(viewport.y, 0.0);
        assert_eq!(viewport.width, 800.0);
        assert_eq!(viewport.height, 600.0);
        assert_eq!(viewport.zoom, 1.0);

        // Test coordinate transformations
        let flow_pos = Position::new(100.0, 200.0);
        let screen_pos = viewport.flow_to_screen(flow_pos);
        assert_eq!(screen_pos, Position::new(100.0, 200.0));

        let back_to_flow = viewport.screen_to_flow(screen_pos);
        assert_eq!(back_to_flow, flow_pos);

        // Test zoom transformations
        let zoomed_viewport = Viewport::new(0.0, 0.0, 800.0, 600.0, 2.0);
        let zoomed_screen = zoomed_viewport.flow_to_screen(flow_pos);
        assert_eq!(zoomed_screen, Position::new(200.0, 400.0));
    }
}
