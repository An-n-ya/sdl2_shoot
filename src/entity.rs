use std::{fmt::Debug, rc::Rc};

use sdl2::render::Texture;

pub struct Entity<'a> {
    pub x: i32,
    pub y: i32,
    pub dx: i32,
    pub dy: i32,
    pub firing: bool,
    pub firing_speed: usize,
    pub cd: usize,
    pub firing_ready: bool,
    pub health: i32,
    pub total_frame: usize,
    pub current_frame: usize,
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
            dx: 0,
            dy: 0,
            health: 1,
            total_frame: 1,
            current_frame: 0,
            firing: false,
            firing_ready: true,
            firing_speed: 20,
            cd: 0,
            texture,
            left: false,
            right: false,
            up: false,
            down: false,
        }
    }

    pub fn update(&mut self, low_x: i32, high_x: i32, low_y: i32, high_y: i32, speed: i32) {
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
        if self.firing && !self.firing_ready {
            self.cd = (self.cd + 1) % self.firing_speed;
            if self.cd == 0 {
                self.firing_ready = true;
            }
        }
    }
}

impl<'a> Debug for Entity<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Entity")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("dx", &self.dx)
            .field("dy", &self.dy)
            .field("health", &self.health)
            .field("total_frame", &self.total_frame)
            .field("current_frame", &self.current_frame)
            .finish()
    }
}
