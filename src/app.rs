use std::{rc::Rc, time::Duration};

use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture},
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::WindowCanvas,
    Sdl,
};

use crate::{
    entity::{ComponentTexture, Entity, EntityEvent},
    player::Player,
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

    pub fn run<'a>(&mut self) -> Result<(), String> {
        let texture_creator = self.canvas.texture_creator();
        let engine_texture = {
            let engine_texture = Rc::new(texture_creator.load_texture(
            "assets/Main Ship/Main Ship - Engine Effects/PNGs/Main Ship - Engines - Base Engine - Powering.png"
        )?);
            ComponentTexture {
                texture: engine_texture,
                total_frame: 4,
                current_frame: 0,
            }
        };
        let projectile_texture = {
            let projectile_texture = Rc::new(texture_creator.load_texture(
                "assets/Main ship weapons/PNGs/Main ship weapon - Projectile - Auto cannon bullet.png"
        )?);
            ComponentTexture {
                texture: projectile_texture,
                total_frame: 4,
                current_frame: 0,
            }
        };
        let body_texture = {
            let body_texture = Rc::new(texture_creator.load_texture(
                "assets/Main Ship/Main Ship - Bases/PNGs/Main Ship - Base - Full health.png",
            )?);
            ComponentTexture {
                texture: body_texture,
                total_frame: 1,
                current_frame: 0,
            }
        };
        let game_viewport = Rect::new(0, 0, Self::WIDTH, Self::HEIGHT);
        let player = Player::new(
            game_viewport,
            engine_texture,
            body_texture,
            projectile_texture,
        );
        let mut entities = vec![];
        let player: Box<dyn Entity> = Box::new(player);
        entities.push(Some(player));
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
            self.clear(&mut entities);

            self.canvas.clear();
            self.render(&mut entities);
            self.canvas.present();

            std::thread::sleep(Duration::from_millis(16));
        }
        Ok(())
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
