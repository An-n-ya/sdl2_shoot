use std::time::Duration;

use sdl2::{
    event::Event,
    image::InitFlag,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{TextureCreator, WindowCanvas},
    video::WindowContext,
    Sdl,
};

use crate::{
    enemy::Enemy,
    entity::{Entity, EntityEvent},
    player::Player,
    texture::{
        ComponentTexture, BASE_TEXTURES, ENGINE_EFFECTS_IDLE_TEXTURES,
        ENGINE_EFFECTS_POWERING_TEXTURES, ENGINE_TEXTURES, PROJECTILE_TEXTURES, WEAPON_TEXTURES,
    },
};

pub struct App {
    sdl: Sdl,
    canvas: WindowCanvas,
}

type EntityType<'a> = Box<dyn Entity<'a> + 'a>;

impl App {
    const WIDTH: u32 = 1280;
    const HEIGHT: u32 = 720;
    pub fn new(name: &str) -> Result<Self, String> {
        let sdl = sdl2::init().unwrap();
        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
        let window = sdl
            .video()?
            .window(name, Self::WIDTH, Self::HEIGHT)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
        let mut canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;
        canvas.set_draw_color(Color::RGBA(96, 128, 255, 255));
        canvas.clear();
        Ok(Self { sdl, canvas })
    }

    fn make_player(texture_creator: &TextureCreator<WindowContext>) -> Player {
        let weapon_texture = ComponentTexture::new(&texture_creator, &WEAPON_TEXTURES[0]);
        let engine_base_texture = ComponentTexture::new(&texture_creator, &ENGINE_TEXTURES[0]);
        let idle_texture =
            ComponentTexture::new(&texture_creator, &ENGINE_EFFECTS_IDLE_TEXTURES[0]);
        let engine_texture =
            ComponentTexture::new(&texture_creator, &ENGINE_EFFECTS_POWERING_TEXTURES[0]);
        let projectile_texture = ComponentTexture::new(&texture_creator, &PROJECTILE_TEXTURES[0]);
        let body_texture = ComponentTexture::new(&texture_creator, &BASE_TEXTURES[1]);
        let game_viewport = Rect::new(0, 0, Self::WIDTH, Self::HEIGHT);
        let player = Player::new(
            game_viewport,
            engine_texture,
            engine_base_texture,
            idle_texture,
            weapon_texture,
            body_texture,
            projectile_texture,
        );
        player
    }

    pub fn run<'a>(&mut self) -> Result<(), String> {
        let texture_creator = self.canvas.texture_creator();
        let player = Self::make_player(&texture_creator);
        let mut entities = vec![];
        let player: Box<dyn Entity> = Box::new(player);
        entities.push(Some(player));

        // make frame rate more accurate
        let mut ticks = unsafe { sdl2_sys::SDL_GetTicks64() };
        let mut remainder = 0.0;

        let mut enemy_spawn_time = rand::random::<u32>() % 60;

