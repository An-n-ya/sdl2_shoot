use crate::{
    entity::{Entity, EntityBase, EntityEvent},
    texture::ComponentTexture,
};

pub enum Side {
    Enemy,
    Player
}

pub struct Bullet<'a> {
    base: EntityBase,
    side: Side,
    angle: f64,
    texture: ComponentTexture<'a>,
}

impl<'a> Bullet<'a> {
    pub fn new(base: EntityBase, side: Side, angle: f64, texture: ComponentTexture<'a>) -> Self {
        Self {
            base,
            side,
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

    fn is_bullet(&self) -> bool {
        true
    }
    fn is_enemy(&self) -> bool {
        match self.side {
            Side::Enemy => true,
            Side::Player => false,
        }
    }
    fn is_player(&self) -> bool {
        match self.side {
            Side::Enemy => false,
            Side::Player => true,
        }
    }
    fn base(&self) -> Option<&EntityBase> {
        Some(&self.base)
    }
    fn base_mut(&mut self) -> Option<&mut EntityBase> {
        Some(&mut self.base)
    }
}
