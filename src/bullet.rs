use crate::{
    entity::{Entity, EntityBase, EntityEvent},
    texture::ComponentTexture,
};

pub struct Bullet<'a> {
    base: EntityBase,
    angle: f64,
    texture: ComponentTexture<'a>,
}

impl<'a> Bullet<'a> {
    pub fn new(base: EntityBase, angle: f64, texture: ComponentTexture<'a>) -> Self {
        Self {
            base,
            texture,
            angle,
        }
    }
}

impl<'a> Entity<'a> for Bullet<'a> {
    fn update(&mut self) -> EntityEvent<'a> {
        if !self.base.update_x() {
            self.base.valid = false;
        }
        EntityEvent::Empty
    }

    fn render(&mut self, canvas: &mut sdl2::render::WindowCanvas) {
        self.texture
            .render((self.base.x, self.base.y), self.angle, canvas);
    }

    fn valid(&self) -> bool {
        self.base.valid
    }
}
