//! Query builder for spatial queries

use crate::types::{NodeId, Position, Rect};

use super::index::SpatialIndex;

/// Query builder for spatial queries
pub struct SpatialQuery<'a> {
    index: &'a SpatialIndex,
    bounds: Option<Rect>,
    center: Option<Position>,
    radius: Option<f64>,
    max_results: Option<usize>,
}

impl<'a> SpatialQuery<'a> {
    /// Create a new spatial query
    pub fn new(index: &'a SpatialIndex) -> Self {
        Self {
            index,
            bounds: None,
            center: None,
            radius: None,
            max_results: None,
        }
    }

    /// Filter by rectangular bounds
    pub fn bounds(mut self, bounds: Rect) -> Self {
        self.bounds = Some(bounds);
        self
    }

    /// Filter by circular area
    pub fn radius(mut self, center: Position, radius: f64) -> Self {
        self.center = Some(center);
        self.radius = Some(radius);
        self
    }

    /// Limit the number of results
    pub fn limit(mut self, max_results: usize) -> Self {
        self.max_results = Some(max_results);
        self
    }

    /// Execute the query and return node IDs
    pub fn execute(self) -> Vec<NodeId> {
        let mut results = if let Some(bounds) = self.bounds {
            self.index.query_rect(&bounds)
        } else if let (Some(center), Some(radius)) = (self.center, self.radius) {
            self.index.query_radius(center, radius)
        } else {
            self.index.node_ids()
        };

        if let Some(max_results) = self.max_results {
            results.truncate(max_results);
        }

        results
    }
}
