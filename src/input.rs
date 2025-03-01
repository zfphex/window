use crate::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Mouse4,
    Mouse5,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MouseState {
    pub pressed: bool,
    pub released: bool,
    pub inital_position: Rect,
    pub release_position: Option<Rect>,
}

impl MouseState {
    pub const fn new() -> Self {
        Self {
            pressed: false,
            released: false,
            inital_position: Rect::new(0, 0, 0, 0),
            release_position: None,
        }
    }
    pub const fn is_pressed(&mut self) -> bool {
        if self.pressed {
            self.pressed = false;
            true
        } else {
            false
        }
    }
    pub const fn is_released(&mut self) -> bool {
        if self.released {
            self.released = false;
            true
        } else {
            false
        }
    }
    //TODO: I was resetting the input each frame before, not sure on the behaviour now.
    pub const fn clicked(&mut self, area: Rect) -> bool {
        if self.released && self.inital_position.intersects(area) {
            self.pressed = false;
            self.released = false;
            true
        } else {
            false
        }
    }
    // pub(crate) const fn reset(&mut self) {
    //     self.pressed = false;
    //     self.released = false;
    // }
    pub(crate) const fn pressed(&mut self, pos: Rect) {
        self.pressed = true;
        self.released = false;
        self.inital_position = pos;
        self.release_position = None;
    }
    pub(crate) const fn released(&mut self, pos: Rect) {
        self.pressed = false;
        self.released = true;
        self.release_position = Some(pos);
    }
}
