use sdl2::{event::Event, rect::Rect, render::WindowCanvas};

use crate::bullet::Bullet;

pub enum EntityEvent<'a> {
    SpawnBullet(Bullet<'a>),
    Empty,
}

pub struct EntityBase {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub dx: i32,
    pub dy: i32,
    pub viewport: Rect,
    pub valid: bool,
}

impl EntityBase {
    pub fn update_x(&mut self) -> bool {
        let new_x = self.x + self.dx;
        if new_x <= self.viewport.x + self.viewport.w && new_x + self.width >= self.viewport.x {
            self.x = new_x;
            true
        } else {
            false
        }
    }
    pub fn update_y(&mut self) -> bool {
        let new_y = self.y + self.dy;
        if new_y <= self.viewport.y + self.viewport.h && new_y + self.height >= self.viewport.y {
            self.y = new_y;
            true
        } else {
            false
        }
    }
    pub fn update_x_rev(&mut self) -> bool {
        let new_x = self.x - self.dx;
        if new_x <= self.viewport.x + self.viewport.w && new_x + self.width >= self.viewport.x {
            self.x = new_x;
            true
        } else {
            false
        }
    }
    pub fn update_y_rev(&mut self) -> bool {
        let new_y = self.y - self.dy;
        if new_y <= self.viewport.y + self.viewport.h && new_y + self.height >= self.viewport.y {
            self.y = new_y;
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
    fn is_bullet(&self) -> bool {
        false
    }
    fn is_player(&self) -> bool {
        false
    }
    fn is_enemy(&self) -> bool {
        false
    }
    fn base(&self) -> Option<&EntityBase> {
        None
    }
    fn base_mut(&mut self) -> Option<&mut EntityBase> {
        None
    }
}
