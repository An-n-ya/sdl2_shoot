use std::rc::Rc;

use sdl2::{
    image::LoadTexture,
    rect::Rect,
    render::{Texture, TextureCreator, WindowCanvas},
    video::WindowContext,
};

pub struct TextureInfo<'a> {
    pub path: &'a str,
    pub total_frame: usize,
}

#[derive(Clone)]
pub struct ComponentTexture<'a> {
    pub texture: Rc<Texture<'a>>,
    pub total_frame: usize,
    pub cnt: usize,
    pub current_frame: usize,
}

impl<'a> ComponentTexture<'a> {
    const SPEED: usize = 5;
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>, texture: &TextureInfo) -> Self {
        let t = texture_creator.load_texture(texture.path).unwrap();
        ComponentTexture {
            texture: Rc::new(t),
            total_frame: texture.total_frame,
            cnt: 0,
            current_frame: 0,
        }
    }

    // return width and height
    pub fn size(&self) -> (u32, u32) {
        let query = self.texture.query();
        let total_width = query.width;
        let width = total_width / self.total_frame as u32;
        let height = query.height;
        (width, height)
    }

    pub fn render_nth(
        &mut self,
        index: usize,
        offset: (i32, i32),
        angle: f64,
        canvas: &mut WindowCanvas,
    ) {
        let query = self.texture.query();
        let total_width = query.width;
        let width = total_width / self.total_frame as u32;
        let height = query.height;
        let src_rect = Rect::new(index as i32 * width as i32, 0, width, height);
        canvas
            .copy_ex(
                &self.texture,
                src_rect,
                Some(Rect::new(offset.0, offset.1, width * 2, height * 2)),
                angle,
                None,
                false,
                false,
            )
            .ok();
    }
    pub fn render(&mut self, offset: (i32, i32), angle: f64, canvas: &mut WindowCanvas) {
        self.render_nth(self.current_frame, offset, angle, canvas);
        self.cnt = (self.cnt + 1) % (self.total_frame * Self::SPEED);
        self.current_frame = self.cnt / Self::SPEED;
    }
}

