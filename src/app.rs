use std::{collections::VecDeque, rc::Rc, time::Duration};

use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture},
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{Texture, TextureCreator, WindowCanvas},
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
        let bullet_texture = Rc::new(self.texture_creator.load_texture(
            "assets/Main ship weapons/PNGs/Main ship weapon - Projectile - Auto cannon bullet.png",
        )?);
        let texture = self.texture_creator.load_texture(
            "assets/Main Ship/Main Ship - Bases/PNGs/Main Ship - Base - Full health.png",
        )?;
        let texture = Rc::new(texture);
        let mut player = Entity::new(100, 100, texture);
        let mut bullets = vec![];
        'mainloop: loop {
            for event in self.sdl.event_pump()?.poll_iter() {
                match Self::handle_event(event, &mut player) {
                    LoopInstruction::Continue => {}
                    LoopInstruction::Break => break 'mainloop,
                }
            }
            player.update(0, Self::WIDTH as i32, 0, Self::HEIGHT as i32, 8);
            if player.firing && player.firing_ready {
                player.firing_ready = false;
                player.cd = 0;
                bullets.push(Self::create_bullet(
                    player.x,
                    player.y,
                    16,
                    4,
                    bullet_texture.clone(),
                ));
            }
            let mut remove_ind = vec![];
            for (index, bullet) in bullets.iter().enumerate() {
                if bullet.x > Self::WIDTH as i32 || bullet.health == 0 {
                    remove_ind.push(index);
                }
            }
            for i in 0..remove_ind.len() {
                bullets.remove(remove_ind[i] - i);
            }
            self.canvas.clear();

            for bullet in bullets.iter_mut() {
                bullet.x += bullet.dx;
                Self::draw_entity(&mut self.canvas, bullet);
            }
            Self::draw_entity(&mut self.canvas, &mut player);
            self.canvas.present();

            std::thread::sleep(Duration::from_millis(16));
        }

        Ok(())
    }

    fn create_bullet<'a>(
        x: i32,
        y: i32,
        dx: i32,
        total_frame: usize,
        texture: Rc<Texture<'a>>,
    ) -> Entity {
        let mut entity = Entity::new(x, y, texture);
        entity.dx = dx;
        entity.total_frame = total_frame;
        entity
    }

    fn draw_entity(canvas: &mut WindowCanvas, entity: &mut Entity) {
        let query = entity.texture.query();
        let total_width = query.width;
        let width = total_width / entity.total_frame as u32;
        let height = query.height;
        let src_rect = Rect::new(entity.current_frame as i32 * width as i32, 0, width, height);
        canvas
            .copy_ex(
                &entity.texture,
                src_rect,
                Some(Rect::new(entity.x, entity.y, width * 2, height * 2)),
                90.0,
                None,
                false,
                false,
            )
            .ok();
        entity.current_frame = (entity.current_frame + 1) % entity.total_frame;
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
            Keycode::LCtrl => {
                entity.firing = false;
                entity.firing_ready = true;
                entity.cd = 0;
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
            }
            Keycode::Down => {
                entity.down = true;
            }
            Keycode::Left => {
                entity.left = true;
            }
            Keycode::Right => {
                entity.right = true;
            }
            Keycode::LCtrl => {
                entity.firing = true;
            }
            Keycode::Escape => return LoopInstruction::Break,
            _ => {}
        }
        LoopInstruction::Continue
    }
}
