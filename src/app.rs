use std::{rc::Rc, time::Duration};

use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture},
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator, WindowCanvas},
    sys::Window,
    video::WindowContext,
    Sdl,
};

use crate::entity::Entity;

pub struct App {
    sdl: Sdl,
    name: String,
    canvas: WindowCanvas,
    texture_creator: TextureCreator<WindowContext>,
}

enum LoopInstruction {
    Continue,
    Break,
}

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
        let texture_creator = canvas.texture_creator();

        Ok(Self {
            sdl,
            name: name.to_string(),
            canvas,
            texture_creator,
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        let texture = self.texture_creator.load_texture(
            "assets/Main Ship/Main Ship - Bases/PNGs/Main Ship - Base - Full health.png",
        )?;
        let texture = Rc::new(texture);
        let mut player = Entity::new(100, 100, texture);
        'mainloop: loop {
            for event in self.sdl.event_pump()?.poll_iter() {
                match Self::handle_event(event, &mut player) {
                    LoopInstruction::Continue => {}
                    LoopInstruction::Break => break 'mainloop,
                }
            }
            player.update_pos(0, Self::WIDTH as i32, 0, Self::HEIGHT as i32, 8);
            self.canvas.clear();
            Self::draw_entity(&mut self.canvas, &player);
            self.canvas.present();

            std::thread::sleep(Duration::from_millis(16));
        }

        Ok(())
    }

    fn draw_entity(canvas: &mut WindowCanvas, entity: &Entity) {
        let query = entity.texture.query();
        let width = query.width;
        let height = query.height;
        canvas
            .copy_ex(
                &entity.texture,
                None,
                Some(Rect::new(entity.x, entity.y, width * 2, height * 2)),
                90.0,
                None,
                false,
                false,
            )
            .ok();
    }

    fn handle_event(event: Event, entity: &mut Entity) -> LoopInstruction {
        match event {
            Event::Quit { .. } => LoopInstruction::Break,
            Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => Self::handle_key_down(keycode, entity),
            Event::KeyUp {
                keycode: Some(keycode),
                ..
            } => Self::handle_key_up(keycode, entity),
            _ => LoopInstruction::Continue,
        }
    }
    fn handle_key_up(keycode: Keycode, entity: &mut Entity) -> LoopInstruction {
        match keycode {
            Keycode::Up => {
                entity.up = false;
            }
            Keycode::Down => {
                entity.down = false;
            }
            Keycode::Left => {
                entity.left = false;
            }
            Keycode::Right => {
                entity.right = false;
            }
            Keycode::Escape => return LoopInstruction::Break,
            _ => {}
        }
        LoopInstruction::Continue
    }

    fn handle_key_down(keycode: Keycode, entity: &mut Entity) -> LoopInstruction {
        match keycode {
            Keycode::Up => {
                entity.up = true;
                if entity.y >= 8 {
                    entity.y -= 8;
                }
            }
            Keycode::Down => {
                entity.down = true;
                if entity.y + 8 < Self::HEIGHT as i32 {
                    entity.y += 8
                }
            }
            Keycode::Left => {
                entity.left = true;
                if entity.x >= 8 {
                    entity.x -= 8
                }
            }
            Keycode::Right => {
                entity.right = true;
                if entity.x + 8 < Self::WIDTH as i32 {
                    entity.x += 8
                }
            }
            Keycode::Escape => return LoopInstruction::Break,
            _ => {}
        }
        LoopInstruction::Continue
    }
}
