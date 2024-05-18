use sdl2::{event::Event, keyboard::Keycode, rect::Rect, render::WindowCanvas};

use crate::{
    bullet::Bullet,
    entity::{ComponentTexture, Entity, EntityBase, EntityEvent},
};

pub struct Player<'a> {
    base: EntityBase,
    engine_texture: ComponentTexture<'a>,
    body_texture: ComponentTexture<'a>,
    projectile_texture: ComponentTexture<'a>,
    pub firing_speed: usize,
    pub cd: usize,
    pub firing: bool,
    pub firing_ready: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
}
impl<'a> Entity<'a> for Player<'a> {
    fn update(&mut self) -> EntityEvent<'a> {
        if self.up {
            self.base.update_y_rev();
        }
        if self.down {
            self.base.update_y();
        }
        if self.left {
            self.base.update_x_rev();
        }
        if self.right {
            self.base.update_x();
        }
        if self.firing {
            if !self.firing_ready {
                self.cd = (self.cd + 1) % self.firing_speed;
                if self.cd == 0 {
                    self.firing_ready = true;
                }
                EntityEvent::Empty
            } else {
                self.firing_ready = false;
                EntityEvent::SpawnBullet(self.spawn_bullet())
            }
        } else {
            EntityEvent::Empty
        }
    }

    fn render(&mut self, canvas: &mut WindowCanvas) {
        self.body_texture.render((self.base.x, self.base.y), canvas);
        self.engine_texture
            .render((self.base.x, self.base.y), canvas);
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => self.handle_key_down(keycode),
            Event::KeyUp {
                keycode: Some(keycode),
                ..
            } => self.handle_key_up(keycode),
            _ => {}
        }
    }

    fn valid(&self) -> bool {
        self.base.valid
    }
}

impl<'a> Player<'a> {
    const DEFAULT_SPEED: i32 = 8;
    const DEFAULT_POSITION: (i32, i32) = (100, 100);
    pub fn new(
        viewport: Rect,
        engine_texture: ComponentTexture<'a>,
        body_texture: ComponentTexture<'a>,
        projectile_texture: ComponentTexture<'a>,
    ) -> Self {
        let base = EntityBase {
            x: Self::DEFAULT_POSITION.0,
            y: Self::DEFAULT_POSITION.1,
            dx: Self::DEFAULT_SPEED,
            dy: Self::DEFAULT_SPEED,
            viewport,
            valid: true,
        };
        Self {
            base,
            engine_texture,
            body_texture,
            projectile_texture,
            firing_speed: 16,
            cd: 0,
            firing: false,
            firing_ready: true,
            left: false,
            right: false,
            up: false,
            down: false,
        }
    }
    pub fn spawn_bullet(&self) -> Bullet<'a> {
        let base = EntityBase {
            x: self.base.x,
            y: self.base.y,
            dx: self.firing_speed as i32,
            dy: 0,
            viewport: self.base.viewport,
            valid: true,
        };

        Bullet::new(base, self.projectile_texture.clone())
    }

    fn handle_key_down(&mut self, keycode: Keycode) {
        match keycode {
            Keycode::Up => {
                self.up = true;
            }
            Keycode::Down => {
                self.down = true;
            }
            Keycode::Left => {
                self.left = true;
            }
            Keycode::Right => {
                self.right = true;
            }
            Keycode::LCtrl => {
                self.firing = true;
                self.firing_ready = true;
                self.cd = 0;
            }
            _ => {}
        }
    }
    fn handle_key_up(&mut self, keycode: Keycode) {
        match keycode {
            Keycode::Up => {
                self.up = false;
            }
            Keycode::Down => {
                self.down = false;
            }
            Keycode::Left => {
                self.left = false;
            }
            Keycode::Right => {
                self.right = false;
            }
            Keycode::LCtrl => {
                self.firing = false;
                self.firing_ready = true;
                self.cd = 0;
            }
            _ => {}
        }
    }
}