        'mainloop: loop {
            for event in self.sdl.event_pump()?.poll_iter() {
                match event {
                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    }
                    | Event::Quit { .. } => break 'mainloop,
                    _ => self.handle_event(&mut entities, event),
                }
            }
            let entity_events = self.update(&mut entities);
            for events in entity_events {
                match events {
                    EntityEvent::SpawnBullet(bullet) => entities.push(Some(Box::new(bullet))),
                    EntityEvent::Empty => {}
                }
            }

            self.handle_collision(&mut entities);

            self.clear(&mut entities);

            self.canvas.clear();
            self.render(&mut entities);
            self.canvas.present();

            Self::spawn_enemy(&mut enemy_spawn_time, &mut entities, &texture_creator);

            Self::cap_frame_rate(&mut ticks, &mut remainder);
        }
        Ok(())
    }

    fn spawn_enemy<'a>(
        enemy_spawn_time: &mut u32,
        entities: &mut Vec<Option<Box<dyn Entity<'a> + 'a>>>,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) {
        *enemy_spawn_time -= 1;
        if *enemy_spawn_time <= 0 {
            let game_viewport = Rect::new(0, 0, Self::WIDTH, Self::HEIGHT);
            let enemy = Enemy::new(texture_creator, game_viewport);
            let enemy: Box<dyn Entity> = Box::new(enemy);
            entities.push(Some(enemy));

            *enemy_spawn_time = 30 + (rand::random::<u32>() % 60);
        }
    }

    fn cap_frame_rate(tick: &mut u64, remainder: &mut f64) {
        let mut wait = 16.0 + *remainder;
        *remainder = remainder.fract();
        let frame_time = unsafe { sdl2_sys::SDL_GetTicks64() } - *tick;
        wait -= (1f64).max(frame_time as f64);
        std::thread::sleep(Duration::from_millis(wait as u64));
        *remainder += 0.667;
        *tick = unsafe { sdl2_sys::SDL_GetTicks64() };
    }

    fn update<'a>(&mut self, entities: &mut Vec<Option<EntityType<'a>>>) -> Vec<EntityEvent<'a>> {
        let mut events = vec![];
        for entity in entities.iter_mut() {
            if let Some(entity_inner) = entity {
                events.push(entity_inner.update());
            }
        }
        events
    }
    fn handle_collision<'a>(&mut self, entities: &mut Vec<Option<EntityType<'a>>>) {
        // dummy version
        let size = entities.len();
        for i in 0..size {
            for j in (i + 1)..size {
                let (slice1, slice2) = entities.split_at_mut(j);
                if let Some(ref mut e1) = slice1[i] {
                    if let Some(ref mut e2) = slice2[0] {
                        if !e1.valid() || !e2.valid() {
                            continue;
                        }
                        if !e1.is_bullet() && e1.is_enemy() && e2.is_bullet() && e2.is_player()
                            || e1.is_bullet() && e1.is_player() && !e2.is_bullet() && e2.is_enemy()
                        {
                            if let Some(e1_base) = e1.base_mut() {
                                if let Some(e2_base) = e2.base_mut() {
                                    let (e1_start_x, e2_start_x) = (e1_base.x, e2_base.x);
                                    let (e1_end_x, e2_end_x) =
                                        (e1_base.width + e1_start_x, e2_base.width + e2_start_x);
                                    let (e1_start_y, e2_start_y) = (e1_base.y, e2_base.y);
                                    let (e1_end_y, e2_end_y) =
                                        (e1_base.height + e1_start_y, e2_base.height + e2_start_y);
                                    if e1_start_x.max(e2_start_x) < e1_end_x.min(e2_end_x)
                                        && e1_start_y.max(e2_start_y) < e1_end_y.min(e2_end_y)
                                    {
                                        // collision happened
                                        e1_base.valid = false;
                                        e2_base.valid = false;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn clear<'a>(&mut self, entities: &mut Vec<Option<EntityType<'a>>>) {
        let mut new_entities = vec![];
        let mut ind = vec![];
        for (index, entity) in entities.iter().enumerate() {
            if let Some(entity_inner) = entity {
                if entity_inner.valid() {
                    ind.push(index);
                }
            }
        }
        for i in ind {
            new_entities.push(entities[i].take());
        }
        entities.clear();
        for entity in new_entities {
            entities.push(entity);
        }
    }

    fn render<'a>(&mut self, entities: &mut Vec<Option<Box<dyn Entity<'a> + 'a>>>) {
        for entity in entities.iter_mut() {
            if let Some(entity_inner) = entity {
                entity_inner.render(&mut self.canvas);
            }
        }
    }

    fn handle_event<'a>(&mut self, entities: &mut Vec<Option<EntityType<'a>>>, event: Event) {
        for entity in entities.iter_mut() {
            if let Some(entity_inner) = entity {
                entity_inner.handle_event(event.clone());
            }
        }
    }
}
