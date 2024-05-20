use sdl2::{event::Event, rect::Rect, render::WindowCanvas};

use crate::bullet::Bullet;

pub enum EntityEvent<'a> {
    SpawnBullet(Bullet<'a>),
    Empty,
}

pub struct EntityBase {
    pub x: i32,
    pub y: i32,
    pub dx: i32,
    pub dy: i32,
    pub viewport: Rect,
    pub valid: bool,
}

impl EntityBase {
    pub fn update_x(&mut self) -> bool {
        if self.x + self.dx < self.viewport.x + self.viewport.w {
            self.x += self.dx;
            true
        } else {
            false
        }
    }
    pub fn update_y(&mut self) -> bool {
        if self.y + self.dy < self.viewport.y + self.viewport.h {
            self.y += self.dy;
            true
        } else {
            false
        }
    }
    pub fn update_x_rev(&mut self) -> bool {
        if self.x - self.dx >= self.viewport.x {
            self.x -= self.dx;
            true
        } else {
            false
        }
    }
    pub fn update_y_rev(&mut self) -> bool {
        if self.y - self.dy >= self.viewport.y {
            self.y -= self.dy;
            true
        } else {
            false
        }
    }
}

pub trait Entity<'a> {
    fn render(&mut self, canvas: &mut WindowCanvas);
    #[allow(unused)]
    fn handle_event(&mut self, event: Event) {}
    fn update(&mut self) -> EntityEvent<'a> {
        EntityEvent::Empty
    }
    fn valid(&self) -> bool;
}
