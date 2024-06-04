use sdl2::{rect::Rect, render::TextureCreator, video::WindowContext};

use crate::{
    bullet::Bullet,
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
            return EntityEvent::SpawnBullet(self.spawn_bullet(50));
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
}

impl<'a> Enemy<'a> {
    const DEFAULT_SPEED: i32 = 6;
    const DEFAULT_ANGLE: f64 = -90.0;
    const BULLET_SPEED: i32 = 15;
    const FIRING_SPEED: u64 = 400;
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>, viewport: Rect) -> Self {
        let engine_texture = ComponentTexture::new(&texture_creator, &ENEMY_ENGINE_TEXTURES[0]);
        let body_texture = ComponentTexture::new(&texture_creator, &ENEMY_BASE_TEXTURES[0]);
        let projectile_texture =
            ComponentTexture::new(&texture_creator, &ENEMY_PROJECTILE_TEXTURES[0]);
        let y = (rand::random::<u32>() % viewport.height()) as i32;
        let y = y.clamp(100, viewport.height() as i32 - 100);
        let ticks = unsafe { sdl2_sys::SDL_GetTicks64() };
        let base = EntityBase {
            x: viewport.width() as i32,
            y,
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
        let base = EntityBase {
            x: self.base.x,
            y: self.base.y + offset,
            dx: -Self::BULLET_SPEED,
            dy: 0,
            viewport: self.base.viewport,
            valid: true,
        };

        Bullet::new(base, Self::DEFAULT_ANGLE, self.projectile_texture.clone())
    }
}
