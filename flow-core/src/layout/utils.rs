//! Layout utility functions

use crate::graph::Graph;
use crate::types::Position;

/// Layout utilities
pub struct LayoutUtils;

impl LayoutUtils {
    /// Center a graph around the origin
    pub fn center_graph<N: Clone, E>(graph: &mut Graph<N, E>) {
        if let Some(bounds) = graph.bounds() {
            let center = bounds.center();
            let offset = Position::zero() - center;

            for node in graph.nodes_mut() {
                node.position += offset;
            }
        }
    }

    /// Scale a graph to fit within given bounds
    pub fn scale_to_fit<N: Clone, E>(
        graph: &mut Graph<N, E>,
        target_width: f64,
        target_height: f64,
    ) {
        if let Some(bounds) = graph.bounds() {
            if bounds.width > 0.0 && bounds.height > 0.0 {
                let scale_x = target_width / bounds.width;
                let scale_y = target_height / bounds.height;
                let scale = scale_x.min(scale_y);

                let center = bounds.center();

                for node in graph.nodes_mut() {
                    let relative_pos = node.position - center;
                    node.position = center + relative_pos * scale;
                }
            }
        }
    }

    /// Apply padding around a graph
    pub fn apply_padding<N, E>(graph: &mut Graph<N, E>, padding: f64) {
        for node in graph.nodes_mut() {
            node.position += Position::new(padding, padding);
        }
    }
}
