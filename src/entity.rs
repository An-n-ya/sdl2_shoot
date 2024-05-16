use std::rc::Rc;

use sdl2::render::Texture;

pub struct Entity<'a> {
    pub x: i32,
    pub y: i32,
    pub texture: Rc<Texture<'a>>,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
}

impl<'a> Entity<'a> {
    pub fn new(x: i32, y: i32, texture: Rc<Texture<'a>>) -> Self {
        Self {
            x,
            y,
            texture,
            left: false,
            right: false,
            up: false,
            down: false,
        }
    }

    pub fn update_pos(&mut self, low_x: i32, high_x: i32, low_y: i32, high_y: i32, speed: i32) {
        if self.up && self.y - speed > low_y {
            self.y -= speed;
        }
        if self.down && self.y + speed < high_y {
            self.y += speed;
        }
        if self.left && self.x - speed > low_x {
            self.x -= speed;
        }
        if self.right && self.x + speed < high_x {
            self.x += speed;
        }
    }
}
