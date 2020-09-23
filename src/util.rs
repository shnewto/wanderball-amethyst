pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct Rectangle {
    pub left: f32,
    pub bottom: f32,
    pub right: f32,
    pub top: f32,
}

pub fn point_near_rect(point: Point, input: Rectangle, threshold: f32) -> bool {
    let updated = Rectangle {
        left: input.left + threshold,
        bottom: input.bottom + threshold,
        right: input.right + threshold,
        top: input.top + threshold,
    };

    point_in_rect(point, updated)
}

fn point_in_rect(point: Point, rect: Rectangle) -> bool {
    point.x >= rect.left && point.x <= rect.right && point.y >= rect.bottom && point.y <= rect.top
}
