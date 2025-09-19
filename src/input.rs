use crate::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Mouse4,
    Mouse5,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MouseButtonState {
    pub pressed: bool,
    pub released: bool,
    pub position: Option<Rect>,
    pub inital_position: Option<Rect>,
    pub release_position: Option<Rect>,
}

impl MouseButtonState {
    pub const fn new() -> Self {
        Self {
            pressed: false,
            released: false,
            position: None,
            inital_position: None,
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

    pub fn clicked(&mut self, area: Rect) -> bool {
        let Some(inital) = self.inital_position else {
            return false;
        };

        let Some(release) = self.release_position else {
            return false;
        };

        //Make sure the user clicked and released the mouse on top of the desired area.
        if self.released && inital.intersects(area) && release.intersects(area) {
            self.position = None;
            self.released = false;
            true
        } else {
            false
        }
    }

    pub(crate) fn pressed(&mut self, pos: Rect) {
        self.pressed = true;
        self.released = false;
        self.inital_position = Some(pos);
    }

    pub(crate) fn released(&mut self, pos: Rect) {
        self.pressed = false;
        self.released = true;
        self.release_position = Some(pos);
    }
}
