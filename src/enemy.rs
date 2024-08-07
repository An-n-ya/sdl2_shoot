use sdl2::{rect::Rect, render::TextureCreator, video::WindowContext};

use crate::{
    bullet::{Bullet, Side},
    entity::{Entity, EntityBase, EntityEvent},
    texture::{
        ComponentTexture, ENEMY_BASE_TEXTURES, ENEMY_ENGINE_TEXTURES, ENEMY_PROJECTILE_TEXTURES,
    },
};

pub struct Enemy<'a> {
    base: EntityBase,
    firing_ticks: u64,
    engine_texture: ComponentTexture<'a>,
    body_texture: ComponentTexture<'a>,
    projectile_texture: ComponentTexture<'a>,
}

impl<'a> Entity<'a> for Enemy<'a> {
    fn update(&mut self) -> EntityEvent<'a> {
        if !self.base.update_x() {
            self.base.valid = false;
        }
        let ticks = unsafe { sdl2_sys::SDL_GetTicks64() };
        if ticks - self.firing_ticks >= Self::FIRING_SPEED {
            self.firing_ticks = ticks;
            return EntityEvent::SpawnBullet(self.spawn_bullet(25));
        }
        EntityEvent::Empty
    }

    fn render(&mut self, canvas: &mut sdl2::render::WindowCanvas) {
        self.engine_texture
            .render((self.base.x, self.base.y), Self::DEFAULT_ANGLE, canvas);
        self.body_texture
            .render((self.base.x, self.base.y), Self::DEFAULT_ANGLE, canvas);
    }

    fn valid(&self) -> bool {
        self.base.valid
    }

    fn is_enemy(&self) -> bool {
        true
    }

    fn base(&self) -> Option<&EntityBase> {
        Some(&self.base)
    }
    fn base_mut(&mut self) -> Option<&mut EntityBase> {
        Some(&mut self.base)
    }
}

impl<'a> Enemy<'a> {
    const DEFAULT_SPEED: i32 = 4;
    const DEFAULT_ANGLE: f64 = -90.0;
    const BULLET_SPEED: i32 = 10;
    const FIRING_SPEED: u64 = 800;
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>, viewport: Rect) -> Self {
        let engine_texture = ComponentTexture::new(&texture_creator, &ENEMY_ENGINE_TEXTURES[0]);
        let body_texture = ComponentTexture::new(&texture_creator, &ENEMY_BASE_TEXTURES[0]);
        let projectile_texture =
            ComponentTexture::new(&texture_creator, &ENEMY_PROJECTILE_TEXTURES[0]);
        let y = (rand::random::<u32>() % viewport.height()) as i32;
        let y = y.clamp(100, viewport.height() as i32 - 100);
        let ticks = unsafe { sdl2_sys::SDL_GetTicks64() };
        let (width, height) = body_texture.size();
        let base = EntityBase {
            x: viewport.width() as i32,
            y,
            width: width.try_into().unwrap(),
            height: height.try_into().unwrap(),
            dx: -(Self::DEFAULT_SPEED + (rand::random::<u32>() % 10) as i32 - 5),
            dy: 0,
            viewport,
            valid: true,
        };
        Self {
            base,
            firing_ticks: ticks,
            engine_texture,
            body_texture,
            projectile_texture,
        }
    }
    pub fn spawn_bullet(&self, offset: i32) -> Bullet<'a> {
        let (width, height) = self.projectile_texture.size();
        let base = EntityBase {
            x: self.base.x,
            y: self.base.y + offset,
            width: width.try_into().unwrap(),
            height: height.try_into().unwrap(),
            dx: -Self::BULLET_SPEED,
            dy: 0,
            viewport: self.base.viewport,
            valid: true,
        };

        Bullet::new(
            base,
            Side::Enemy,
            Self::DEFAULT_ANGLE,
            self.projectile_texture.clone(),
        )
    }
}