pub const PROJECTILE_TEXTURES: [TextureInfo<'static>; 4] = [
    TextureInfo {
        path:
            "assets/Main ship weapons/PNGs/Main ship weapon - Projectile - Auto cannon bullet.png",
        total_frame: 4,
    },
    TextureInfo {
        path: "assets/Main ship weapons/PNGs/Main ship weapon - Projectile - Big Space Gun.png",
        total_frame: 10,
    },
    TextureInfo {
        path: "assets/Main ship weapons/PNGs/Main ship weapon - Projectile - Rocket.png",
        total_frame: 3,
    },
    TextureInfo {
        path: "assets/Main ship weapons/PNGs/Main ship weapon - Projectile - Zapper.png",
        total_frame: 8,
    },
];
pub const WEAPON_TEXTURES: [TextureInfo<'static>; 4] = [
    TextureInfo {
        path: "assets/Main Ship/Main Ship - Weapons/PNGs/Main Ship - Weapons - Auto Cannon.png",
        total_frame: 7,
    },
    TextureInfo {
        path: "assets/Main Ship/Main Ship - Weapons/PNGs/Main Ship - Weapons - Big Space Gun.png",
        total_frame: 12,
    },
    TextureInfo {
        path: "assets/Main Ship/Main Ship - Weapons/PNGs/Main Ship - Weapons - Rockets.png",
        total_frame: 17,
    },
    TextureInfo {
        path: "assets/Main Ship/Main Ship - Weapons/PNGs/Main Ship - Weapons - Zapper.png",
        total_frame: 14,
    },
];
pub const ENGINE_TEXTURES: [TextureInfo<'static>; 4] = [
    TextureInfo {
        path: "assets/Main Ship/Main Ship - Engines/PNGs/Main Ship - Engines - Base Engine.png",
        total_frame: 1,
    },
    TextureInfo {
        path: "assets/Main Ship/Main Ship - Engines/PNGs/Main Ship - Engines - Big Pulse Engine - Powering.png",
        total_frame: 1,
    },
    TextureInfo {
        path: "assets/Main Ship/Main Ship - Engines/PNGs/Main Ship - Engines - Burst Engine - Powering.png",
        total_frame: 1,
    },
    TextureInfo {
        path: "assets/Main Ship/Main Ship - Engines/PNGs/Main Ship - Engines - Supercharged Engine - Powering.png",
        total_frame: 1,
    },
];
pub const ENGINE_EFFECTS_POWERING_TEXTURES: [TextureInfo<'static>; 4] = [
    TextureInfo {
        path: "assets/Main Ship/Main Ship - Engine Effects/PNGs/Main Ship - Engines - Base Engine - Powering.png",
        total_frame: 4,
    },
    TextureInfo {
        path: "assets/Main Ship/Main Ship - Engine Effects/PNGs/Main Ship - Engines - Big Pulse Engine - Powering.png",
        total_frame: 4,
    },
    TextureInfo {
        path: "assets/Main Ship/Main Ship - Engine Effects/PNGs/Main Ship - Engines - Burst Engine - Powering.png",
        total_frame: 6,
    },
    TextureInfo {
        path: "assets/Main Ship/Main Ship - Engine Effects/PNGs/Main Ship - Engines - Supercharged Engine - Powering.png",
        total_frame: 4,
    },
];
pub const ENGINE_EFFECTS_IDLE_TEXTURES: [TextureInfo<'static>; 4] = [
    TextureInfo {
        path: "assets/Main Ship/Main Ship - Engine Effects/PNGs/Main Ship - Engines - Base Engine - Idle.png",
        total_frame: 3,
    },
    TextureInfo {
        path: "assets/Main Ship/Main Ship - Engine Effects/PNGs/Main Ship - Engines - Big Pulse Engine - Idle.png",
        total_frame: 4,
    },
    TextureInfo {
        path: "assets/Main Ship/Main Ship - Engine Effects/PNGs/Main Ship - Engines - Burst Engine - Idle.png",
        total_frame: 7,
    },
    TextureInfo {
        path: "assets/Main Ship/Main Ship - Engine Effects/PNGs/Main Ship - Engines - Supercharged Engine - Idle.png",
        total_frame: 4,
    },
];
pub const BASE_TEXTURES: [TextureInfo<'static>; 4] = [
    TextureInfo {
        path: "assets/Main Ship/Main Ship - Bases/PNGs/Main Ship - Base - Damaged.png",
        total_frame: 1,
    },
    TextureInfo {
        path: "assets/Main Ship/Main Ship - Bases/PNGs/Main Ship - Base - Full health.png",
        total_frame: 1,
    },
    TextureInfo {
        path: "assets/Main Ship/Main Ship - Bases/PNGs/Main Ship - Base - Slight damage.png",
        total_frame: 1,
    },
    TextureInfo {
        path: "assets/Main Ship/Main Ship - Bases/PNGs/Main Ship - Base - Very damaged.png",
        total_frame: 1,
    },
];

pub const ENEMY_BASE_TEXTURES: [TextureInfo<'static>; 2] = [
    TextureInfo {
        path: "assets/Kla'ed/Weapons/PNGs/Kla'ed - Scout - Weapons.png",
        total_frame: 6,
    },
    TextureInfo {
        path: "assets/Kla'ed/Weapons/PNGs/Kla'ed - Frigate - Weapons.png",
        total_frame: 6,
    },
];
pub const ENEMY_PROJECTILE_TEXTURES: [TextureInfo<'static>; 2] = [
    TextureInfo {
        path: "assets/Kla'ed/Projectiles/PNGs/Kla'ed - Bullet.png",
        total_frame: 4,
    },
    TextureInfo {
        path: "assets/Kla'ed/Projectiles/PNGs/Kla'ed - Big Bullet.png",
        total_frame: 4,
    },
];

pub const ENEMY_ENGINE_TEXTURES: [TextureInfo<'static>; 2] = [
    TextureInfo {
        path: "assets/Kla'ed/Engine/PNGs/Kla'ed - Scout - Engine.png",
        total_frame: 10,
    },
    TextureInfo {
        path: "assets/Kla'ed/Engine/PNGs/Kla'ed - Frigate - Engine.png",
        total_frame: 12,
    },
];
