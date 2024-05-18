use std::rc::Rc;

use sdl2::{
    event::Event,
    rect::Rect,
    render::{Texture, WindowCanvas},
};

use crate::bullet::Bullet;

pub enum EntityEvent<'a> {
    SpawnBullet(Bullet<'a>),
    Empty,
}

#[derive(Clone)]
pub struct ComponentTexture<'a> {
    pub texture: Rc<Texture<'a>>,
    pub total_frame: usize,
    pub current_frame: usize,
}

impl<'a> ComponentTexture<'a> {
    pub fn render(&mut self, offset: (i32, i32), canvas: &mut WindowCanvas) {
        let query = self.texture.query();
        let total_width = query.width;
        let width = total_width / self.total_frame as u32;
        let height = query.height;
        let src_rect = Rect::new(self.current_frame as i32 * width as i32, 0, width, height);
        canvas
            .copy_ex(
                &self.texture,
                src_rect,
                Some(Rect::new(offset.0, offset.1, width * 2, height * 2)),
                90.0,
                None,
                false,
                false,
            )
            .ok();
        self.current_frame = (self.current_frame + 1) % self.total_frame;
    }
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
