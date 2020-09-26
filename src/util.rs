use serde::{Deserialize, Serialize};

use crate::side::Side;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Rectangle {
    pub left: f32,
    pub bottom: f32,
    pub right: f32,
    pub top: f32,
}

pub fn _point_in_rect(point: &Point, rect: &Rectangle) -> bool {
    point.x >= rect.left && point.x <= rect.right && point.y >= rect.bottom && point.y <= rect.top
}

pub fn _point_on_edge_of_rect(point: Point, rect_center: Point, dist_to_edge: f32) -> Option<Side> {
    if point.x >= rect_center.x + dist_to_edge {
        Some(Side::Right)
    } else if point.x <= rect_center.x - dist_to_edge {
        Some(Side::Left)
    } else if point.y >= rect_center.y + dist_to_edge {
        Some(Side::Top)
    } else if point.y <= rect_center.y - dist_to_edge {
        Some(Side::Bottom)
    } else {
        None
    }
}

pub fn point_near_edge_of_rect(
    point: &Point,
    rect_center: &Point,
    dist_to_edge: f32,
    threshold: f32,
) -> Option<Side> {
    if (point.x + threshold) >= rect_center.x + dist_to_edge {
        Some(Side::Right)
    } else if (point.x - threshold) <= rect_center.x - dist_to_edge {
        Some(Side::Left)
    } else if (point.y + threshold) >= rect_center.y + dist_to_edge {
        Some(Side::Top)
    } else if (point.y - threshold) <= rect_center.y - dist_to_edge {
        Some(Side::Bottom)
    } else {
        None
    }
}
